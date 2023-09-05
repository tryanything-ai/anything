pub mod json;
pub mod protobuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("deserialization error: {0}")]
    DeserializeError(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("conversion error")]
    ConversionError,
}

pub trait Serializer<T> {
    /// Serialize a value of type `T` into a byte vector.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to serialize.
    ///
    /// # Returns
    ///
    /// A byte vector containing the serialized value.
    fn serialize(&self, value: T) -> Vec<u8>;
}

pub trait Deserializer<T> {
    /// Deserialize a byte vector into a value of type `T`.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte vector to deserialize.
    ///
    /// # Returns
    ///
    /// A value of type `T`.
    fn deserialize(&self, bytes: Vec<u8>) -> Result<T, Error>;
}

pub trait Serde<T>: Serializer<T> + Deserializer<T> {}

impl<K, T> Serde<T> for K where K: Serializer<T> + Deserializer<T> {}
