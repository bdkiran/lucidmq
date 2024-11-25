use pyo3::prelude::*;

mod consumer;
mod lucidmq;
mod message;
mod producer;
mod lucidmq_errors;

use consumer::Consumer;
use lucidmq::LucidMQ;
use message::Message;
use producer::Producer;

/// A Python module implemented in Rust.
#[pymodule]
fn pylucidmq(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Message>()?;
    m.add_class::<Producer>()?;
    m.add_class::<Consumer>()?;
    m.add_class::<LucidMQ>()?;
    Ok(())
}
