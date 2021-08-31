use crate::atomic_store::{AtomicStoreLoader, PersistentStore, StorageLocation};
use crate::error::PersistenceError;
use crate::utils::PersistedLocationHandler;

use chrono::Utc;
use serde::Serialize;
use serde::de::DeserializeOwned;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

struct AppendLog<StoredResource: 'static> {
    persisted_sync: PersistedLocationHandler,
    file_path: PathBuf,
    file_pattern: String,
    file_fill_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    write_file_counter: u32,
    phantom: PhantomData<&'static StoredResource>,
}

impl<StoredResource: Serialize + DeserializeOwned> AppendLog<StoredResource> {
    pub(crate) fn open_impl(
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
        let (write_pos, counter) = match location {
            Some(ref location) => {
                let append_point = location.store_start + location.store_length;
                if append_point < file_fill_size {
                    (append_point, location.file_counter)
                } else {
                    (0, location.file_counter + 1)
                }
            }
            None => (0, 0),
        };
        Ok(AppendLog {
            persisted_sync: PersistedLocationHandler::new(location),
            file_path: file_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_fill_size,
            write_to_file: None,
            write_pos,
            write_file_counter: counter,
            phantom: PhantomData,
        })
    }

    pub fn open(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<AppendLog<StoredResource>>>, PersistenceError> {
        let created = Arc::new(RwLock::new(Self::open_impl(loader.look_up_resource(file_pattern), loader.persistence_path(), file_pattern, file_fill_size)?));
        loader.add_resource_handle(file_pattern, created.clone());
        Ok(created)
    }
    pub fn create_new(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<AppendLog<StoredResource>>>, PersistenceError> {
        let created = Arc::new(RwLock::new(Self::open_impl(None, loader.persistence_path(), file_pattern, file_fill_size)?));
        loader.add_resource_handle(file_pattern, created.clone());
        Ok(created)
    }

    fn open_write_file(&mut self) -> Result<(), PersistenceError> {
        let mut out_file_path_buf = self.file_path.clone();
        out_file_path_buf.set_file_name(format!("{}_{}", self.file_pattern, self.write_file_counter));
        let out_file_path = out_file_path_buf.as_path();
        if out_file_path.exists() {
            if !out_file_path.is_file() {
                return Err(PersistenceError::InvalidPathToFile{path: out_file_path.to_string_lossy().to_string()});
            }
            if let Ok(metadata) = fs::metadata(out_file_path) {
                if metadata.len() > self.write_pos {
                    let mut backup_path = self.file_path.clone();
                    backup_path.set_file_name(format!("{}_{}.bak.{}", self.file_pattern, self.write_file_counter, Utc::now().timestamp()));
                    if self.write_pos > 0 {
                        fs::copy(out_file_path, backup_path.as_path());
                    } else {
                        fs::rename(out_file_path, backup_path.as_path());
                    }
                }
            }
        }
        let mut file = OpenOptions::new().read(true).append(true).create(true).open(out_file_path)?;
        if file.stream_position()? != self.write_pos {
            file.set_len(self.write_pos);
            file.seek(SeekFrom::Start(self.write_pos));
        }
        self.write_to_file = Some(file);
        Ok(())
    }

    pub fn store_resource(
        &mut self,
        resource: &StoredResource,
    ) -> Result<StorageLocation, PersistenceError> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = bincode::serialize(resource)?;
        self.write_to_file.unwrap().write_all(&serialized);
        self.write_pos = self.write_pos + serialized.len() as u64;
        if self.write_pos >= self.file_fill_size {
            self.write_pos = 0;
            self.write_file_counter = self.write_file_counter + 1;
            self.write_to_file = None;
        }
        Ok(StorageLocation {file_counter: self.write_file_counter, store_start: self.write_pos, store_length: 0})
    }

        /// Opens a new handle to the file. Possibly need to revisit in the future?
    pub fn load_latest(&self) -> Result<StoredResource, PersistenceError> {
        if let Some(location) = self.persisted_sync.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {key: self.file_pattern})
        }
    }
    pub fn load_specified(&self, location: &StorageLocation) -> Result<StoredResource, PersistenceError> {
        let mut read_file_path_buf = self.file_path.clone();
        read_file_path_buf.set_file_name(format!("{}_{}", self.file_pattern, location.file_counter));
        let read_file_path = read_file_path_buf.as_path();
        let mut read_file = File::open(read_file_path_buf.as_path())?;
        read_file.seek(SeekFrom::Start(location.store_start));
        let mut reader = read_file.take(location.store_length);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let resource = bincode::deserialize(&buffer[..])?;
        Ok(resource)
    }
}

impl<StoredResource> PersistentStore for AppendLog<StoredResource> {
    fn persisted_location(&self) -> Option<StorageLocation> {
        self.persisted_sync.last_location().clone()
    }
    fn active_location(&self) -> Option<StorageLocation> {
        self.persisted_sync.next_location().clone()
    }
    fn update_location(&mut self) -> Option<StorageLocation> {
        Some(StorageLocation { file_counter: self.write_file_counter, store_start: self.write_pos, store_length: 0 })
    }
    fn wait_for_version(&self) -> Result<(), PersistenceError> {
        self.persisted_sync.wait_for_version();
        Ok(())
    }
    fn resource_key(&self) -> &str {
        &self.file_pattern
    }

}
