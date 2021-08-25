use crate::atomic_store::{AtomicStoreLoader, PersistentStore, StorageLocation};
use crate::error::PersistenceError;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

struct AppendLog<StoredResource> {
    file_path: Path,
    file_pattern: String,
    last_version_location: StorageLocation,
    next_version_location: StorageLocation,
    write_to_file: File,
    write_pos: u64,
    write_file_counter: u32,
}

impl AppendLog<StoredResource> {
    pub(crate) fn load_impl(
        file_path: &Path,
        file_pattern: &str,
        write_pos: u64,
        counter: u32,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }

    pub(crate) fn create_new_impl(file_path: &Path, file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}

    pub(crate) fn delete_all_impl(file_path: &Path, file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}

    pub fn load(loader: &mut AtomicStoreLoader, file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}
    pub fn create_new(loader: &mut AtomicStoreLoader, file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}
}

impl PersistentStore<StoredResource> for AppendLog {
    type ResourceUnit = StoredResource;
    fn persisted_location(&self) -> Option<StorageLocation>;
    fn active_location(&self) -> Option<StorageLocation>;
    fn update_location(&mut self) -> Option<StorageLocation>;

    fn store_resource(
        &mut self,
        resource: &ResourceUnit,
    ) -> Result<StorageLocation, PersistenceError>;

    fn load_latest(&self) -> Result<ResourceUnit, PersistenceError>;
    fn load_specified(&self, location: &StorageLocation) -> Result<ResourceUnit, PersistenceError>;
}
