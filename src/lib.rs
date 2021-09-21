pub mod append_log;
pub mod atomic_store;
pub mod error;
pub mod fixed_append_log;
pub mod rolling_log;
pub mod storage_location;
pub mod version_sync;
pub mod load_store;

pub use crate::{
    append_log::AppendLog,
    atomic_store::{AtomicStore, AtomicStoreLoader, PersistentStore},
    error::PersistenceError,
    fixed_append_log::FixedAppendLog,
    rolling_log::RollingLog,
};

/// Convenience type alias
pub type Result<T> = std::result::Result<T, error::PersistenceError>;
