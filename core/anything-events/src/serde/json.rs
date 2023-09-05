use super::{Deserializer, Serializer};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy)]
pub struct Json<T>(std::marker::PhantomData<T>);

impl<T> Default for Json<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T> Serializer<T> for Json<T>
where
    T: Serialize,
{
    fn serialize(&self, value: T) -> Vec<u8> {
        serde_json::to_vec(&value).expect("json serialization failed")
    }
}

impl<T> Deserializer<T> for Json<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn deserialize(&self, bytes: Vec<u8>) -> Result<T, super::Error> {
        serde_json::from_slice(&bytes).map_err(|e| super::Error::DeserializeError(Box::new(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
    struct TestEvent {
        pub id: i32,
        pub name: String,
    }

    #[test]
    fn it_serializes_json_data() {
        let json_serializer = Json::<TestEvent>::default();
        let event = TestEvent {
            id: 1,
            name: "test".to_string(),
        };

        let serialized_data = json_serializer.serialize(event.clone());
        let deserialized_data = json_serializer.deserialize(serialized_data).unwrap();

        assert_eq!(event, deserialized_data);
    }
}
