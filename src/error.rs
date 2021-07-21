use snafu::Snafu;

/// Error type for AtomicStore
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
#[non_exhaustive]
pub enum PersistenceError {
    /// Failed to write to file
    #[snafu(display("Failed to write resource to file {:?} at {}", filename, position))]
    FailedToWriteToFile {
        /// The name of the actual file
        filename: String,
        /// the write position in the file
        position: u64,
    },

}