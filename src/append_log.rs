use crate::atomic_store::{AtomicStoreLoader, PersistentStore, StorageLocation};
use crate::error::PersistenceError;
use crate::utils::PersistedLocationHandler;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::path::Path;

struct AppendLog<StoredResource> {
    persisted_sync: PersistedLocationHandler,
    file_path: Path,
    file_pattern: String,
    max_file_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    write_file_counter: u32,
}

impl AppendLog<StoredResource> {
    pub(crate) fn open_impl(
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        max_file_size: u64,
        counter: u32,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
        let write_pos = match (location) {
            Some(ref location) => location.store_start + location.store_length,
            None => 0,
        };
        Ok(AppendLog {
            persisted_sync: PersistedLocationHandler::new(location),
            file_path: file_path.clone(),
            file_pattern: String::from(file_pattern),
            max_file_size,
            write_to_file: None,
            write_pos,
            write_file_counter,
        })
    }

    pub(crate) fn create_new_impl(
        file_path: &Path,
        file_pattern: &str,
        max_file_size: u64,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }

    pub(crate) fn delete_all_impl(
        file_path: &Path,
        file_pattern: &str,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }

    pub fn open(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        max_file_size: u64,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }
    pub fn create_new(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        max_file_size: u64,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }
}

impl PersistentStore<StoredResource> for AppendLog {
    type ResourceUnit = StoredResource;
    fn persisted_location(&self) -> Option<StorageLocation> {
        self.location.last_location()
    }
    fn active_location(&self) -> Option<StorageLocation> {
        self.location.next_location()
    }
    fn update_location(&mut self) -> Option<StorageLocation> {
        self.location.update_version()
    }

    fn store_resource(
        &mut self,
        resource: &ResourceUnit,
    ) -> Result<StorageLocation, PersistenceError>;

    fn load_latest(&self) -> Result<ResourceUnit, PersistenceError>;
    fn load_specified(&self, location: &StorageLocation) -> Result<ResourceUnit, PersistenceError>;
}
