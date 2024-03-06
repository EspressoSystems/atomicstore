// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::error::{
    BincodeDeSnafu, BincodeSerSnafu, PersistenceError, StdIoDirOpsSnafu, StdIoOpenSnafu,
    StdIoReadSnafu, StdIoWriteSnafu,
};
use crate::storage_location::StorageLocation;
use crate::utils::unix_timestamp;
use crate::version_sync::VersionSyncHandle;
use crate::Result;

use regex::Regex;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// This exists to provide a common type for serializing and deserializing of the atomic store
/// table of contents, so the prior state can be pre-loaded without sacrificing single point of initialization.
#[derive(Debug, Serialize, Deserialize)]
struct AtomicStoreFileContents {
    pub file_counter: u32,
    pub resource_files: HashMap<String, StorageLocation>,
}

fn load_state(path: &Path) -> Result<AtomicStoreFileContents> {
    let mut file = File::open(path).context(StdIoOpenSnafu)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).context(StdIoReadSnafu)?;
    bincode::deserialize::<AtomicStoreFileContents>(&buf[..]).context(BincodeDeSnafu)
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

/// Iterate over archive files in the directory `root_path`.
///
/// Yields (in an unspecified order) file paths for each archive file in `root_path`, along with the
/// file counter associated with each archive file.
fn archive_files<'a>(
    root_path: &'a Path,
    file_pattern: &'a str,
) -> Result<impl 'a + Iterator<Item = Result<(PathBuf, u32)>>> {
    let re = Regex::new(&format!("^{file_pattern}_archived_(\\d+)$")).unwrap();
    Ok(fs::read_dir(root_path)
        .context(StdIoDirOpsSnafu)?
        .map(move |de| {
            let de = de.context(StdIoDirOpsSnafu)?;
            let os_name = de.file_name();
            let Some(name) = os_name.to_str() else {
                // If the file name is not unicode, it cannot match the archive file pattern.
                return Ok(None);
            };
            let Some(captures) = re.captures(name) else {
                return Ok(None);
            };
            // The regex used cannot match without capturing something in the first capture group,
            // so `.get(1).unwrap()` will never panic.
            let archive_num_str = captures.get(1).unwrap().as_str();
            let Ok(archive_num) = archive_num_str.parse() else {
                // If the "archive number" portion of the file name is not a number, then this is
                // not an archive file after all.
                return Ok(None);
            };
            Ok(Some((root_path.join(name), archive_num)))
        })
        // Filter out `Ok(None)` entries, which are files that didn't match the archive file
        // pattern.
        .filter_map(Result::transpose))
}

fn archive_file_exists(root_path: &Path, file_pattern: &str) -> Result<bool> {
    let mut err = None;
    for res in archive_files(root_path, file_pattern)? {
        match res {
            // If the iterator yields any element successfully, then at least one archive file
            // exists.
            Ok(_) => return Ok(true),
            Err(e) => err = Some(e),
        }
    }
    if let Some(err) = err {
        // If we didn't find an archive file but encountered an error while iterating, we can't be
        // sure whether there is an archive file.
        Err(err)
    } else {
        Ok(false)
    }
}

