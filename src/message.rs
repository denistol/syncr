use std::sync::Arc;

use crate::client::Client;

enum Message {
    ListClients(Vec<u8>),
    GetClients,
    To(Arc<Client>, Vec<u8>)
}
