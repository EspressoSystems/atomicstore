use crate::atomic_store::AtomicStoreLoader;
use crate::error::{
    BincodeDeError, BincodeSerError, PersistenceError, StdIoDirOpsError, StdIoOpenError,
    StdIoReadError, StdIoSeekError, StdIoWriteError,
};
use crate::load_store::LoadStore;
use crate::storage_location::StorageLocation;
use crate::version_sync::VersionSyncHandle;
use crate::Result;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

// future: declare with #[repr(C)] and directly map?
#[derive(Serialize, Deserialize, Copy, Clone)]
struct IndexContents {
    byte_order: u32,
    chunk_size: u32,
    file_size: u32,
    commit_index: u32,
}

const BYTE_ORDER: u32 = 0x8001FEFFu32;
const BYTE_DISORDER: u32 = 0xFFFE0180u32;

fn load_existing_index(index_file_path: &Path) -> Result<IndexContents> {
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
    let mut buffer = Vec::new();
    index_file
        .read_to_end(&mut buffer)
        .context(StdIoReadError)?;
    let contents: IndexContents = bincode::deserialize(&buffer[..]).context(BincodeDeError)?;
    if contents.byte_order == BYTE_DISORDER {
        return Err(PersistenceError::FeatureNotYetImplemented {
            description: "byte order reordering".to_string(),
        });
    } else if contents.byte_order != BYTE_ORDER {
        return Err(PersistenceError::InvalidFileContents {
            path: index_file_path.to_string_lossy().to_string(),
        });
    }

    Ok(contents)
}

fn format_index_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_index", file_pattern));
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

fn format_range_file_path(
    root_path: &Path,
    file_pattern: &str,
    from_index: u64,
    up_to_index: u64,
) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_{}_{}", file_pattern, from_index, up_to_index));
    buf
}

fn compute_location(from_index: &IndexContents) -> StorageLocation {
    StorageLocation {
        store_start: (from_index.commit_index % from_index.file_size) as u64
            * from_index.chunk_size as u64,
        store_length: from_index.chunk_size,
        file_counter: (from_index.commit_index / from_index.file_size),
    }
}

/// For now, this is implemented with a direct copy from memory to file, using native order, but the order is recorded in a header, and can be used to support transfer in the future.
#[derive(Debug)]
pub struct FixedAppendLog<ResourceType: LoadStore> {
    persisted_sync: Arc<RwLock<VersionSyncHandle>>,
    file_path: PathBuf,
    file_pattern: String,
    resource_size: u64, // must match ResourceType::ParamType serialized size.
    file_size: u64, // number of ResourceType::ParamType serializations per file; must not change, will check on load.
    write_to_file: Option<File>,
    commit_index: u64,
    write_index: u64, // other indexes can be derived.
    phantom: PhantomData<ResourceType>,
}

pub struct Iter<ResourceType: LoadStore> {
    file_path: PathBuf,
    file_pattern: String,
    resource_size: u64,
    file_size: u64,
    read_from_file: Option<File>,
    from_index: u64,
    end_index: u64,
    phantom: PhantomData<ResourceType>,
}

