use crate::{constants::START_HEADER, sync_message::SyncMessage};

#[derive(Debug)]
pub struct SyncEvent {
    pub inner_buffer: Vec<u8>,
    pub data: Vec<u8>,
}
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}
impl ToString for SyncEvent {
    fn to_string(&self) -> String {
        return String::from_utf8(self.data.clone()).unwrap_or(String::new());
    }
}
impl SyncEvent {
    pub fn new() -> Self {
        Self {
            inner_buffer: vec![],
            data: vec![],
        }
    }

    pub fn parse(&mut self, incomig: &[u8]) -> Option<SyncMessage> {
        self.inner_buffer.extend_from_slice(incomig);

        let has_start = find_subsequence(&self.inner_buffer, START_HEADER);
        if let Some(start_header_index) = has_start {
            let end = START_HEADER.len() + start_header_index;

            let msg_size_from = end;
            let msg_size_end = msg_size_from + 50;

            let content_length = &self.inner_buffer[msg_size_from..msg_size_end];

            println!(
                "=========: {:?}",
                String::from_utf8(content_length.to_vec()).unwrap()
            );
        }

        None
    }
}
