use crate::atomic_store::{AtomicStoreLoader, PersistentStore};
use crate::error::{
    PersistenceError, StdIoDirOpsError, StdIoOpenError,
    StdIoReadError, StdIoSeekError, StdIoWriteError,
};
use crate::load_store::LoadStore;
use crate::storage_location::StorageLocation;
use crate::version_sync::PersistedLocationHandler;
use crate::Result;

use chrono::Utc;
use snafu::ResultExt;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct RollingLog<TypeStore: LoadStore> {
    persisted_sync: PersistedLocationHandler,
    file_path: PathBuf,
    file_pattern: String,
    file_fill_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    write_file_counter: u32,
    _type_store: TypeStore,
}

impl<TypeStore: 'static + LoadStore + Default> RollingLog<TypeStore> {
    pub(crate) fn open_impl(
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<TypeStore>> {
        let (write_pos, counter) = match location {
            Some(ref location) => {
                let append_point = location.store_start + location.store_length as u64;
                if append_point < file_fill_size {
                    (append_point, location.file_counter)
                } else {
                    (0, location.file_counter + 1)
                }
            }
            None => (0, 0),
        };
        Ok(RollingLog {
            persisted_sync: PersistedLocationHandler::new(location),
            file_path: file_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_fill_size,
            write_to_file: None,
            write_pos,
            write_file_counter: counter,
            _type_store: TypeStore::default(),
        })
    }

    pub fn load(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<RollingLog<TypeStore>>>> {
        let created = Arc::new(RwLock::new(Self::open_impl(
            loader.look_up_resource(file_pattern),
            loader.persistence_path(),
            file_pattern,
            file_fill_size,
        )?));
        loader.add_resource_handle(file_pattern, created.clone())?;
        Ok(created)
    }
    pub fn create(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<RollingLog<TypeStore>>>> {
        let created = Arc::new(RwLock::new(Self::open_impl(
            None,
            loader.persistence_path(),
            file_pattern,
            file_fill_size,
        )?));
        loader.add_resource_handle(file_pattern, created.clone())?;
        Ok(created)
    }

    fn open_write_file(&mut self) -> Result<()> {
        let mut out_file_path_buf = self.file_path.clone();
        out_file_path_buf.push(format!("{}_{}", self.file_pattern, self.write_file_counter));
        let out_file_path = out_file_path_buf.as_path();

        if out_file_path.exists() {
            if !out_file_path.is_file() {
                return Err(PersistenceError::InvalidPathToFile {
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }

            if let Ok(metadata) = fs::metadata(out_file_path) {
                if metadata.len() > self.write_pos {
                    let mut backup_path = self.file_path.clone();
                    backup_path.push(format!(
                        "{}_{}.bak.{}",
                        self.file_pattern,
                        self.write_file_counter,
                        Utc::now().timestamp()
                    ));
                    if self.write_pos > 0 {
                        fs::copy(out_file_path, backup_path.as_path()).context(StdIoDirOpsError)?;
                    } else {
                        fs::rename(out_file_path, backup_path.as_path())
                            .context(StdIoDirOpsError)?;
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
        if file.stream_position().context(StdIoSeekError)? != self.write_pos {
            file.set_len(self.write_pos).context(StdIoWriteError)?;
            let _lines = file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekError)?;
        }
        self.write_to_file = Some(file);
        Ok(())
    }

    // Writes out a resource instance; does not update the commit position, but in this version, does advance the pending commit position.
    // In the future, we may support a queue of commit points, or even entire sequences of pre-written alternative future versions (for chained consensus), which would require a more complex interface.
    pub fn store_resource(
        &mut self,
        resource: &TypeStore::ParamType,
    ) -> Result<StorageLocation> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = TypeStore::store(resource)?;
        let resource_length = serialized.len() as u32;
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
        if self.write_pos >= self.file_fill_size {
            self.write_pos = 0;
            self.write_file_counter += 1;
            self.write_to_file = None;
        }
        self.persisted_sync.advance_next(Some(location));
        Ok(location)
    }

    // This currenty won't have any effect if called again before the atomic store has processed the prior committed version. A more appropriate behavior might be to block. A version that supports queued writes could enqueue the commit points.
    pub fn commit_version(&mut self) {
        self.persisted_sync.update_version();
    }

    pub fn skip_version(&mut self) {
        self.persisted_sync.skip_version();
    }

    // Opens a new handle to the file. Possibly need to revisit in the future?
    pub fn load_latest(&self) -> Result<TypeStore::ParamType> {
        if let Some(location) = self.persisted_sync.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {
                key: self.file_pattern.to_string(),
            })
        }
    }
    pub fn load_specified(
        &self,
        location: &StorageLocation,
    ) -> Result<TypeStore::ParamType> {
        let mut read_file_path_buf = self.file_path.clone();
        read_file_path_buf.push(format!("{}_{}", self.file_pattern, location.file_counter));
        let mut read_file = File::open(read_file_path_buf.as_path()).context(StdIoOpenError)?;
        read_file
            .seek(SeekFrom::Start(location.store_start))
            .context(StdIoSeekError)?;
        let mut reader = read_file.take(location.store_length as u64);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).context(StdIoReadError)?;
        TypeStore::load(&buffer[..])
    }
}

impl<TypeStore: LoadStore> PersistentStore for RollingLog<TypeStore> {
    fn resource_key(&self) -> &str {
        &self.file_pattern
    }
    fn persisted_location(&self) -> Option<StorageLocation> {
        *self.persisted_sync.last_location()
    }
    fn wait_for_version(&self) -> Result<()> {
        self.persisted_sync.wait_for_version();
        Ok(())
    }
    fn start_next_version(&mut self) -> Result<()> {
        self.persisted_sync.start_version()
    }
}
