use crate::atomic_store::{AtomicStoreLoader, PersistentStore};
use crate::error::{
    PersistenceError, StdIoDirOpsError, StdIoOpenError, StdIoReadError, StdIoSeekError,
    StdIoWriteError,
};
use crate::fixed_append_log;
use crate::fixed_append_log::FixedAppendLog;
use crate::load_store::{LoadStore, StorageLocationLoadStore};
use crate::storage_location::{StorageLocation, STORAGE_LOCATION_SERIALIZED_SIZE};
use crate::version_sync::PersistedLocationHandler;
use crate::Result;

use chrono::Utc;
use snafu::ResultExt;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct AppendLog<TypeStore: LoadStore> {
    persisted_sync: PersistedLocationHandler,
    file_path: PathBuf,
    file_pattern: String,
    file_fill_size: u64,
    write_to_file: Option<File>,
    write_pos: u64,
    write_file_counter: u32,
    index_log: Arc<RwLock<FixedAppendLog<StorageLocationLoadStore>>>,
    _type_store: TypeStore,
}

pub struct Iter<TypeStore: LoadStore> {
    inner_iter: fixed_append_log::Iter<StorageLocationLoadStore>,
    file_path: PathBuf,
    file_pattern: String,
    read_from_file: Option<File>,
    read_from_counter: u32,
    _type_store: TypeStore,
}

fn format_index_file_pattern(file_pattern: &str) -> String {
    format!("{}_index", file_pattern)
}

fn format_nth_file_path(root_path: &Path, file_pattern: &str, file_count: u32) -> PathBuf {
    let mut buf = root_path.to_path_buf();
    buf.push(format!(".{}_{}", file_pattern, file_count));
    buf
}

fn load_from_file<TypeStore: LoadStore>(
    read_file: &mut File,
    location: &StorageLocation,
) -> Result<TypeStore::ParamType> {
    read_file
        .seek(SeekFrom::Start(location.store_start))
        .context(StdIoSeekError)?;
    let mut reader = read_file.take(location.store_length as u64);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).context(StdIoReadError)?;
    TypeStore::load(&buffer[..])
}

