use crate::error::{
    BincodeDeError, BincodeSerError, GlobRuntime, GlobSyntax, PersistenceError, StdIoDirOpsError,
    StdIoOpenError, StdIoReadError, StdIoWriteError,
};
use crate::storage_location::StorageLocation;
use crate::version_sync::VersionSyncHandle;
use crate::Result;

use chrono::Utc;
use glob::glob;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, RwLock};

/// This exists to provide a common type for serializing and deserializing of the atomic store
/// table of contents, so the prior state can be pre-loaded without sacrificing single point of initialization.
#[derive(Debug, Serialize, Deserialize)]
struct AtomicStoreFileContents {
    pub file_counter: u32,
    pub resource_files: HashMap<String, StorageLocation>,
}

fn load_state(path: &Path) -> Result<AtomicStoreFileContents> {
    let mut file = File::open(path).context(StdIoOpenError)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).context(StdIoReadError)?;
    bincode::deserialize::<AtomicStoreFileContents>(&buf[..]).context(BincodeDeError)
}

fn extract_count(file_pattern: &str, path_result: &glob::GlobResult) -> Option<u32> {
    if let Ok(path) = path_result {
        let suffix = path
            .file_name()?
            .to_str()?
            .strip_prefix(file_pattern)?
            .strip_prefix("_archived_")?;
        return u32::from_str(suffix).ok();
    }
    None
}

fn format_latest_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_latest", file_pattern));
    buf
}

fn format_archived_file_path(root_path: &Path, file_pattern: &str, counter: u32) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!("{}_archived_{}", file_pattern, counter));
    buf
}

fn format_working_file_path(root_path: &Path, file_pattern: &str) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_working", file_pattern));
    buf
}

/// Enables each managed resource storage instance to initialize before creating the AtomicStore.
pub struct AtomicStoreLoader {
    file_path: PathBuf,
    file_pattern: String,
    file_counter: u32,
    initial_run: bool,
    // TODO: type checking on load/store format embedded in StorageLocation?
    resource_files: HashMap<String, StorageLocation>,
    resources: HashMap<String, Arc<RwLock<VersionSyncHandle>>>,
}

impl AtomicStoreLoader {
    /// Attempt to load the specified atomic state in the specified directory; if no files exist, will initialize a new state
    pub fn load(storage_path: &Path, file_pattern: &str) -> Result<AtomicStoreLoader> {
        let file_path = storage_path.to_path_buf();
        let load_path_buf = format_latest_file_path(storage_path, file_pattern);
        let alt_path_buf;
        let load_path = if load_path_buf.as_path().exists() {
            load_path_buf.as_path()
        } else {
            let max_match = if storage_path.exists() {
                // attempt to use the most recent backup file
                let mut path_pattern_buf = file_path.clone();
                path_pattern_buf.push(format!("{}_archived_*", file_pattern));
                let path_pattern = path_pattern_buf.to_string_lossy().to_string();
                // TODO: could be simplified by doing a length sort and then a lexical sort of the longest length.
                glob(&path_pattern)
                    .context(GlobSyntax)?
                    .max_by_key(|res| -> i32 {
                        if let Some(count) = extract_count(file_pattern, res) {
                            count as i32
                        } else {
                            -1
                        }
                    })
            } else {
                fs::create_dir_all(storage_path).context(StdIoDirOpsError)?;
                None
            };

            // let matches = glob(&path_pattern).context(GlobSyntax)?.filter_map(|path_res| extract_count(file_pattern, path_res)).collect::<BTreeMap<u32,PathBuf>>();
            // TODO: more resiliant approach that may be able to recover after one or more stores are corrupted...
            // for (key, value) in matches.iter().rev() {
            //     // attempt to load, return on success
            // }
            if max_match.is_none() {
                // start from scratch
                return Ok(AtomicStoreLoader {
                    file_path,
                    file_pattern: String::from(file_pattern),
                    file_counter: 0,
                    initial_run: true,
                    resource_files: HashMap::new(),
                    resources: HashMap::new(),
                });
            }
            alt_path_buf = max_match.unwrap().context(GlobRuntime)?;
            alt_path_buf.as_path()
        };
        if !load_path.is_file() {
            return Err(PersistenceError::InvalidPathToFile {
                path: load_path.to_string_lossy().to_string(),
            });
        }
        let loaded_state = load_state(load_path)?;
        Ok(AtomicStoreLoader {
            file_path: storage_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_counter: loaded_state.file_counter,
            initial_run: false,
            resource_files: loaded_state.resource_files,
            resources: HashMap::new(),
        })
    }
    /// Attempt to initialize a new atomic state in the specified directory; if files exist, will back up existing directory before creating
    pub fn create(storage_path: &Path, file_pattern: &str) -> Result<AtomicStoreLoader> {
        if !storage_path.exists() {
            fs::create_dir_all(storage_path).context(StdIoDirOpsError)?;
        } else if format_archived_file_path(storage_path, file_pattern, 0).exists()
            || format_latest_file_path(storage_path, file_pattern).exists()
        {
            let mut backup_path = storage_path.to_path_buf();
            let mut temp_path = storage_path.to_path_buf();
            if !temp_path.pop() {
                // TODO: maybe use some kind of known location instead?
                // std::env::temp_dir() is not guaranteed secure, std::env::current_dir() and std::env::home_dir() are unreliable.
                // Maybe it's better to just require the path to be in a writable parent if you're going to try to create new with existing files.
                return Err(PersistenceError::FailedToResolvePath {
                    path: storage_path.to_string_lossy().to_string(),
                });
            }
            temp_path.push("temporary");
            backup_path.push(format!("backup.{}", Utc::now().timestamp()));

            fs::rename(&storage_path, &temp_path).context(StdIoDirOpsError)?;
            fs::create_dir(&storage_path).context(StdIoDirOpsError)?;
            fs::rename(&temp_path, &backup_path).context(StdIoDirOpsError)?;
        }
        // TODO: sane behavior if files are already present
        Ok(AtomicStoreLoader {
            file_path: storage_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_counter: 0,
            initial_run: true,
            resource_files: HashMap::new(),
            resources: HashMap::new(),
        })
    }
    pub(crate) fn persistence_path(&self) -> &Path {
        self.file_path.as_path()
    }
    pub(crate) fn look_up_resource(&self, key: &str) -> Option<StorageLocation> {
        self.resource_files.get(key).copied()
    }
    pub(crate) fn add_sync_handle(
        &mut self,
        key: &str,
        handle: Arc<RwLock<VersionSyncHandle>>,
    ) -> Result<()> {
        if let Entry::Vacant(insert_point) = self.resources.entry(key.to_string()) {
            insert_point.insert(handle);
        } else {
            return Err(PersistenceError::DuplicateResourceKey {
                key: key.to_string(),
            });
        }
        Ok(())
    }
}

