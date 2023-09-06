#![allow(unused)]
use std::collections::HashMap;

use serde_json::Value;

pub mod vectorize {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::iter::FromIterator;

    pub fn serialize<'a, T, K, V, S>(target: T, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: IntoIterator<Item = (&'a K, &'a V)>,
        K: Serialize + 'a,
        V: Serialize + 'a,
    {
        let container: Vec<_> = target.into_iter().collect();
        serde::Serialize::serialize(&container, ser)
    }

    pub fn deserialize<'de, T, K, V, D>(des: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromIterator<(K, V)>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        let container: Vec<_> = serde::Deserialize::deserialize(des)?;
        Ok(T::from_iter(container.into_iter()))
    }
}

pub fn json_to_hashmap(
    json: &str,
    keys: Vec<&str>,
) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).unwrap();
    let mut map = HashMap::new();
    for key in keys {
        let (k, v) = lookup.remove_entry(key).unwrap();
        map.insert(k, v);
    }
    Ok(map)
}
