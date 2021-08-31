use crate::atomic_store::StorageLocation;
use crate::error::PersistenceError;

use std::sync::{Arc, Mutex, Condvar};

pub(crate) struct PersistedLocationHandler {
    last_version_location: Option<StorageLocation>,
    next_version_location: Option<StorageLocation>,
    version_pending: Arc<(Mutex<bool>, Condvar)>,
}

impl PersistedLocationHandler {
    pub(crate) fn new(last_version_location: Option<StorageLocation>) -> PersistedLocationHandler {
        PersistedLocationHandler {
            last_version_location: last_version_location.clone(),
            next_version_location : last_version_location,
            version_pending: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }
    pub(crate) fn last_location(&self) -> &Option<StorageLocation> { &self.last_version_location }
    pub(crate) fn next_location(&self) -> &Option<StorageLocation> { &self.next_version_location }
    pub(crate) fn start_version(&mut self) -> Result<(), PersistenceError> {
        let (mtx, _) = &*self.version_pending;
        let guard = mtx.lock().unwrap(); // should not be possible without process corruption
        *guard = false;
    }
    pub(crate) fn advance_next(&mut self, next_version_location: Option<StorageLocation>) {
        self.next_version_location = next_version_location;
    }
    pub(crate) fn update_version(&mut self) {
        let (mtx, cv) = &*self.version_pending;
        let mut guard = mtx.lock().unwrap(); // should not be possible without process corruption
        self.last_version_location = self.next_version_location.clone();
        *guard = true;
        cv.notify_one();
    }
    pub(crate) fn wait_for_version(&self) {
        let version_pending = Arc::clone(&self.version_pending);
        let (mtx, cv) = &*version_pending;
        let mut guard = mtx.lock().unwrap();
        while !*guard {
            cv.wait(guard).unwrap();
        }
    }
}

