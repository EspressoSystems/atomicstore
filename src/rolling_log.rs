use crate::atomic_store::AtomicStoreLoader;
use crate::error::{
    PersistenceError, StdIoDirOpsError, StdIoOpenError, StdIoReadError, StdIoSeekError,
    StdIoWriteError,
};
use crate::load_store::LoadStore;
use crate::storage_location::StorageLocation;
use crate::version_sync::VersionSyncHandle;
use crate::Result;

use chrono::Utc;
use snafu::ResultExt;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct RollingLog<ResourceAdaptor: LoadStore> {
    persisted_sync: Arc<RwLock<VersionSyncHandle>>,
    file_path: PathBuf,
    file_pattern: String,
    file_fill_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    file_entries: u32,
    write_file_counter: u32,
    adaptor: ResourceAdaptor,
}

fn format_nth_file_path(root_path: &Path, file_pattern: &str, file_count: u32) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_{}", file_pattern, file_count));
    buf
}

fn load_from_file<ResourceAdaptor: LoadStore>(
    read_file: &mut File,
    adaptor: &ResourceAdaptor,
    location: &StorageLocation,
) -> Result<ResourceAdaptor::ParamType> {
    read_file
        .seek(SeekFrom::Start(location.store_start))
        .context(StdIoSeekError)?;
    let mut reader = read_file.take(location.store_length as u64);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).context(StdIoReadError)?;
    adaptor.load(&buffer[..])
}

impl<ResourceAdaptor: LoadStore> RollingLog<ResourceAdaptor> {
    pub(crate) fn open_impl(
        adaptor: ResourceAdaptor,
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let (write_pos, counter) = match location {
            Some(ref location) => {
                let append_point = location.store_start + location.store_length as u64;
                if append_point < file_fill_size {
                    (append_point, location.file_counter)
                } else {
                    (4, location.file_counter + 1)
                }
            }
            None => (4, 0), // every rolling file starts with a counter
        };
        Ok(RollingLog {
            persisted_sync: Arc::new(RwLock::new(VersionSyncHandle::new(file_pattern, location))),
            file_path: file_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_fill_size,
            write_to_file: None,
            write_pos,
            file_entries: 0,
            write_file_counter: counter,
            adaptor,
        })
    }

    pub fn load(
        loader: &mut AtomicStoreLoader,
        adaptor: ResourceAdaptor,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let resource = loader.look_up_resource(file_pattern);
        let path = loader.persistence_path().to_path_buf();
        let created = Self::open_impl(adaptor, resource, &path, file_pattern, file_fill_size)?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }
    pub fn create(
        loader: &mut AtomicStoreLoader,
        adaptor: ResourceAdaptor,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let path = loader.persistence_path().to_path_buf();
        let created = Self::open_impl(adaptor, None, &path, file_pattern, file_fill_size)?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }

