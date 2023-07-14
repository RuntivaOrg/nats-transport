use std::marker::PhantomData;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::server::serde::{Deserializer, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct NatsJson<T>(PhantomData<T>);

impl<T> Default for NatsJson<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Serializer<T> for NatsJson<T>
where
    T: Serialize,
{
    fn serialize(&self, value: T) -> Bytes {
        serde_json::to_vec(&value)
            .expect("json serialization should not fail")
            .into()
    }
}

impl<T> Deserializer<T> for NatsJson<T>
where
    for<'d> T: Deserialize<'d>,
{
    type Error = serde_json::Error;

    fn deserialize(&self, data: Bytes /*Vec<u8>*/) -> Result<T, Self::Error> {
        serde_json::from_slice(&data[..])
    }
}

#[cfg(test)]
#[path = "./json_tests.rs"]
mod json_tests;
