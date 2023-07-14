mod json;
mod prost;

pub use self::prost::NatsMessageSerde;
use bytes::Bytes;
pub use json::NatsJson;

pub trait Serializer<T> {
    fn serialize(&self, value: T) -> Bytes;
}

impl<T, F> Serializer<T> for F
where
    F: Fn(T) -> Bytes,
{
    fn serialize(&self, value: T) -> Bytes {
        self(value)
    }
}

pub trait Deserializer<T> {
    type Error;

    fn deserialize(&self, data: Bytes) -> Result<T, Self::Error>;
}

impl<T, Err, F> Deserializer<T> for F
where
    F: Fn(Bytes) -> Result<T, Err>,
{
    type Error = Err;

    fn deserialize(&self, data: Bytes) -> Result<T, Self::Error> {
        self(data)
    }
}

pub trait Serde<T>: Serializer<T> + Deserializer<T> {}

impl<K, T> Serde<T> for K where K: Serializer<T> + Deserializer<T> {}
