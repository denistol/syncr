use std::{cmp::Ordering, fmt::Display, path::PathBuf};

use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub struct FileItem {
    pub full_path: PathBuf,
    pub size: u64,
    pub last_mod: Option<i64>,
}

impl Display for FileItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.size, self.full_path.to_string_lossy())
    }
}

impl PartialEq for FileItem {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl PartialOrd for FileItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}
impl Eq for FileItem {}
impl Ord for FileItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}
