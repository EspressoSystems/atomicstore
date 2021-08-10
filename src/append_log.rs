use crate::atomic_store::{PersistentStore, StorageLocation};
use crate::error::PersistenceError;
use serde::{Deserialize, Serialize};
use std::fs::File;

struct AppendLog<StoredResource> {
    current_file: File,
    current_write_pos: u64,
    file_pattern: String,
    file_counter: u32,
}

impl AppendLog<StoredResource> {
    pub fn load(
        file_pattern: &str,
        counter: u32,
        write_pos: u64,
    ) -> Result<AppendLog<StoredResource>, PersistenceError> {
    }

    pub fn create_new(file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}

    pub fn delete_all(file_pattern: &str) -> Result<AppendLog<StoredResource>, PersistenceError> {}
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
