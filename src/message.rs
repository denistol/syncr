use std::{error::Error, str::Bytes, sync::Arc};

use crate::client::Client;

pub enum Message {
    GetDir,
    ShowDir,
}

impl Message {
    pub fn to_byte(&self) -> u8 {
        match self {
            Message::GetDir => 49, // 1
            Message::ShowDir => 50, // 2
        }
    }

    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            49 => Some(Self::GetDir), // 1
            50 => Some(Self::ShowDir), // 2
            _ => None,
        }
    }
}
