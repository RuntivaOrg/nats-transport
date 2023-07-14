use async_trait::async_trait;
use prost::Message;
use serde::Serialize;

use crate::server::NatsTransportError;

/// Publish is a [NatsServer] trait used to publish a NATS message using a gRPC protocol buffer
#[async_trait]
pub trait PublishProst<T>: Send + Sync
where
    T: Message + Default + 'static,
{
    /// Loads an Aggregate Root instance from the data store,
    /// referenced by its unique identifier.
    async fn publish(&self, subject: String, msg: T) -> Result<(), NatsTransportError>;
}

/// Publish is a [NatsServer] trait used to publish a NATS message using JSON serialization
#[async_trait]
pub trait PublishJson<T>: Send + Sync
where
    T: Serialize + 'static,
{
    /// Loads an Aggregate Root instance from the data store,
    /// referenced by its unique identifier.
    async fn publish(&self, subject: String, msg: T) -> Result<(), NatsTransportError>;
}

/// Request is a [NatsServer] trait used to perform a request/reply NATS message using a gRPC protocol buffer
#[async_trait]
pub trait RequestProst<T>: Send + Sync
where
    T: Message + Default + Send + Sync + 'static,
{
    /// Loads an Aggregate Root instance from the data store,
    /// referenced by its unique identifier.
    async fn request(
        &self,
        subject: String,
        msg: T,
    ) -> Result<async_nats::Message, NatsTransportError>;
}

/// Request is a [NatsServer] trait used to perform a request/reply NATS message using JSON serialization
#[async_trait]
pub trait RequestJson<T>: Send + Sync
where
    T: Serialize + Send + Sync + 'static,
{
    /// Loads an Aggregate Root instance from the data store,
    /// referenced by its unique identifier.
    async fn request(
        &self,
        subject: String,
        msg: T,
    ) -> Result<async_nats::Message, NatsTransportError>;
}
