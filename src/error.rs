use ark_serialize;
use bincode;
use glob;
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
    /// Path to file is invalid
    #[snafu(display("File {:?} does not contain valid data", path))]
    InvalidFileContents {
        note: String,
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
    /// Duplicate resource name
    #[snafu(display("Resource key collision for {}", key))]
    DuplicateResourceKey {
        /// Resource key/file pattern
        key: String,
    },
    /// Stored state mismatch with load specification
    #[snafu(display("Stored state for {} is inconsistent with startup specification", key))]
    ResourceFormatInconsistent {
        /// Resource key/file pattern
        key: String,
    },
    /// Unimplemented feature
    #[snafu(display("Feature not yet implemented: {}", description))]
    FeatureNotYetImplemented { description: String },
    /// std::io directory operations error
    StdIoDirOpsError { source: std::io::Error },
    /// std::io open error
    StdIoOpenError { source: std::io::Error },
    /// std::io seek error
    StdIoSeekError { source: std::io::Error },
    /// std::io write error
    StdIoWriteError { source: std::io::Error },
    /// std::io read error
    StdIoReadError { source: std::io::Error },
    /// Bincode serialization error
    BincodeSerError { source: bincode::Error },
    /// Bincode deserialization error
    BincodeDeError { source: bincode::Error },
    /// ArkWorks serialization error
    #[snafu(display("Arkworks Serialization Error on write: {}", err))]
    ArkSerError {
        err: ark_serialize::SerializationError,
    },
    /// ArkWorks deserialization error
    #[snafu(display("Arkworks Serialization Error on read: {}", err))]
    ArkDeError {
        err: ark_serialize::SerializationError,
    },
    OtherStoreError {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
    OtherLoadError {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Glob syntax error
    GlobSyntax { source: glob::PatternError },
    /// Glob iteration error
    GlobRuntime { source: glob::GlobError },
    /// Placeholder for PoisonError specializations
    SyncPoisonError { description: String },
}

impl<T> From<std::sync::PoisonError<T>> for PersistenceError {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        PersistenceError::SyncPoisonError {
            description: error.to_string(),
        }
    }
}
