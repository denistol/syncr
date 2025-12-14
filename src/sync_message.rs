use std::{net::TcpStream, sync::{Arc, Mutex}};

pub enum SyncMessageType {
    GET_FILES,
    LIST_FILES,
}
pub struct SyncMessage {
    stream: Arc<Mutex<TcpStream>>,
    message_type: SyncMessageType,
    data: Option<Vec<u8>>,
}