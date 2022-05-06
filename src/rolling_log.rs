// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::atomic_store::AtomicStoreLoader;
use crate::error::{
    PersistenceError, StdIoDirOpsSnafu, StdIoOpenSnafu, StdIoReadSnafu, StdIoSeekSnafu,
    StdIoWriteSnafu,
};
use crate::load_store::LoadStore;
use crate::storage_location::StorageLocation;
use crate::utils::unix_timestamp;
use crate::version_sync::VersionSyncHandle;
use crate::Result;

use snafu::ResultExt;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

const DEFAULT_RETAINED_ENTRIES: u32 = 128;

#[derive(Debug)]
pub struct RollingLog<ResourceAdaptor: LoadStore> {
    persisted_sync: Arc<RwLock<VersionSyncHandle>>,
    file_path: PathBuf,
    file_pattern: String,
    file_fill_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    file_entries: u32,
    write_file_counter: u32,
    adaptor: ResourceAdaptor,
    retained_entries: u32,
}

fn format_nth_file_path(root_path: &Path, file_pattern: &str, file_count: u32) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_{}", file_pattern, file_count));
    buf
}

fn load_from_file<ResourceAdaptor: LoadStore>(
    read_file: &mut File,
    adaptor: &ResourceAdaptor,
    location: &StorageLocation,
) -> Result<ResourceAdaptor::ParamType> {
    read_file
        .seek(SeekFrom::Start(location.store_start))
        .context(StdIoSeekSnafu)?;
    let mut reader = read_file.take(location.store_length as u64);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).context(StdIoReadSnafu)?;
    adaptor.load(&buffer[..])
}

impl<ResourceAdaptor: LoadStore> RollingLog<ResourceAdaptor> {
    pub(crate) fn open_impl(
        adaptor: ResourceAdaptor,
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        file_fill_size: u64,
        retained_entries: u32,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let (write_pos, counter) = match location {
            Some(ref location) => {
                let append_point = location.store_start + location.store_length as u64;
                if append_point < file_fill_size {
                    (append_point, location.file_counter)
                } else {
                    (4, location.file_counter + 1)
                }
            }
            None => (4, 0), // every rolling file starts with a counter
        };
        Ok(RollingLog {
            persisted_sync: Arc::new(RwLock::new(VersionSyncHandle::new(file_pattern, location))),
            file_path: file_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_fill_size,
            write_to_file: None,
            write_pos,
            file_entries: 0,
            write_file_counter: counter,
            adaptor,
            retained_entries,
        })
    }

