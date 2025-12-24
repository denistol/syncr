use crate::constants::{MASK, MAX_CHUNK, MIN_CHUNK, WINDOW};
use crate::file::file_item::FileItem;

use bincode::{Decode, Encode};
use blake3;
use serde::{Deserialize, Serialize};

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

#[derive(Debug, Serialize, Deserialize, Encode, Decode, Clone)]
pub struct ChunkInfo {
    pub hash: String,
    pub offset: u64,
    pub size: u32,
}

const READ_BUF_SIZE: usize = 1024 * 1024; // 1MB

pub fn get_chunks(file_info: &FileItem) -> io::Result<Vec<ChunkInfo>> {
    let mut file = File::open(&file_info.full_path)?;

    let mut chunks = Vec::new();
    let mut read_buf = vec![0u8; READ_BUF_SIZE];

    let mut rolling_window: VecDeque<u8> = VecDeque::with_capacity(WINDOW);
    let mut rolling_hash: u32 = 0;

    let mut file_offset: u64 = 0;
    let mut chunk_start: u64 = 0;

    let mut chunk_hasher = blake3::Hasher::new();

    loop {
        let n = file.read(&mut read_buf)?;
        if n == 0 {
            break;
        }

        for &b in &read_buf[..n] {
            file_offset += 1;

            if rolling_window.len() == WINDOW {
                let old = rolling_window.pop_front().unwrap();
                rolling_hash = rolling_hash.wrapping_sub(old as u32);
            }

            rolling_window.push_back(b);
            rolling_hash = rolling_hash.wrapping_add(b as u32);

            chunk_hasher.update(&[b]);

            let chunk_size = file_offset - chunk_start;

            if (chunk_size >= MIN_CHUNK as u64 && (rolling_hash & MASK) == 0)
                || chunk_size >= MAX_CHUNK as u64
            {
                let hash = chunk_hasher.finalize().to_string();

                chunks.push(ChunkInfo {
                    hash,
                    offset: chunk_start,
                    size: chunk_size as u32,
                });

                chunk_start = file_offset;
                rolling_window.clear();
                rolling_hash = 0;
                chunk_hasher = blake3::Hasher::new();
            }
        }
    }

    if chunk_start < file_offset {
        let chunk_size = file_offset - chunk_start;
        let hash = chunk_hasher.finalize().to_string();

        chunks.push(ChunkInfo {
            hash,
            offset: chunk_start,
            size: chunk_size as u32,
        });
    }

    Ok(chunks)
}
