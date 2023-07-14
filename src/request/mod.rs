/// This module contains the functionality to deserialize NATs requests on the publishing side
/// and deserialize/extract the header/data from the NATs request on the subscribing side.
///
/// # Example
///
/// ```rust
mod nats_request;
pub use nats_request::{NatsEnvelope, RequestHeaders};

mod converter;
pub use converter::Converter;
