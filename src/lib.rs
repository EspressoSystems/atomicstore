// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod testing;

mod utils;

#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

pub mod append_log;
pub mod atomic_store;
pub mod error;
pub mod fixed_append_log;
pub mod kv_store;
pub mod load_store;
pub mod rolling_log;
pub mod storage_location;
pub mod version_sync;

pub use crate::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader},
    error::PersistenceError,
    fixed_append_log::FixedAppendLog,
    rolling_log::RollingLog,
};

/// Convenience type alias
pub type Result<T> = std::result::Result<T, error::PersistenceError>;
