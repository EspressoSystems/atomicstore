use bincode;
use snafu::Snafu;

/// Error type for AtomicStore
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
#[non_exhaustive]
pub enum PersistenceError {
    /// Failed to resolve path
    #[snafu(display("Failed to resolve a path '{:?}'", path))]
    FailedToResolvePath {
        /// The provided path
        path: String,
    },
    /// Failed to find resource
    #[snafu(display("Failed to find an expected resource '{:?}'", key))]
    FailedToFindExpectedResource {
        /// The resource key
        key: String,
    },
    /// Path to file is invalid
    #[snafu(display("Path '{:?}' cannot be used for a file", path))]
    InvalidPathToFile {
        /// The provided path
        path: String,
    },
    /// Failed to write to file
    #[snafu(display("Failed to write resource to file {:?} at {}", filename, position))]
    FailedToWriteToFile {
        /// The name of the actual file
        filename: String,
        /// the write position in the file
        position: u64,
    },
    /// Unspecified IO Error
    #[snafu(display("std::io::Error {:?}", e))]
    StdIoError {
        e: std::io::Error,
    },
    /// Unspecified bincode ser/de Error
    #[snafu(display("bincode error {:?}", e))]
    BincodeError {
        e: bincode::Error,
    }
}

impl From<std::io::Error> for PersistenceError {
    fn from(e: std::io::Error) -> Self {
        PersistenceError::StdIoError{e}
    }
}

impl From<bincode::Error> for PersistenceError {
    fn from(e: bincode::Error) -> Self {
        PersistenceError::BincodeError{e}
    }
}