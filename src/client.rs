use std::{net::{TcpListener, TcpStream}, sync::Arc};

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub enum ClientStatus {
    ACTIVE,
    DISABLED
}
pub struct Client {
    id: String,
    // connected_at: DateTime<Utc>,
    pub last_message_at: Option<DateTime<Utc>>,
    pub stream: Option<TcpStream>,
    // status: ClientStatus,
    // listener: Arc<TcpListener>,
    // clients: Arc<Vec<Client>>,
}
impl Client {
    pub fn new (stream: TcpStream) -> Self {
        let id = Uuid::new_v4().to_string();
        println!("[+] Added new client: {}", &id);
        Self {
            id,
            last_message_at: None,
            stream: Some(stream)
        }
    }
}