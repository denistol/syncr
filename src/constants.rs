pub const START_HEADER: &[u8] = b"<START>";
pub const END_HEADER: &[u8] = b"<END>";
pub const LOG_FILENAME: &str = "syncr.bin";
pub const BUFFER_SIZE: usize = 4096;

pub const WINDOW: usize = 64; // rolling hash size
pub const AVG_CHUNK: usize = 4 * 1024 * 1024;
pub const MIN_CHUNK: usize = AVG_CHUNK / 4;
pub const MAX_CHUNK: usize = AVG_CHUNK * 2;
pub const MASK: u32 = (AVG_CHUNK / WINDOW - 1) as u32;