impl<ResourceType: LoadStore + Default> FixedAppendLog<ResourceType> {
    pub(crate) fn open_impl(
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        resource_size: u64,
        file_size: u64,
    ) -> Result<FixedAppendLog<ResourceType>> {
        let index_file_path = format_index_file_path(file_path, file_pattern);
        let backup_file_path = format_backup_index_file_path(file_path, file_pattern);
        let commit_index;
        let write_index;
        if let Some(location) = location {
            // expect the files to exist; if files do not exist, make an attempt to recover the backed up index. Do not attempt to open an abandoned working index file.
            let index_contents = if index_file_path.exists() {
                load_existing_index(&index_file_path)
            } else if backup_file_path.exists() {
                load_existing_index(&backup_file_path)
            } else {
                Err(PersistenceError::FailedToResolvePath {
                    path: index_file_path.as_path().to_string_lossy().to_string(),
                })
            }?;
            if index_contents.file_size as u64 != file_size
                || index_contents.chunk_size as u64 != resource_size
            {
                return Err(PersistenceError::ResourceFormatInconsistent {
                    key: file_pattern.to_string(),
                });
            }
            let indexed_location = compute_location(&index_contents);
            if indexed_location != location {}
            commit_index = index_contents.commit_index as u64;
            write_index = commit_index + 1;
        } else {
            commit_index = 0u64;
            write_index = 0u64;
        }
        Ok(FixedAppendLog {
            persisted_sync: Arc::new(RwLock::new(VersionSyncHandle::new(file_pattern, location))),
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
    pub fn load(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        resource_size: u64,
        file_size: u64,
    ) -> Result<FixedAppendLog<ResourceType>> {
        let created = Self::open_impl(
            loader.look_up_resource(file_pattern),
            loader.persistence_path(),
            file_pattern,
            resource_size,
            file_size,
        )?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }
    pub fn create(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        resource_size: u64,
        file_size: u64,
    ) -> Result<FixedAppendLog<ResourceType>> {
        let created = Self::open_impl(
            None,
            loader.persistence_path(),
            file_pattern,
            resource_size,
            file_size,
        )?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }

    fn location_to_index(&self, location: &StorageLocation) -> Result<u64> {
        if location.store_length as u64 != self.resource_size
            || location.store_start % self.resource_size != 0
        {
            Err(PersistenceError::ResourceFormatInconsistent {
                key: self.file_pattern.clone(),
            })
        } else {
            Ok((location.file_counter as u64 * self.file_size)
                + (location.store_start / self.resource_size))
        }
    }

    fn index_to_location(&self, index: u64) -> StorageLocation {
        StorageLocation {
            store_start: (index % self.file_size) * self.resource_size,
            store_length: self.resource_size as u32,
            file_counter: (index / self.file_size) as u32,
        }
    }

    fn open_write_file(&mut self) -> Result<()> {
        let file_index = self.write_index % self.file_size;
        let write_pos = file_index * self.resource_size;
        let range_begin = self.write_index - file_index;
        let range_end = range_begin + self.file_size;
        let out_file_path =
            format_range_file_path(&self.file_path, &self.file_pattern, range_begin, range_end);
        if out_file_path.exists() {
            if !out_file_path.is_file() {
                return Err(PersistenceError::InvalidPathToFile {
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }

            if let Ok(metadata) = fs::metadata(&out_file_path) {
                if metadata.len() > write_pos {
                    let mut backup_path = self.file_path.clone();
                    backup_path.push(format!(
                        "{}_{}_{}.bak.{}",
                        self.file_pattern,
                        range_begin,
                        range_end,
                        Utc::now().timestamp()
                    ));
                    if file_index > 0 {
                        fs::copy(&out_file_path, &backup_path).context(StdIoDirOpsError)?;
                    } else {
                        fs::rename(&out_file_path, &backup_path).context(StdIoDirOpsError)?;
                    }
                }
            }
        }
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(out_file_path)
            .context(StdIoOpenError)?;
        if file.stream_position().context(StdIoSeekError)? != write_pos {
            file.set_len(write_pos).context(StdIoWriteError)?;
            let _lines = file
                .seek(SeekFrom::Start(write_pos))
                .context(StdIoSeekError)?;
        }
        self.write_to_file = Some(file);
        Ok(())
    }

    // Writes out a resource instance; does not update the commit position, but in this version, does advance the pending commit position.
    // In the future, we may support a queue of commit points, or even entire sequences of pre-written alternative future versions (for chained consensus), which would require a more complex interface.
    pub fn store_resource(
        &mut self,
        resource: &ResourceType::ParamType,
    ) -> Result<StorageLocation> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = ResourceType::store(resource)?;
        debug_assert_eq!(serialized.len() as u64, self.resource_size);
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&serialized)
            .context(StdIoWriteError)?;

        let location = self.index_to_location(self.write_index);

        self.write_index += 1;
        if self.write_index % self.file_size == 0 {
            self.write_to_file = None;
        }
        self.persisted_sync.write()?.advance_next(Some(location));
        Ok(location)
    }

    // This currenty won't have any effect if called again before the atomic store has processed the prior committed version. A more appropriate behavior might be to block. A version that supports queued writes could enqueue the commit points.
    pub fn commit_version(&mut self) -> Result<()> {
        let index_file_path = format_index_file_path(&self.file_path, &self.file_pattern);
        let backup_file_path = format_backup_index_file_path(&self.file_path, &self.file_pattern);
        let working_file_path = format_working_index_file_path(&self.file_path, &self.file_pattern);

        self.commit_index = self.write_index - 1; // revisit if adding support for longer pending chains

        let contents = IndexContents {
            byte_order: BYTE_ORDER,
            chunk_size: self.resource_size as u32,
            file_size: self.file_size as u32,
            commit_index: self.commit_index as u32,
        };

        let serialized = bincode::serialize(&contents).context(BincodeSerError)?;

        let mut write_index_file = File::create(&working_file_path).context(StdIoOpenError)?;
        write_index_file
            .write_all(&serialized)
            .context(StdIoWriteError)?;
        if index_file_path.exists() {
            if backup_file_path.exists() {
                fs::remove_file(&backup_file_path).context(StdIoDirOpsError)?;
            }
            fs::rename(&index_file_path, &backup_file_path).context(StdIoDirOpsError)?;
        }
        fs::rename(&working_file_path, &index_file_path).context(StdIoDirOpsError)?;

        self.persisted_sync.write()?.update_version()
    }

    pub fn skip_version(&mut self) -> Result<()> {
        self.persisted_sync.write()?.skip_version()
    }

    // these function as an alternative to the LogLoader, based on external (StorageLocation) addressing.
    pub fn load_latest(&self) -> Result<ResourceType::ParamType> {
        if let Some(location) = self.persisted_sync.read()?.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {
                key: self.file_pattern.to_string(),
            })
        }
    }

    pub fn load_specified(&self, location: &StorageLocation) -> Result<ResourceType::ParamType> {
        let index = self.location_to_index(location)?;
        self.load_at(index)
    }

    // this works like the LogLoader, but doesn't keep resources after the call completes.
    pub fn load_at(&self, index: u64) -> Result<ResourceType::ParamType> {
        let file_index = index % self.file_size;
        let file_offset = index * self.resource_size;
        let range_begin = index - file_index;
        let range_end = range_begin + self.file_size;
        let read_file_path =
            format_range_file_path(&self.file_path, &self.file_pattern, range_begin, range_end);

        let mut read_file = File::open(read_file_path).context(StdIoOpenError)?;
        read_file
            .seek(SeekFrom::Start(file_offset))
            .context(StdIoSeekError)?;
        let mut reader = read_file.take(self.resource_size);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).context(StdIoReadError)?;
        ResourceType::load(&buffer[..])
    }

