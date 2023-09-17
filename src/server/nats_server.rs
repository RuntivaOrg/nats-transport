use async_nats::Client;
use async_trait::async_trait;
use bytes::Bytes;
use prost::Message;
use serde::Serialize;

use crate::server::{
    serde::{NatsJson, NatsMessageSerde, Serializer},
    server_traits::{RequestJson, RequestProst},
    NatsTransportError, PublishJson, PublishProst,
};

pub struct NatsServer {
    nats: Client,
}

impl NatsServer {
    /// initializes the NATS client connection
    pub async fn initialize(nats_url: &str) -> Result<NatsServer, NatsTransportError> {
        let client = async_nats::connect(nats_url).await?;
        Ok(NatsServer { nats: client })
    }

    pub fn client(&self) -> &Client {
        &self.nats
    }

    // TODO: update payload_json to msg: NatsMsg<T>
    pub async fn push_msg(
        &self,
        subject: String,
        payload_json: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        //tracing::info!("pushing msg: {} / {}", subject, payload_json);

        self.nats.publish(subject, payload_json.into()).await?;

        Ok(())
    }

    async fn internal_publish(
        &self,
        subject: String,
        message: Bytes,
    ) -> Result<(), NatsTransportError> {
        self.nats
            .publish(subject, message)
            .await
            .map_err(NatsTransportError::NatsPublishError)
    }

    async fn internal_request(
        &self,
        subject: String,
        message: Bytes,
    ) -> Result<async_nats::Message, NatsTransportError> {
        self.nats
            .request(subject, message)
            .await
            .map_err(NatsTransportError::NatsRequestError)
    }
}

#[async_trait]
impl<T> PublishProst<T> for NatsServer
where
    Self: Send + Sync,
    T: Message + Default + 'static,
{
    async fn publish(&self, subject: String, msg: T) -> Result<(), NatsTransportError> {
        let serde = NatsMessageSerde::<T>::default();
        let serialized_msg = serde.serialize(msg);
        self.internal_publish(subject, serialized_msg).await
    }
}

#[async_trait]
impl<T> PublishJson<T> for NatsServer
where
    Self: Send + Sync,
    T: Serialize + Send + Sync + 'static,
{
    async fn publish(&self, subject: String, msg: T) -> Result<(), NatsTransportError> {
        let serde = NatsJson::<T>::default();
        let serialized_msg = serde.serialize(msg);
        self.internal_publish(subject, serialized_msg).await
    }
}

#[async_trait]
impl<T> RequestProst<T> for NatsServer
where
    Self: Send + Sync,
    T: Message + Send + Sync + Default + 'static,
{
    async fn request(
        &self,
        subject: String,
        msg: T,
    ) -> Result<async_nats::Message, NatsTransportError> {
        let serde = NatsMessageSerde::<T>::default();
        let serialized_msg = serde.serialize(msg);
        self.internal_request(subject, serialized_msg).await
    }
}

#[async_trait]
impl<T> RequestJson<T> for NatsServer
where
    Self: Send + Sync,
    T: Serialize + Send + Sync + 'static,
{
    async fn request(
        &self,
        subject: String,
        msg: T,
    ) -> Result<async_nats::Message, NatsTransportError> {
        let serde = NatsJson::<T>::default();
        let serialized_msg = serde.serialize(msg);
        self.internal_request(subject, serialized_msg).await
    }
}

#[cfg(test)]
#[path = "./nats_server_tests.rs"]
mod nats_server_tests;
