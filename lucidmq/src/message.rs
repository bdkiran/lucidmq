use log::error;
use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::lucidmq_errors::MessageError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Message {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: i64,
}

impl Message {
    pub fn new(key: &[u8], value: &[u8], timestamp: Option<i64>) -> Message {
        let message_timestamp: i64 = match timestamp {
            Some(timestamp) => timestamp,
            None => SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        };
        let key_vec = key.to_vec();
        let value_vec = value.to_vec();
        Message {
            key: key_vec,
            value: value_vec,
            timestamp: message_timestamp,
        }
    }

    pub fn serialize_message(&mut self) -> Result<Vec<u8>, MessageError> {
        bincode::serialize(&self).map_err(|e| {
            error!("{}", e);
            MessageError::new("Unable to serialize message")
        })
    }

    pub fn deserialize_message(message_bytes: &[u8]) -> Result<Message, MessageError>  {
        bincode::deserialize(message_bytes).map_err(|e| {
            error!("{}", e);
            MessageError::new("Unable to deserialize message")
        })
    }
}
