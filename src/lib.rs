pub mod append_log;
pub mod atomic_store;
pub mod error;
pub mod fixed_append_log;
pub mod rolling_log;
pub mod storage_location;
mod version_sync;

pub use crate::atomic_store::PersistentStore;

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_append() {}
}
