pub struct Message {
    buffer: Vec<u8>,
    pub filled: bool,
    pub data: Vec<u8>,
}

static START_HEADER: &[u8] = b"START";
static END_HEADER: &[u8] = b"END";

impl Message {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            filled: false,
            data: Vec::new(),
        }
    }

    pub fn append(&mut self, incoming: &[u8]) {
        self.buffer.extend_from_slice(incoming);

        if let Some(start_pos) = find_subsequence(&self.buffer, START_HEADER) {
            if let Some(end_rel) =
                find_subsequence(&self.buffer[start_pos + START_HEADER.len()..], END_HEADER)
            {
                let end_pos = start_pos + START_HEADER.len() + end_rel;

                self.data = self.buffer[start_pos + START_HEADER.len()..end_pos].to_vec();
                self.filled = true;

                self.buffer.drain(..end_pos + END_HEADER.len());
            }
        } else {
            if self.buffer.len() > START_HEADER.len() {
                let keep = START_HEADER.len() - 1;
                let drain_len = self.buffer.len() - keep;
                self.buffer.drain(..drain_len);
            }
        }
    }

    pub fn is_filled(&self) -> bool {
        self.filled
    }

    pub fn reset(&mut self) {
        self.filled = false;
        self.data.clear();
    }

    pub fn print_message(&self) {
        if self.filled {
            println!("MESSAGE READY: {}", String::from_utf8_lossy(&self.data));
        }
    }
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}