    pub fn load(
        loader: &mut AtomicStoreLoader,
        adaptor: ResourceAdaptor,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let resource = loader.look_up_resource(file_pattern);
        let path = loader.persistence_path().to_path_buf();
        let created = Self::open_impl(
            adaptor,
            resource,
            &path,
            file_pattern,
            file_fill_size,
            DEFAULT_RETAINED_ENTRIES,
        )?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }
    pub fn create(
        loader: &mut AtomicStoreLoader,
        adaptor: ResourceAdaptor,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<RollingLog<ResourceAdaptor>> {
        let path = loader.persistence_path().to_path_buf();
        let created = Self::open_impl(
            adaptor,
            None,
            &path,
            file_pattern,
            file_fill_size,
            DEFAULT_RETAINED_ENTRIES,
        )?;
        loader.add_sync_handle(file_pattern, created.persisted_sync.clone())?;
        Ok(created)
    }

    fn open_write_file(&mut self) -> Result<()> {
        let out_file_path =
            format_nth_file_path(&self.file_path, &self.file_pattern, self.write_file_counter);

        if out_file_path.exists() {
            if !out_file_path.is_file() {
                return Err(PersistenceError::InvalidPathToFile {
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }

            if let Ok(metadata) = fs::metadata(&out_file_path) {
                if metadata.len() > self.write_pos {
                    let mut backup_path = self.file_path.clone();
                    backup_path.push(format!(
                        "{}_{}.bak.{}",
                        self.file_pattern,
                        self.write_file_counter,
                        unix_timestamp()
                    ));
                    if self.write_pos > 0 {
                        fs::copy(&out_file_path, &backup_path).context(StdIoDirOpsSnafu)?;
                    } else {
                        fs::rename(&out_file_path, &backup_path).context(StdIoDirOpsSnafu)?;
                    }
                }
            }
        }
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(out_file_path.clone())
            .context(StdIoOpenSnafu)?;
        file.seek(SeekFrom::End(0)).context(StdIoSeekSnafu)?;
        self.file_entries = 0;
        if file.stream_position().context(StdIoSeekSnafu)? == 0 {
            file.write_all(&[0u8; 4]).context(StdIoWriteSnafu)?;
        }
        if file.stream_position().context(StdIoSeekSnafu)? < self.write_pos {
            return Err(PersistenceError::InvalidFileContents {
                note: "file too small for last recorded entry".to_string(),
                path: out_file_path.to_string_lossy().to_string(),
            });
        }
        if file.stream_position().context(StdIoSeekSnafu)? > 4 {
            self.file_entries = 0;
            let _lines = file.seek(SeekFrom::Start(0)).context(StdIoSeekSnafu)?;
            let mut buffer = [0u8; 4];
            file.read_exact(&mut buffer).context(StdIoReadSnafu)?;
            let remembered_entries = u32::from_le_bytes(buffer);
            let mut read_position = 4u64;
            while read_position < self.write_pos {
                let mut buffer = [0u8; 4];
                file.read_exact(&mut buffer).context(StdIoReadSnafu)?;
                read_position += 4;
                let entry_size = u32::from_le_bytes(buffer);
                read_position += entry_size as u64;
                let _lines = file
                    .seek(SeekFrom::Start(read_position))
                    .context(StdIoSeekSnafu)?;
                self.file_entries += 1;
            }
            if read_position > self.write_pos {
                return Err(PersistenceError::InvalidFileContents {
                    note: format!(
                        "file stream mismatch for last recorded entry: {} > {}",
                        read_position, self.write_pos
                    ),
                    path: out_file_path.to_string_lossy().to_string(),
                });
            }
            if self.file_entries != remembered_entries {
                file.seek(SeekFrom::Start(0)).context(StdIoSeekSnafu)?;
                file.write_all(&self.file_entries.to_le_bytes())
                    .context(StdIoWriteSnafu)?;
            }
            file.seek(SeekFrom::End(0)).context(StdIoSeekSnafu)?;
        }
        if file.stream_position().context(StdIoSeekSnafu)? > self.write_pos {
            file.set_len(self.write_pos).context(StdIoWriteSnafu)?;
            let _lines = file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekSnafu)?;
        }
        self.write_to_file = Some(file);
        assert!(self.write_pos > 0);
        Ok(())
    }

    // Writes out a resource instance; does not update the commit position, but in this version,
    // does advance the pending commit position.
    pub fn store_resource(
        &mut self,
        resource: &ResourceAdaptor::ParamType,
    ) -> Result<StorageLocation> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = self.adaptor.store(resource)?;
        let resource_length = serialized.len() as u32;
        assert_eq!(resource_length.to_le_bytes().len(), 4);
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&resource_length.to_le_bytes())
            .context(StdIoWriteSnafu)?;
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&serialized)
            .context(StdIoWriteSnafu)?;

        let location = StorageLocation {
            file_counter: self.write_file_counter,
            store_start: self.write_pos + 4,
            store_length: resource_length,
        };