impl<TypeStore: 'static + LoadStore + Default> AppendLog<TypeStore> {
    pub(crate) fn open_impl(
        loader: &mut AtomicStoreLoader,
        location: Option<StorageLocation>,
        file_path: &Path,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<AppendLog<TypeStore>> {
        let index_pattern = format_index_file_pattern(file_pattern);
        let (write_pos, counter, index_log) = match location {
            Some(ref location) => {
                let index_log: Arc<RwLock<FixedAppendLog<StorageLocationLoadStore>>> =
                    FixedAppendLog::load(
                        loader,
                        &index_pattern,
                        STORAGE_LOCATION_SERIALIZED_SIZE,
                        256,
                    )?;
                let append_point = location.store_start + location.store_length as u64;
                if append_point < file_fill_size {
                    (append_point, location.file_counter, index_log)
                } else {
                    (0, location.file_counter + 1, index_log)
                }
            }
            None => {
                let index_log: Arc<RwLock<FixedAppendLog<StorageLocationLoadStore>>> =
                    FixedAppendLog::create(
                        loader,
                        &index_pattern,
                        STORAGE_LOCATION_SERIALIZED_SIZE,
                        256,
                    )?;
                (0, 0, index_log)
            }
        };

        Ok(AppendLog {
            persisted_sync: PersistedLocationHandler::new(location),
            file_path: file_path.to_path_buf(),
            file_pattern: String::from(file_pattern),
            file_fill_size,
            write_to_file: None,
            write_pos,
            write_file_counter: counter,
            index_log,
            _type_store: TypeStore::default(),
        })
    }

    pub fn load(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<AppendLog<TypeStore>>>> {
        let resource = loader.look_up_resource(file_pattern);
        let path = loader.persistence_path().to_path_buf();
        let created = Arc::new(RwLock::new(Self::open_impl(
            loader,
            resource,
            &path,
            file_pattern,
            file_fill_size,
        )?));
        loader.add_resource_handle(file_pattern, created.clone())?;
        Ok(created)
    }
    pub fn create(
        loader: &mut AtomicStoreLoader,
        file_pattern: &str,
        file_fill_size: u64,
    ) -> Result<Arc<RwLock<AppendLog<TypeStore>>>> {
        let path = loader.persistence_path().to_path_buf();
        let created = Arc::new(RwLock::new(Self::open_impl(
            loader,
            None,
            &path,
            file_pattern,
            file_fill_size,
        )?));
        loader.add_resource_handle(file_pattern, created.clone())?;
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
                        Utc::now().timestamp()
                    ));
                    if self.write_pos > 0 {
                        fs::copy(&out_file_path, &backup_path).context(StdIoDirOpsError)?;
                    } else {
                        fs::rename(&out_file_path, &backup_path).context(StdIoDirOpsError)?;
                    }
                }
            }
        }
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(out_file_path)
            .context(StdIoOpenError)?;
        if file.stream_position().context(StdIoSeekError)? != self.write_pos {
            file.set_len(self.write_pos).context(StdIoWriteError)?;
            let _lines = file
                .seek(SeekFrom::Start(self.write_pos))
                .context(StdIoSeekError)?;
        }
        self.write_to_file = Some(file);
        Ok(())
    }

    // Writes out a resource instance; does not update the commit position, but in this version, does advance the pending commit position.
    // In the future, we may support a queue of commit points, or even entire sequences of pre-written alternative future versions (for chained consensus), which would require a more complex interface.
    pub fn store_resource(&mut self, resource: &TypeStore::ParamType) -> Result<StorageLocation> {
        if self.write_to_file.is_none() {
            self.open_write_file()?;
        }
        let serialized = TypeStore::store(resource)?;
        let resource_length = serialized.len() as u32;
        self.write_to_file
            .as_ref()
            .unwrap()
            .write_all(&serialized)
            .context(StdIoWriteError)?;

        let location = StorageLocation {
            file_counter: self.write_file_counter,
            store_start: self.write_pos,
            store_length: resource_length,
        };

        self.write_pos += resource_length as u64;
        if self.write_pos >= self.file_fill_size {
            self.write_pos = 0;
            self.write_file_counter += 1;
            self.write_to_file = None;
        }
        self.index_log.write()?.store_resource(&location)?;
        self.persisted_sync.advance_next(Some(location));
        Ok(location)
    }

    // This currenty won't have any effect if called again before the atomic store has processed the prior committed version. A more appropriate behavior might be to block. A version that supports queued writes could enqueue the commit points.
    pub fn commit_version(&mut self) -> Result<()> {
        self.index_log.write()?.commit_version()?;
        self.persisted_sync.update_version();
        Ok(())
    }

    pub fn skip_version(&mut self) -> Result<()> {
        self.index_log.write()?.skip_version()?;
        self.persisted_sync.skip_version();
        Ok(())
    }

    // Opens a new handle to the file. Possibly need to revisit in the future?
    pub fn load_latest(&self) -> Result<TypeStore::ParamType> {
        if let Some(location) = self.persisted_sync.last_location() {
            self.load_specified(location)
        } else {
            Err(PersistenceError::FailedToFindExpectedResource {
                key: self.file_pattern.to_string(),
            })
        }
    }
    pub fn load_specified(&self, location: &StorageLocation) -> Result<TypeStore::ParamType> {
        let read_file_path =
            format_nth_file_path(&self.file_path, &self.file_pattern, location.file_counter);
        let mut read_file = File::open(read_file_path.as_path()).context(StdIoOpenError)?;
        load_from_file::<TypeStore>(&mut read_file, location)
    }

    pub fn iter(&self) -> Iter<TypeStore> {
        Iter {
            inner_iter: self.index_log.read().unwrap().iter(),
            file_path: self.file_path.clone(),
            file_pattern: self.file_pattern.clone(),
            read_from_file: None,
            read_from_counter: 0,
            _type_store: TypeStore::default(),
        }
    }
}

impl<TypeStore: LoadStore> Iter<TypeStore> {
    fn helper(&mut self, location: &StorageLocation) -> Result<TypeStore::ParamType> {
        if location.file_counter != self.read_from_counter {
            self.read_from_file = None;
        }
        if self.read_from_file.is_none() {
            self.read_from_counter = location.file_counter;
            let read_file_path =
                format_nth_file_path(&self.file_path, &self.file_pattern, location.file_counter);
            self.read_from_file =
                Some(File::open(read_file_path.as_path()).context(StdIoOpenError)?);
        }
        load_from_file::<TypeStore>(&mut self.read_from_file.as_mut().unwrap(), location)
    }
}

impl<TypeStore: LoadStore> Iterator for Iter<TypeStore> {
    type Item = Result<TypeStore::ParamType>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.inner_iter
                .next()?
                .map_or_else(Err, |loc| self.helper(&loc)),
        )
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_iter.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        Some(
            self.inner_iter
                .nth(n)?
                .map_or_else(Err, |loc| self.helper(&loc)),
        )
    }
}

impl<TypeStore: LoadStore> ExactSizeIterator for Iter<TypeStore> {
    fn len(&self) -> usize {
        self.inner_iter.len()
    }
}

impl<TypeStore: LoadStore> PersistentStore for AppendLog<TypeStore> {
    fn resource_key(&self) -> &str {
        &self.file_pattern
    }
    fn persisted_location(&self) -> Option<StorageLocation> {
        *self.persisted_sync.last_location()
    }
    fn wait_for_version(&self) -> Result<()> {
        self.persisted_sync.wait_for_version();
        Ok(())
    }
    fn start_next_version(&mut self) -> Result<()> {
        self.persisted_sync.start_version()
    }
}
