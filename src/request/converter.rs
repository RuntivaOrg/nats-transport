use std::fmt::Debug;
use std::marker::PhantomData;

use bytes::Bytes;

use crate::server::{
    serde::{Deserializer, Serde},
    NatsTransportError,
};

use super::{nats_request::TryFromNatsRequest, NatsEnvelope};

pub struct Converter<Msg, OutMsg, S>
where
    Msg: TryFromNatsRequest<OutMsg> + Debug,
    OutMsg: prost::Message,
    S: Serde<OutMsg>,
{
    serde: S,
    msg_type: PhantomData<Msg>,
    out_msg_type: PhantomData<OutMsg>,
}

impl<Msg, OutMsg, S> Converter<Msg, OutMsg, S>
where
    Msg: TryFromNatsRequest<OutMsg> + Debug,
    <Msg as TryFromNatsRequest<OutMsg>>::Error: std::error::Error + Send + Sync + 'static,
    OutMsg: prost::Message,
    S: Serde<OutMsg>,
    <S as Deserializer<OutMsg>>::Error: std::error::Error + Send + Sync + 'static,
{
    pub fn new(serde: S) -> Self {
        Self {
            serde,
            msg_type: PhantomData,
            out_msg_type: PhantomData,
        }
    }

    pub fn convert(&self, msg: Bytes) -> Result<NatsEnvelope<Msg>, NatsTransportError> {
        let out_msg = self
            .serde
            .deserialize(msg)
            .map_err(|err| NatsTransportError::DeserializeEvent(Box::new(err)))?;

        let (msg, headers) = Msg::try_from(out_msg)
            .map_err(|err| NatsTransportError::ConvertEvent(Box::new(err)))?;

        let nats_envelope = NatsEnvelope::new(headers, msg);

        Ok(nats_envelope)
    }
}

#[cfg(test)]
#[path = "./converter_tests.rs"]
mod converter_tests;
