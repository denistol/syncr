use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::file::file_chunk::{ChunkInfo, get_chunks};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct FileItem {
    pub size: u64,
    pub full_path: PathBuf,
    pub last_mod: Option<i64>,
    pub chunks: Vec<ChunkInfo>,
}
impl FileItem {
    pub fn new(entry: &Path, size: u64, last_mod: Option<i64>) -> Self {
        let item = Self {
            full_path: entry.to_path_buf(),
            size,
            last_mod,
            chunks: vec![],
        };
        return item;
    }
    pub fn get_chunks(&mut self) {
        if let Ok(chunks) = get_chunks(self) {
            self.chunks = chunks;
        }
    }
}

impl Display for FileItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chunks_count = self.chunks.len();

        let chunkz: String = self
            .chunks
            .iter()
            .map(|c| format!("hash: {}, size: {}, offset: {}\n", c.hash, c.size, c.offset))
            .collect();

        write!(
            f,
            "
        SIZE:{}\tFULL_PATH:{}\tCHUNKS_COUNT:{}
        -chunks-
        {}
        --------------
        ",
            self.size,
            self.full_path.to_string_lossy(),
            chunks_count,
            chunkz
        )
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
