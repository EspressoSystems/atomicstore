use crate::atomic_store::{AtomicStoreLoader, PersistentStore};
use crate::error::{
    BincodeDeError, BincodeSerError, PersistenceError, StdIoDirOpsError, StdIoOpenError,
    StdIoReadError, StdIoSeekError, StdIoWriteError,
};
use crate::storage_location::StorageLocation;
use crate::version_sync::PersistedLocationHandler;

use chrono::Utc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use snafu::ResultExt;

use std::convert::TryInto;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

// future: declare with #[repr(C)] and directly map?
#[derive(Copy, Clone)]
struct IndexContents {
    byte_order: u32,
    chunk_size: u32,
    file_size: u32,
    commit_index: u32,
}

const BYTE_ORDER: u32 = 0x8001FEFFu32;
const BYTE_DISORDER: u32 = 0xFFFE0180u32;

fn load_existing_index(index_file_path: &Path) -> Result<IndexContents, PersistenceError> {
    if !index_file_path.is_file() {
        return Err(PersistenceError::InvalidPathToFile {
            path: index_file_path.to_string_lossy().to_string(),
        });
    }
    let metadata = fs::metadata(index_file_path).context(StdIoOpenError)?;
    if metadata.len() < 16 {
        // file doesn't contain a minimal IndexContents
        return Err(PersistenceError::InvalidFileContents {
            path: index_file_path.to_string_lossy().to_string(),
        });
    }
    let mut index_file = File::open(index_file_path).context(StdIoOpenError)?;
    let mut buffer = [0, 16];
    let read_length = index_file.read(&mut buffer).context(StdIoReadError)?;
    if read_length < 16 {
        return Err(PersistenceError::InvalidFileContents {
            path: index_file_path.to_string_lossy().to_string(),
        });
    }

    let contents = IndexContents {
        byte_order: u32::from_ne_bytes(buffer[0..4].try_into().unwrap()),
        chunk_size: u32::from_ne_bytes(buffer[4..8].try_into().unwrap()),
        file_size: u32::from_ne_bytes(buffer[8..12].try_into().unwrap()),
        commit_index: u32::from_ne_bytes(buffer[12..16].try_into().unwrap()),
    };
    
    if contents.byte_order == BYTE_DISORDER {
        return Err(PersistenceError::FeatureNotYetImplemented{description: "byte order reordering".to_string()}); 
    } else if contents.byte_order != BYTE_ORDER {
        return Err(PersistenceError::InvalidFileContents{path: index_file_path.to_string_lossy().to_string()});
    }

    Ok(contents)
}

fn format_index_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_index", file_pattern));
    buf
}

fn format_range_file_path(root_path: &Path, file_pattern: &str, from_index: u64, up_to_index: u64) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_{}_{}", file_pattern, from_index, up_to_index));
    buf
}

fn format_backup_index_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_index_working", file_pattern));
    buf
}

fn format_working_index_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_index_backup", file_pattern));
    buf
}

fn compute_location(from_index: &IndexContents) -> StorageLocation {
    StorageLocation {
        store_start: (from_index.commit_index % from_index.file_size) as u64 * from_index.chunk_size as u64,
        store_length: from_index.chunk_size,
        file_counter: (from_index.commit_index / from_index.file_size),
    }
}

/// For now, this is implemented with a direct copy from memory to file, using native order, but the order is recorded in a header, and can be used to support transfer in the future.
#[derive(Debug)]
pub struct FixedAppendLog<StoredResource: 'static> {
    persisted_sync: PersistedLocationHandler,
    file_path: PathBuf,
    file_pattern: String,
    resource_size: u64, // must match StoredResource serialized size.
    file_size: u64, // number of StoredResource serializations per file; must not change, will check on load.
    write_to_file: Option<File>,
    commit_index: u64,
    write_index: u64, // other indexes can be derived.
    phantom: PhantomData<&'static StoredResource>,
}

impl<StoredResource: Copy> FixedAppendLog<StoredResource> {
    pub(crate) fn open_impl(
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        resource_size: u64,
        file_size: u64,
    ) -> Result<FixedAppendLog<StoredResource>, PersistenceError> {
        let index_file_path = format_index_file_path(file_path, file_pattern);
        let backup_file_path = format_backup_index_file_path(file_path, file_pattern);
        let working_file_path = format_working_index_file_path(file_path, file_pattern);
        let commit_index;
        let write_index;
        if let Some(location) = location {
            // expect the files to exist; if files do not exist, make an attempt to recover the backed up index. Do not attempt to open an abandoned working index file.
            let index_contents = if index_file_path.exists() {
                load_existing_index(&index_file_path)
            } else if backup_file_path.exists() {
                load_existing_index(&backup_file_path)
            } else {
                Err(PersistenceError::FailedToResolvePath{path: index_file_path.as_path().to_string_lossy().to_string()})
            }?;
            if index_contents.file_size as u64 != file_size ||
               index_contents.chunk_size as u64 != resource_size {
                   return Err(PersistenceError::ResourceFormatInconsistent{key: file_pattern.to_string()});
            }
            let indexed_location = compute_location(&index_contents);
            commit_index = index_contents.commit_index as u64;
            write_index = commit_index + 1;
        } else {
            commit_index = 0u64;
            write_index = 0u64;
        }
        Ok(FixedAppendLog {
            persisted_sync: PersistedLocationHandler::new(location),
            file_path: file_path.to_path_buf(),
            file_pattern: file_pattern.to_string(),
            resource_size,
            file_size,
            write_to_file: None,
            commit_index,
            write_index,
            phantom: PhantomData,
        })
    }
}