fn prune_archives(root_path: &Path, file_pattern: &str, below: u32) -> Result<()> {
    for res in archive_files(root_path, file_pattern)? {
        let (path, num) = res?;
        if num >= below {
            continue;
        }
        fs::remove_file(path).context(StdIoDirOpsSnafu)?;
    }
    Ok(())
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
    // How many backup index files to retain at any given time. If `None`, all archives will be
    // retained.
    retained_archives: Option<u32>,
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
                archive_files(storage_path, file_pattern)?
                    .max_by_key(|res| Some(res.as_ref().ok()?.1))
                    .and_then(|res| Some(res.ok()?.0))
            } else {
                fs::create_dir_all(storage_path).context(StdIoDirOpsSnafu)?;
                None
            };

            if max_match.is_none() {
                // start from scratch
                return Ok(AtomicStoreLoader {
                    file_path,
                    file_pattern: String::from(file_pattern),
                    file_counter: 0,
                    initial_run: true,
                    resource_files: HashMap::new(),
                    resources: HashMap::new(),
                    retained_archives: None,
                });
            }
            alt_path_buf = max_match.unwrap();
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
            retained_archives: None,
        })
    }
    /// Attempt to initialize a new atomic state in the specified directory; if files exist, will back up existing directory before creating
    pub fn create(storage_path: &Path, file_pattern: &str) -> Result<AtomicStoreLoader> {
        if !storage_path.exists() {
            fs::create_dir_all(storage_path).context(StdIoDirOpsSnafu)?;
        } else if archive_file_exists(storage_path, file_pattern)?
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
            backup_path.push(format!("backup.{}", unix_timestamp()));

            fs::rename(storage_path, &temp_path).context(StdIoDirOpsSnafu)?;
            fs::create_dir(storage_path).context(StdIoDirOpsSnafu)?;
            fs::rename(&temp_path, &backup_path).context(StdIoDirOpsSnafu)?;
        }
        // TODO: sane behavior if files are already present
        Ok(AtomicStoreLoader {
            file_path: storage_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_counter: 0,
            initial_run: true,
            resource_files: HashMap::new(),
            resources: HashMap::new(),
            retained_archives: None,
        })
    }

    pub fn retain_archives(&mut self, retained_archives: u32) {
        self.retained_archives = Some(retained_archives);
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
    // How long `commit_version` will wait for resource versions before returning an error.
    // defaults to 100 milliseconds
    commit_timeout: Duration,
    // How many backup index files to retain at any given time. If `None`, all archives will be
    // retained.
    retained_archives: Option<u32>,
}

