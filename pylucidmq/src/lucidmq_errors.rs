use std::error::Error;
use std::fmt;

use pyo3::exceptions::PyOSError;
use pyo3::PyErr;

//------------LucidMQ Error--------------------
#[derive(Debug, PartialEq)]
pub struct LucidMqError {
    details: String,
}

impl LucidMqError {
    pub fn new(msg: &str) -> LucidMqError {
        LucidMqError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for LucidMqError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for LucidMqError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::convert::From<LucidMqError> for PyErr {
    fn from(err: LucidMqError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}


//------------Topic Error--------------------
#[derive(Debug, PartialEq)]
pub struct TopicError {
    details: String,
}

impl TopicError {
    pub fn new(msg: &str) -> TopicError {
        TopicError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TopicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TopicError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::convert::From<TopicError> for PyErr {
    fn from(err: TopicError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}


//------------Message Error--------------------
#[derive(Debug, PartialEq)]
pub struct MessageError {
    details: String,
}

impl MessageError {
    pub fn new(msg: &str) -> MessageError {
        MessageError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for MessageError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::convert::From<MessageError> for PyErr {
    fn from(err: MessageError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}


//------------Consumer Error--------------------
#[derive(Debug, PartialEq)]
pub struct ConsumerError {
    details: String,
}

impl ConsumerError {
    pub fn new(msg: &str) -> ConsumerError {
        ConsumerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ConsumerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ConsumerError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::convert::From<ConsumerError> for PyErr {
    fn from(err: ConsumerError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}

//------------Producer Error--------------------
#[derive(Debug, PartialEq)]
pub struct ProducerError {
    details: String,
}

impl ProducerError {
    pub fn new(msg: &str) -> ProducerError {
        ProducerError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ProducerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ProducerError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl std::convert::From<ProducerError> for PyErr {
    fn from(err: ProducerError) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}