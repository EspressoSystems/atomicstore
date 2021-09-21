// boy oh boy did clippy get this one wrong
#![allow(clippy::mutex_atomic)]

use crate::storage_location::StorageLocation;
use crate::Result;

use std::sync::{Arc, Condvar, Mutex};

#[derive(Debug)]
pub struct PersistedLocationHandler {
    last_version_location: Option<StorageLocation>,
    next_version_location: Option<StorageLocation>,
    version_pending: Arc<(Mutex<bool>, Condvar)>,
}

impl PersistedLocationHandler {
    pub fn new(last_version_location: Option<StorageLocation>) -> PersistedLocationHandler {
        PersistedLocationHandler {
            last_version_location,
            next_version_location: last_version_location,
            version_pending: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }
    pub fn last_location(&self) -> &Option<StorageLocation> {
        &self.last_version_location
    }
    // pub(crate) fn next_location(&self) -> &Option<StorageLocation> {
    //     &self.next_version_location
    // }
    pub fn start_version(&mut self) -> Result<()> {
        let (mtx, _) = &*self.version_pending;
        let mut version_ready = mtx.lock().unwrap(); // should not be possible without process corruption
        *version_ready = false;
        Ok(())
    }
    pub fn advance_next(&mut self, next_version_location: Option<StorageLocation>) {
        self.next_version_location = next_version_location;
    }
    pub fn update_version(&mut self) {
        let (mtx, cv) = &*self.version_pending;
        let mut version_ready = mtx.lock().unwrap(); // should not be possible without process corruption
        if !*version_ready {
            self.last_version_location = self.next_version_location;
            *version_ready = true;
            cv.notify_one();
        }
    }
    pub fn skip_version(&mut self) {
        let (mtx, cv) = &*self.version_pending;
        let mut version_ready = mtx.lock().unwrap();
        if !*version_ready {
            *version_ready = true;
            cv.notify_one();
        }
    }

    pub fn wait_for_version(&self) {
        let version_pending = Arc::clone(&self.version_pending);
        let (mtx, cv) = &*version_pending;
        let mut version_ready = mtx.lock().unwrap();
        while !*version_ready {
            version_ready = cv.wait(version_ready).unwrap();
        }
    }
}
