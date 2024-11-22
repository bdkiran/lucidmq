use log::error;
use nolan::Commitlog;
use std::sync::Mutex;

use crate::{lucidmq_errors::ProducerError, message::Message};

pub struct Producer {
    topic: String,
    commitlog: Mutex<Commitlog>,
}

impl Producer {
    pub fn new(
        directory: String,
        topic_name: String,
        max_segment_size_bytes: u64,
        max_commitlog_size: u64,
    ) -> Result<Producer, ProducerError> {
        let cl = Commitlog::new(&directory, max_segment_size_bytes, max_commitlog_size).map_err(|e| {
            error!("{}", e);
            ProducerError::new("Unable to create a producer for the given commitlog")
        })?;
        Ok(Producer {
            topic: topic_name,
            commitlog: Mutex::new(cl),
        })
    }

    pub fn produce_message(&mut self, mut message: Message) -> Result<u16, ProducerError> {
        let message_bytes = message.serialize_message().map_err(|e| {
            error!("{}", e);
            ProducerError::new("Unable to produce message to the commitlog")
        })?;
        self.produce_bytes(&message_bytes)
    }

    pub fn produce_bytes(&mut self, bytes: &[u8]) -> Result<u16, ProducerError> {
        let mut cl = self.commitlog.lock().map_err(|e| {
            error!("{}", e);
            ProducerError::new("Unable to produce message to the commitlog")
        })?;
        cl.append(bytes).map_err(|e| {
            error!("{}", e);
            ProducerError::new("Unable to produce message to the commitlog")
        })
    }


    pub fn get_topic(&self) -> String {
        self.topic.clone()
    }
}
