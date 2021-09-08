use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct StorageLocation {
    pub store_start: u64,
    pub store_length: u32,
    pub file_counter: u32,
}

impl fmt::Display for StorageLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "@{}{{{}+{}}}",
            self.file_counter, self.store_start, self.store_length
        )
    }
}
