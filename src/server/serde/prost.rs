use std::marker::PhantomData;

use prost::{bytes::Bytes, Message};

use crate::server::serde::{Deserializer, Serializer};

#[derive(Debug, Clone, Copy, Default)]
pub struct NatsMessageSerde<T>(PhantomData<T>)
where
    T: Message;

impl<T> Serializer<T> for NatsMessageSerde<T>
where
    T: Message,
{
    fn serialize(&self, value: T) -> Bytes {
        value.encode_to_vec().into()
    }
}

impl<T> Deserializer<T> for NatsMessageSerde<T>
where
    T: Message + Default,
{
    type Error = prost::DecodeError;

    fn deserialize(&self, data: Bytes) -> Result<T, Self::Error> {
        //let buf = Bytes::from(data);

        T::decode(data)
    }
}

#[cfg(test)]
#[path = "./prost_tests.rs"]
mod prost_tests;