    pub fn iter(&self) -> Iter<ResourceType> {
        Iter {
            file_path: self.file_path.clone(),
            file_pattern: self.file_pattern.clone(),
            resource_size: self.resource_size,
            file_size: self.file_size,
            read_from_file: None,
            from_index: 0,
            end_index: self.commit_index + 1,
            phantom: PhantomData,
        }
    }
}

impl<ResourceType: LoadStore + Default> Iter<ResourceType> {
    fn helper(&mut self) -> Result<ResourceType::ParamType> {
        let file_offset = self.from_index % self.file_size;
        let range_begin = self.from_index - file_offset;
        let range_end = range_begin + self.file_size;
        if self.read_from_file.is_none() {
            let file_name =
                format_range_file_path(&self.file_path, &self.file_pattern, range_begin, range_end);
            self.read_from_file = Some(File::open(file_name).context(StdIoOpenError)?);
            if file_offset > 0 {
                self.read_from_file
                    .as_ref()
                    .unwrap()
                    .seek(SeekFrom::Start(file_offset * self.resource_size))
                    .context(StdIoSeekError)?;
            }
        }
        let mut reader = self
            .read_from_file
            .as_ref()
            .unwrap()
            .take(self.resource_size);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).context(StdIoReadError)?;

        ResourceType::load(&buffer[..])
    }
}

impl<ResourceType: LoadStore> Iterator for Iter<ResourceType> {
    type Item = Result<ResourceType::ParamType>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from_index >= self.end_index {
            return None;
        }
        let resource = self.helper();
        self.from_index += 1;
        if self.from_index % self.file_size == 0 {
            self.read_from_file = None;
        }
        Some(resource)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end_index - self.from_index) as usize;
        (remaining, Some(remaining))
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if self.from_index + n as u64 >= self.end_index {
            self.from_index = self.end_index;
            return None;
        }
        if self.from_index / self.file_size != self.from_index + n as u64 / self.file_size {
            self.read_from_file = None;
        }
        self.from_index += n as u64;
        self.next()
    }
}

impl<ResourceType: LoadStore> ExactSizeIterator for Iter<ResourceType> {
    fn len(&self) -> usize {
        (self.end_index - self.from_index) as usize
    }
}