/// The central index of an atomic version of truth across multiple persisted data structures;
/// Guarantees that all managed resources can be loaded in a consistent state across an entire logical entity.
pub struct AtomicStore {
    // because there is only one instance per file for the table of contents, we do not keep it open.
    file_path: PathBuf,
    file_pattern: String,
    file_counter: u32,
    last_counter: Option<u32>,
    resources: HashMap<String, Arc<RwLock<VersionSyncHandle>>>,
}

impl AtomicStore {
    pub fn open(load_info: AtomicStoreLoader) -> Result<AtomicStore> {
        Ok(AtomicStore {
            file_path: load_info.file_path,
            file_pattern: load_info.file_pattern,
            file_counter: if load_info.initial_run {
                load_info.file_counter
            } else {
                load_info.file_counter + 1
            },
            last_counter: if load_info.initial_run {
                None
            } else {
                Some(load_info.file_counter)
            },
            resources: load_info.resources,
        })
    }

    pub fn commit_version(&mut self) -> Result<()> {
        let mut collected_locations = HashMap::<String, StorageLocation>::new();
        for (resource_key, resource_store) in self.resources.iter() {
            {
                let store_access = resource_store.read()?;
                store_access.wait_for_version()?;
                if let Some(location_found) = store_access.last_location() {
                    collected_locations.insert(resource_key.to_string(), *location_found);
                }
            }
            {
                let mut store_access = resource_store.write()?;
                store_access.start_version()?;
            }
        }

        let latest_file_path = format_latest_file_path(&self.file_path, &self.file_pattern);
        let temp_file_path = format_working_file_path(&self.file_path, &self.file_pattern);
        let mut temp_file = File::create(&temp_file_path).context(StdIoOpenError)?;
        let out_state = AtomicStoreFileContents {
            file_counter: self.file_counter,
            resource_files: collected_locations,
        };
        let serialized = bincode::serialize(&out_state).context(BincodeSerError)?;
        temp_file.write_all(&serialized).context(StdIoWriteError)?;
        temp_file.flush().context(StdIoWriteError)?;
        if latest_file_path.exists() {
            let last_counter = if self.last_counter.is_none() {
                let loaded_state = load_state(latest_file_path.as_path())?;
                loaded_state.file_counter
            } else {
                self.last_counter.unwrap()
            };
            let archived_file_path =
                format_archived_file_path(&self.file_path, &self.file_pattern, last_counter);

            fs::rename(&latest_file_path, &archived_file_path).context(StdIoDirOpsError)?;
        }
        self.last_counter = Some(self.file_counter);
        fs::rename(&temp_file_path, &latest_file_path).context(StdIoDirOpsError)?;
        self.file_counter += 1; // advance for the next version
        Ok(())
    }
}
