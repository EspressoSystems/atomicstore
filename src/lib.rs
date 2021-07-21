pub mod atomic_store;
pub mod append_log;
pub mod recent_log;
pub mod error;

pub use crate::atomic_store::PersistentStore;

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_append() {}
}
