use crate::error::PersistenceError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// State management for a single persisted resource (a series of items, a snapshot of state, etc.)
/// Resources that store as a sequence may or may not load the entire sequence into memory,
/// and may or may not cache frequently accessed portions
pub trait PersistentStore {
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
}

/// The central index of an atomic version of truth across multiple persisted data structures
/// This unit allows a load to be confident that it is consistent with a single point in time
/// without persistant store operations blocking the entire system from beginning to end
pub struct AtomicStoreLoader {
    file_path: PathBuf,
    file_pattern: String,
    file_counter: u32,
    // TODO: type checking on load/store format embedded in StorageLocation?
    resource_files: HashMap<String, StorageLocation>,
    resources: HashMap<String, Arc<RwLock<dyn PersistentStore>>>,
}

impl AtomicStoreLoader {
    pub fn new(storage_path: &Path, file_pattern: &str) -> Result<AtomicStoreLoader, PersistenceError> {
        Ok(AtomicStoreLoader {
            file_path: storage_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_counter: 0,
        })
    }
    pub(crate) fn persistence_path(&self) -> &Path {
        self.file_path.as_path()
    }
    pub(crate) fn look_up_resource(&self, key: &str) -> Option<StorageLocation> {
        self.resource_files.get(key)
    }
    pub(crate) fn add_resource_handle(&mut self, key: &str, resource: Arc<RwLock<dyn PersistentStore>>) -> Result<(), PersistenceError> {
        self.resources.try_insert(key, resource)?;
        ()
    }
}

pub struct AtomicStore {
    /// because there is only one instance per file for the table of contents, we do not keep it open.
    file_path: PathBuf,
    file_pattern: String,
    file_counter: u32,

    resources: HashMap<String, Arc<RwLock<dyn PersistentStore>>>,
}

impl AtomicStore {
    fn load_store(load_info: AtomicStoreLoader) -> Result<AtomicStore, PersistenceError> {
        Ok(AtomicStore {
            file_path: load_info.file_path,
            file_pattern: load_info.file_pattern,
            file_counter: load_info.file_counter,
            resources: load_info.resources,
        })
    }
    fn create_new(deps_info: AtomicStoreLoader) -> Result<AtomicStore, PersistenceError> {
        Ok(AtomicStore {
            file_path: deps_info.file_path,
            file_pattern: deps_info.file_pattern,
            file_counter: deps_info.file_counter,
            resources: deps_info.resources,
        })
    }


}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub file_counter: u32,
    pub store_start: u64,
    pub store_length: u64,
}
