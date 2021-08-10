pub mod append_log;
pub mod atomic_store;
pub mod error;
pub mod rolling_log;

pub use crate::atomic_store::PersistentStore;

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_append() {}
}