    fn open_write_file(&mut self) -> Result<()> {
        let out_file_path =
            format_nth_file_path(&self.file_path, &self.file_pattern, self.write_file_counter);

        if out_file_path.exists() {
            if !out_file_path.is_file() {
                return Err(PersistenceError::InvalidPathToFile {
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }

            if let Ok(metadata) = fs::metadata(&out_file_path) {
                if metadata.len() > self.write_pos {
                    let mut backup_path = self.file_path.clone();
                    backup_path.push(format!(
                        "{}_{}.bak.{}",
                        self.file_pattern,
                        self.write_file_counter,
                        Utc::now().timestamp()
                    ));
                    if self.write_pos > 0 {
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
            .open(out_file_path.clone())
            .context(StdIoOpenError)?;
        self.file_entries = 0;
        if file.stream_position().context(StdIoSeekError)? == 0 {
            file.write_all(&[0u8; 4]).context(StdIoWriteError)?;
        }
        if file.stream_position().context(StdIoSeekError)? < self.write_pos {
            return Err(PersistenceError::InvalidFileContents {
                note: "file too small for last recorded entry".to_string(),
                path: out_file_path.to_string_lossy().to_string(),
            });
        }
        if file.stream_position().context(StdIoSeekError)? > 4 {
            self.file_entries = 0;
            let _lines = file.seek(SeekFrom::Start(0)).context(StdIoSeekError)?;
            let mut buffer = [0u8; 4];
            file.read_exact(&mut buffer).context(StdIoReadError)?;
            let remembered_entries = u32::from_le_bytes(buffer);
            let mut read_position = 4u64;
            while read_position < self.write_pos {
                let mut buffer = [0u8; 4];
                file.read_exact(&mut buffer).context(StdIoReadError)?;
                let entry_size = u32::from_le_bytes(buffer);
                read_position += entry_size as u64;
                let _lines = file
                    .seek(SeekFrom::Start(read_position))
                    .context(StdIoSeekError)?;
                self.file_entries += 1;
            }
            if read_position > self.write_pos {
                return Err(PersistenceError::InvalidFileContents {
                    note: "file stream mismatch for last recorded entry".to_string(),
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }
            if self.file_entries != remembered_entries {
                file.seek(SeekFrom::Start(0)).context(StdIoSeekError)?;
                file.write_all(&self.file_entries.to_le_bytes())
                    .context(StdIoWriteError)?;
            }
            file.seek(SeekFrom::End(0)).context(StdIoSeekError)?;
        }
        if file.stream_position().context(StdIoSeekError)? > self.write_pos {
            file.set_len(self.write_pos).context(StdIoWriteError)?;
            let _lines = file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekError)?;
        }
        self.write_to_file = Some(file);
        Ok(())
    }

    // Writes out a resource instance; does not update the commit position, but in this version,
    // does advance the pending commit position.
    pub fn store_resource(
        &mut self,
        resource: &ResourceAdaptor::ParamType,
    ) -> Result<StorageLocation> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = self.adaptor.store(resource)?;
        let resource_length = serialized.len() as u32;
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&resource_length.to_le_bytes())
            .context(StdIoWriteError)?;
        self.write_pos += 4;
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&serialized)
            .context(StdIoWriteError)?;

        let location = StorageLocation {
            file_counter: self.write_file_counter,
            store_start: self.write_pos,
            store_length: resource_length,
        };

        self.write_pos += resource_length as u64;
        self.file_entries += 1;
        if self.write_pos >= self.file_fill_size {
            if let Some(mut write_to_file) = self.write_to_file.as_ref() {
                let _lines = write_to_file
                    .seek(SeekFrom::Start(0))
                    .context(StdIoSeekError)?;
                write_to_file
                    .write_all(&self.file_entries.to_le_bytes())
                    .context(StdIoWriteError)?;
            }
            self.write_pos = 0;
            self.file_entries = 0;
            self.write_file_counter += 1;
            self.write_to_file = None;
        }
        self.persisted_sync.write()?.advance_next(Some(location));
        Ok(location)
    }

    // This currenty won't have any effect if called again before the atomic store has processed the prior committed version. A more appropriate behavior might be to block. A version that supports queued writes could enqueue the commit points.
    pub fn commit_version(&mut self) -> Result<()> {
        if let Some(mut write_to_file) = self.write_to_file.as_ref() {
            let _lines = write_to_file
                .seek(SeekFrom::Start(0))
                .context(StdIoSeekError)?;
            write_to_file
                .write_all(&self.file_entries.to_le_bytes())
                .context(StdIoWriteError)?;
            let _lines = write_to_file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekError)?;
        }
        self.persisted_sync.write()?.update_version()
    }

    pub fn skip_version(&mut self) -> Result<()> {
        self.persisted_sync.write()?.skip_version()
    }

    pub fn revert_version(&mut self) -> Result<()> {
        self.write_to_file = None;
        self.persisted_sync.write()?.revert_version()
    }

    // Opens a new handle to the file. Possibly need to revisit in the future?
    pub fn load_latest(&self) -> Result<ResourceAdaptor::ParamType> {
        if let Some(location) = self.persisted_sync.read()?.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {
                key: self.file_pattern.to_string(),
            })
        }
    }
    pub fn load_specified(&self, location: &StorageLocation) -> Result<ResourceAdaptor::ParamType> {
        let read_file_path =
            format_nth_file_path(&self.file_path, &self.file_pattern, location.file_counter);
        let mut read_file = File::open(read_file_path.as_path()).context(StdIoOpenError)?;
        load_from_file::<ResourceAdaptor>(&mut read_file, &self.adaptor, location)
    }
}