impl AtomicStore {
    pub fn open(load_info: AtomicStoreLoader) -> Result<AtomicStore> {
        let commit_timeout = if std::env::var("ATOMIC_STORE_NO_TIMEOUT").is_ok() {
            Duration::MAX
        } else {
            Duration::from_millis(100)
        };

        if !load_info.initial_run {
            if let Some(retained_archives) = load_info.retained_archives {
                // At startup, prune all existing archive files that fall outside the retaining
                // window. This is necessary in case the `retained_archives` parameter has changed
                // since the last time the store was opened. Once this is done, we know the number
                // of archive files on disk is consistent with `retained_archives`, and we only need
                // to check for at most one old archive each time we commit a new version.
                prune_archives(
                    &load_info.file_path,
                    &load_info.file_pattern,
                    load_info.file_counter.saturating_sub(retained_archives),
                )?;
            }
        }

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
            commit_timeout,
            retained_archives: load_info.retained_archives,
        })
    }

    /// Set the commit timeout. By default, `AtomicStore` will wait 100 milliseconds for all logs to be committed.
    pub fn set_commit_timeout(&mut self, timeout: Duration) {
        self.commit_timeout = timeout;
    }

    /// Commit the version. Note that all logs and stores must call `.commit_version()` or `.skip_version()` before this function is called.
    ///
    /// This will timeout after 100 milliseconds (configurable with `set_commit_timeout`). If you want to disable this timeout, set the `ATOMIC_STORE_NO_TIMEOUT` environment variable before calling `AtomicStore::open`.
    pub fn commit_version(&mut self) -> Result<()> {
        let mut collected_locations = HashMap::<String, StorageLocation>::new();
        for (resource_key, resource_store) in self.resources.iter() {
            {
                let store_access = resource_store.read()?;
                store_access.wait_for_version_with_timeout(self.commit_timeout)?;
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
        let mut temp_file = File::create(&temp_file_path).context(StdIoOpenSnafu)?;
        let out_state = AtomicStoreFileContents {
            file_counter: self.file_counter,
            resource_files: collected_locations,
        };
        let serialized = bincode::serialize(&out_state).context(BincodeSerSnafu)?;
        temp_file.write_all(&serialized).context(StdIoWriteSnafu)?;
        temp_file.flush().context(StdIoWriteSnafu)?;
        temp_file.sync_all().context(StdIoDirOpsSnafu)?;
        if latest_file_path.exists() {
            let last_counter = if self.last_counter.is_none() {
                let loaded_state = load_state(latest_file_path.as_path())?;
                loaded_state.file_counter
            } else {
                self.last_counter.unwrap()
            };
            let archived_file_path =
                format_archived_file_path(&self.file_path, &self.file_pattern, last_counter);

            fs::rename(&latest_file_path, archived_file_path).context(StdIoDirOpsSnafu)?;
        }
        self.last_counter = Some(self.file_counter);
        fs::rename(&temp_file_path, &latest_file_path).context(StdIoDirOpsSnafu)?;

        // Prune an old archive if this commit has just pushed one outside of the retention window.
        if let Some(retained_archives) = self.retained_archives {
            if let Some(num) = self.file_counter.checked_sub(retained_archives + 1) {
                let prune_path =
                    format_archived_file_path(&self.file_path, &self.file_pattern, num);
                if prune_path.exists() {
                    if let Err(err) = fs::remove_file(&prune_path) {
                        // If we fail to prune the old archive, we have still committed the version
                        // update, so we shouldn't fail here. Just log a warning.
                        tracing::warn!(
                            %err,
                            path = %prune_path.display(),
                            "failed to prune old archive file",
                        );
                    }
                }
            }
        }

        self.file_counter += 1; // advance for the next version
        Ok(())
    }
}

#[test]
fn test_atomic_store_log_timeout() {
    use crate::load_store::BincodeLoadStore;

    let prefix = "test_atomic_store_log_timeout";
    let dir = tempfile::tempdir().expect("Could not create tempdir");
    let mut loader =
        AtomicStoreLoader::load(dir.path(), prefix).expect("Could not open an atomic store");
    let mut log = crate::AppendLog::load(
        &mut loader,
        BincodeLoadStore::<u64>::default(),
        prefix,
        1024,
    )
    .expect("Could not create appendlog");
    let mut store = AtomicStore::open(loader).expect("Could not open store");

    // oops we forgot to commit log
    if let Err(crate::error::PersistenceError::TimedOut) = store.commit_version() {
        // ok
    } else {
        panic!("Atomic store should've timed out");
    }

    // Committing again should work
    log.commit_version().expect("Could not commit log");
    store.commit_version().expect("Could not commit store");
}

#[test]
fn test_archive_pruning() {
    let dir = tempfile::tempdir().expect("Could not create tempdir");
    let file_pattern = "test_archive_pruning";

    let list_archives = || {
        let mut versions = archive_files(dir.path(), file_pattern)
            .unwrap()
            .map(|res| res.unwrap().1)
            .collect::<Vec<_>>();
        versions.sort();
        versions
    };

    // First create a store without any pruning and create several archive files.
    {
        let loader = AtomicStoreLoader::create(dir.path(), file_pattern)
            .expect("Could not create an atomic store");
        let mut store = AtomicStore::open(loader).expect("Could not open store");

        for _ in 0..5 {
            store.commit_version().expect("Could not commit store");
        }
        // There should be one archive for each committed version except the latest.
        assert_eq!(list_archives(), vec![0, 1, 2, 3]);
    }

    // Now reopen the store with pruning turned on; ensure old versions get pruned immediately.
    {
        let mut loader = AtomicStoreLoader::load(dir.path(), file_pattern)
            .expect("Could not create an atomic store");
        loader.retain_archives(2);
        let mut store = AtomicStore::open(loader).expect("Could not open store");

        assert_eq!(list_archives(), vec![2, 3]);

        // Commit more versions, check that we always retain the two latest archives.
        for i in 5..10 {
            store.commit_version().expect("Could not commit store");
            assert_eq!(list_archives(), vec![i - 2, i - 1])
        }
    }

    // Check that we can delete the latest version and reload from an archive, even though some
    // older archives have been pruned.
    {
        let latest = format_latest_file_path(dir.path(), file_pattern);
        assert!(latest.exists());
        fs::remove_file(latest).expect("Could not delete latest version");

        let loader = AtomicStoreLoader::load(dir.path(), file_pattern)
            .expect("Could not create an atomic store");
        let store = AtomicStore::open(loader).expect("Could not open store");
        assert_eq!(store.file_counter, 9);
    }
}
