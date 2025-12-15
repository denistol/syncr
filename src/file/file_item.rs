use bincode::{Decode, Encode};
use blake3;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct ChunkInfo {
    pub hash: String,
    pub offset: u64,
    pub size: u32,
}
#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct FileItem {
    pub size: u64,
    pub full_path: PathBuf,
    pub last_mod: Option<i64>,
    pub chunks: Vec<ChunkInfo>,
}
const CHUNK_SIZE: usize = 4 * 1024 * 1024; // 4MB

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
        if let Ok(mut f) = File::open(&self.full_path) {
            let mut buf = vec![0u8; CHUNK_SIZE];
            let mut offset: u64 = 0;

            loop {
                let chunk_offset = offset;
                let n = match f.read(&mut buf) {
                    Ok(0) => break, // EOF
                    Ok(n) => n,
                    Err(_) => break,
                };

                let hash = blake3::hash(&buf[..n]).to_hex().to_string();
                let chnunk_info = ChunkInfo {
                    hash,
                    offset: chunk_offset,
                    size: n as u32,
                };
                self.chunks.push(chnunk_info);
                offset += n as u64;
            }
        };
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
