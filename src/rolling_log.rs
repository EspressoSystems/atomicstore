use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::error::PersistenceError;
use crate::atomic_store::{PersistentStore, StorageLocation};

struct RollingLog<StoredResource> {
    current_file: File,
    current_write_pos: u64,
    file_pattern: String,
    file_counter: u32,
}

impl RollingLog<StoredResource> {
  pub fn load(file_pattern: &str) -> Result<RollingLog<StoredResource>, PersistenceError> {

  }

  pub fn create_new(file_pattern: &str) -> Result<RollingLog<StoredResource>, PersistenceError> {

  }

  pub fn delete_all(file_pattern: &str) -> Result<RollingLog<StoredResource>, PersistenceError> {

  }
}

impl PersistentStore<StoredResource> for RollingLog {
    type ResourceUnit = StoredResource;
    fn persisted_location(&self) -> Option<StorageLocation>;
    fn active_location(&self) -> Option<StorageLocation>;
    fn update_location(&mut self) -> Option<StorageLocation>;

    fn store_resource(&mut self, resource: &ResourceUnit) -> Result<StorageLocation, PersistenceError>;
    fn load_latest(&self) -> Result<ResourceUnit, PersistenceError>;
    fn load_specified(&self, location: &StorageLocation) -> Result<ResourceUnit, PersistenceError>;
}
