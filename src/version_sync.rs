// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

// boy oh boy did clippy get this one wrong
#![allow(clippy::mutex_atomic)]

use crate::storage_location::StorageLocation;
use crate::Result;

use std::{
    sync::{Arc, Condvar, Mutex},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct VersionSyncHandle {
    last_version_location: Option<StorageLocation>,
    next_version_location: Option<StorageLocation>,
    version_pending: Arc<(Mutex<bool>, Condvar)>,
    _resource_key: String,
}

impl VersionSyncHandle {
    pub fn new(key: &str, last_version_location: Option<StorageLocation>) -> VersionSyncHandle {
        VersionSyncHandle {
            last_version_location,
            next_version_location: last_version_location,
            version_pending: Arc::new((Mutex::new(false), Condvar::new())),
            _resource_key: key.to_string(),
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
        let mut version_ready = mtx.lock()?;
        *version_ready = false;
        Ok(())
    }
    pub fn advance_next(&mut self, next_version_location: Option<StorageLocation>) {
        self.next_version_location = next_version_location;
    }
    pub fn update_version(&mut self) -> Result<()> {
        let (mtx, cv) = &*self.version_pending;
        let mut version_ready = mtx.lock()?;
        if !*version_ready {
            self.last_version_location = self.next_version_location;
            *version_ready = true;
            cv.notify_one();
        }
        Ok(())
    }
    pub fn skip_version(&mut self) -> Result<()> {
        let (mtx, cv) = &*self.version_pending;
        let mut version_ready = mtx.lock()?;
        if !*version_ready {
            *version_ready = true;
            cv.notify_one();
        }
        Ok(())
    }
    pub fn revert_version(&mut self) -> Result<()> {
        let (mtx, _cv) = &*self.version_pending;
        let _version_ready = mtx.lock()?;
        self.next_version_location = self.last_version_location;
        Ok(())
    }

    pub fn wait_for_version_with_timeout(&self, timeout: Duration) -> Result<()> {
        let version_pending = Arc::clone(&self.version_pending);
        let (mtx, cv) = &*version_pending;
        let mut version_ready = mtx.lock()?;
        // Get the current time, to make sure that `mtx` won't constantly update with `false` and this function won't
        // ever time out
        let start_time = Instant::now();
        while !*version_ready {
            // Make sure the `timeout` has not elapsed since the beginning of this function
            let elapsed = start_time.elapsed();
            if elapsed >= timeout {
                return Err(crate::error::PersistenceError::TimedOut);
            }

            let remaining_time = timeout - elapsed;
            let (guard, result) = cv.wait_timeout(version_ready, remaining_time)?;
            if result.timed_out() {
                return Err(crate::error::PersistenceError::TimedOut);
            }
            version_ready = guard;
        }
        Ok(())
    }
}
