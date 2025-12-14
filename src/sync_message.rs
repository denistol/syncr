use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub enum SyncMessageType {
    GetFiles,
    ListFiles,
}
pub struct SyncMessage {
    stream: Arc<Mutex<TcpStream>>,
    message_type: SyncMessageType,
    data: Option<Vec<u8>>,
}
