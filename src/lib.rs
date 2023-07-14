#![deny(unsafe_code, unused_qualifications, trivial_casts)]
#![deny(clippy::all)]
// #![warn(clippy::pedantic)]
// This library wraps the [async-nats] crate adding centralized functionality
// of serializion (gRPC and JSON) and applying consistent message wrapper and metadata
//
// It also provides the plumbing for standardization of Requests, Replies and Event messages the transport over NATS.

/// This module will provide the creation/wrapping of External Events that are published to NATS for any subscribers to pick up.
pub mod event;

/// This module provides the creation/wrapping of Requests that are then published to NATS for transport
/// for processing by a NATS subscriber
pub mod request;

/// This module provides the creation/wrapping of  Responses that are then published back to NATS for transport
/// as Reply messages (in response to NATs Requests)
pub mod response;

/// This module contains the [NatsServer] struct which provides NATS server access
pub mod server;

mod subject;
pub use subject::SubjectName;

#[allow(unused_qualifications)]
#[allow(clippy::all)]
pub mod proto_test {
    tonic::include_proto!("test");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("test");
}
