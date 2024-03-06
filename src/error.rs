// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the AtomicStore library.

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
// You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::storage_location::StorageLocation;
use ark_serialize;
use bincode;
use snafu::prelude::*;

/// Error type for AtomicStore
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum PersistenceError {
    /// Failed to resolve path
    #[snafu(display("Failed to resolve a path '{path}'"))]
    FailedToResolvePath {
        /// The provided path
        path: String,
    },
    /// Failed to find resource
    #[snafu(display("Failed to find an expected resource '{key}'"))]
    FailedToFindExpectedResource {
        /// The resource key
        key: String,
    },
    /// Path to file is invalid
    #[snafu(display("Path '{path}' cannot be used for a file"))]
    InvalidPathToFile {
        /// The provided path
        path: String,
    },
    /// Path to file is invalid
    #[snafu(display("File '{path}' does not contain valid data: note: '{note}'"))]
    InvalidFileContents {
        note: String,
        /// The provided path
        path: String,
    },
    /// Failed to write to file
    #[snafu(display("Failed to write resource to file '{filename}' at {position}"))]
    FailedToWriteToFile {
        /// The name of the actual file
        filename: String,
        /// the write position in the file
        position: u64,
    },
    /// Duplicate resource name
    #[snafu(display("Resource key collision for '{key}'"))]
    DuplicateResourceKey {
        /// Resource key/file pattern
        key: String,
    },
    /// Stored state mismatch with load specification
    #[snafu(display("Stored state for '{key}' is inconsistent with startup specification"))]
    ResourceFormatInconsistent {
        /// Resource key/file pattern
        key: String,
    },
    /// Unimplemented feature
    #[snafu(display("Feature not yet implemented: {description}"))]
    FeatureNotYetImplemented { description: String },
    /// std::io directory operations error
    StdIoDirOps { source: std::io::Error },
    /// std::io open error
    StdIoOpen { source: std::io::Error },
    /// std::io seek error
    StdIoSeek { source: std::io::Error },
    /// std::io write error
    StdIoWrite { source: std::io::Error },
    /// std::io read error
    StdIoRead { source: std::io::Error },
    /// Bincode serialization error
    BincodeSer { source: bincode::Error },
    /// Bincode deserialization error
    BincodeDe { source: bincode::Error },
    /// ArkWorks serialization error
    #[snafu(display("Arkworks Serialization Error on write: {}", err))]
    ArkSer {
        err: ark_serialize::SerializationError,
    },
    /// ArkWorks deserialization error
    #[snafu(display("Arkworks Serialization Error on read: {}", err))]
    ArkDe {
        err: ark_serialize::SerializationError,
    },
    OtherStore {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
    OtherLoad {
        inner: Box<dyn std::error::Error + Send + Sync>,
    },
    /// Placeholder for PoisonError specializations
    SyncPoison { description: String },
    /// `AtomicStore::commit_version` took to long to wait for Log versions and timed out
    TimedOut,
    #[snafu(display("location {stored_location:?} stored by log is older than location {expected_location:?} in global index"))]
    LocationOutOfDate {
        stored_location: StorageLocation,
        expected_location: StorageLocation,
    },
}

impl<T> From<std::sync::PoisonError<T>> for PersistenceError {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        PersistenceError::SyncPoison {
            description: error.to_string(),
        }
    }
}