        self.write_pos += (4 + resource_length) as u64;
        self.file_entries += 1;
        if self.write_pos >= self.file_fill_size {
            if let Some(mut write_to_file) = self.write_to_file.as_ref() {
                let _lines = write_to_file
                    .seek(SeekFrom::Start(0))
                    .context(StdIoSeekSnafu)?;
                write_to_file
                    .write_all(&self.file_entries.to_le_bytes())
                    .context(StdIoWriteSnafu)?;
            }
            self.write_pos = 4;
            self.file_entries = 0;
            self.write_file_counter += 1;
            self.write_to_file = None;
        }
        self.persisted_sync.write()?.advance_next(Some(location));
        Ok(location)
    }

    // This currenty won't have any effect if called again before the atomic store has processed the prior committed version. A more appropriate behavior might be to block. A version that supports queued writes could enqueue the commit points.
    pub fn commit_version(&mut self) -> Result<()> {
        if let Some(mut write_to_file) = self.write_to_file.as_ref() {
            let _lines = write_to_file
                .seek(SeekFrom::Start(0))
                .context(StdIoSeekSnafu)?;
            write_to_file
                .write_all(&self.file_entries.to_le_bytes())
                .context(StdIoWriteSnafu)?;
            let _lines = write_to_file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekSnafu)?;
        }
        self.persisted_sync.write()?.update_version()
    }

    pub fn skip_version(&mut self) -> Result<()> {
        self.persisted_sync.write()?.skip_version()
    }

    pub fn revert_version(&mut self) -> Result<()> {
        self.write_to_file = None;
        self.persisted_sync.write()?.revert_version()
    }

    // Opens a new handle to the file. Possibly need to revisit in the future?
    pub fn load_latest(&self) -> Result<ResourceAdaptor::ParamType> {
        if let Some(location) = self.persisted_sync.read()?.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {
                key: self.file_pattern.to_string(),
            })
        }
    }
    pub fn load_specified(&self, location: &StorageLocation) -> Result<ResourceAdaptor::ParamType> {
        let read_file_path =
            format_nth_file_path(&self.file_path, &self.file_pattern, location.file_counter);
        let mut read_file = File::open(read_file_path.as_path()).context(StdIoOpenSnafu)?;
        load_from_file::<ResourceAdaptor>(&mut read_file, &self.adaptor, location)
    }

    pub fn set_retained_entries(&mut self, retained_entries: u32) {
        self.retained_entries = retained_entries;
    }

    // Prune write files after the total number of entries exceeds the retained number
    pub fn prune_file_entries(&self) -> Result<()> {
        if let Some(commit_pos) = self.persisted_sync.read()?.last_location() {
            let mut file_index = commit_pos.file_counter;
            let mut retained_counter = self
                .retained_entries
                .checked_sub(commit_pos.store_length)
                .unwrap_or(0);
            loop {
                if file_index == 0 {
                    break;
                }
                file_index -= 1;

                let path = format_nth_file_path(&self.file_path, &self.file_pattern, file_index);
                if !path.exists() {
                    break;
                } else if retained_counter == 0 {
                    fs::remove_file(path).context(StdIoDirOpsSnafu)?;
                } else {
                    let mut read_file = File::open(path).context(StdIoOpenSnafu)?;
                    let mut buffer = [0u8; 4];
                    read_file.read_exact(&mut buffer).context(StdIoReadSnafu)?;
                    let store_length = u32::from_le_bytes(buffer);
                    retained_counter = retained_counter.checked_sub(store_length).unwrap_or(0);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_store::BincodeLoadStore;
    use tempfile::TempDir;

    #[test]
    fn prune_file_entries() -> Result<()> {
        let loader_tag = "test_key";
        let value_tag = format!("{}_type", loader_tag);
        let dir: TempDir = tempfile::Builder::new().tempdir().unwrap();
        let mut loader = AtomicStoreLoader::load(dir.path(), loader_tag).unwrap();
        let mut log: RollingLog<BincodeLoadStore<u64>> =
            RollingLog::load(&mut loader, Default::default(), &value_tag, 1).unwrap();

        log.store_resource(&5).unwrap();
        log.store_resource(&5).unwrap();
        log.commit_version().unwrap();

        let first_file_path = format_nth_file_path(dir.path(), &value_tag, 0);
        assert!(first_file_path.exists());

        log.set_retained_entries(1);
        log.prune_file_entries().unwrap();
        assert!(!first_file_path.exists());

        Ok(())
    }
}
