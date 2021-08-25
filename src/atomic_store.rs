use crate::error::PersistenceError;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

/// State management for a single persisted resource (a series of items, a snapshot of state, etc.)
/// Resources that store as a sequence may or may not load the entire sequence into memory,
/// and may or may not cache frequently accessed portions
pub trait PersistentStore {
    type ResourceUnit: Serialize + DeserializeOwned;
    /// type for a single serialize+store operation

    /// location for last confirmed atomic update; set when AtomicStore initiates load
    fn persisted_location(&self) -> Option<StorageLocation>;
    /// moving position as local store operations complete
    fn active_location(&self) -> Option<StorageLocation>;
    /// set persisted_location to active_location, perform any cleanup, mark version complete
    /// returns persisted location of latest resource, None if no resource is available
    fn update_location(&mut self) -> Option<StorageLocation>;

    fn wait_for_version(&self) -> Result<(), PersistenceError>;

    /// the resource key should be the file pattern, and the loader will enforce uniqueness
    fn resource_key(&self) -> &str;

    /// persist the next unit of resource
    fn store_resource(
        &mut self,
        resource: &ResourceUnit,
    ) -> Result<StorageLocation, PersistenceError>;

    /// load the latest persisted unit of resource
    fn load_latest(&self) -> Result<ResourceUnit, PersistenceError>;
    /// load a specified unit of resource, if available
    fn load_specified(&self, location: &StorageLocation) -> Result<ResourceUnit, PersistenceError>;
}

/// The central index of an atomic version of truth across multiple persisted data structures
/// This unit allows a load to be confident that it is consistent with a single point in time
/// without persistant store operations blocking the entire system from beginning to end
pub struct AtomicStoreLoader {
    file_path: Path,
    file_pattern: String,
    file_counter: u32,
    // TODO: type checking on load/store format embedded in StorageLocation?
    resource_files: Hashmap<String, StorageLocation>,
    resources: Hashmap<String, Arc<RwLock<dyn PersistentStore>>>,
}

impl AtomicStoreLoader {
    fn new(storage_path: &Path, file_pattern: &str) -> Result<AtomicStoreLoader, PersistenceError> {
        /// FIX
        Err(PersistenceError::FailedToResolvePath)
    }
}

pub struct AtomicStore {
    /// because there is only one instance per file for the table of contents, we do not keep it open.
    file_path: Path,
    file_pattern: String,
    file_counter: u32,

    resources: Hashmap<String, Arc<RwLock<dyn PersistentStore>>>,
}

impl AtomicStore {
    pub fn load_store(load_info: AtomicStoreLoader) -> Result<AtomicStore, PersistenceError>;
    pub fn create_new(deps_info: AtomicStoreLoader) -> Result<AtomicStore, PersistenceError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub file_pattern: String,
    pub file_counter: u32,
    pub file_offset: u64,
}
