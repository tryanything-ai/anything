use std::marker::PhantomData;

use protobuf::Message;

use super::{Deserializer, Error, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct Protobuf<I, O>(PhantomData<I>, PhantomData<O>)
where
    O: Message;

impl<I, O> Default for Protobuf<I, O>
where
    O: Message,
{
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<I, O> Serializer<I> for Protobuf<I, O>
where
    O: From<I> + Message,
{
    fn serialize(&self, value: I) -> Vec<u8> {
        let target = O::from(value);
        target
            .write_to_bytes()
            .expect("protobuf serialization failed")
    }
}

impl<I, O> Deserializer<I> for Protobuf<I, O>
where
    I: TryFrom<O>,
    O: Message,
{
    fn deserialize(&self, data: Vec<u8>) -> Result<I, Error> {
        let target =
            O::parse_from_bytes(&data).map_err(|e| Error::DeserializeError(Box::new(e)))?;
        I::try_from(target).map_err(|_| Error::ConversionError)
    }
}
