use std::{
    fmt::{self},
    marker::PhantomData,
    str::FromStr,
};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{EngineKind, RuntimeError};

pub type DeserializeValue = toml::Value;

pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}

pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = RuntimeError>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = RuntimeError>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub fn option_string_or_struct<'de, D>(deserializer: D) -> Result<Option<EngineKind>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrStruct {
        Str(String),
        Table(InterpreterKindTable),
        Struct(EngineKind),
    }

    #[derive(Deserialize)]
    struct InterpreterKindTable {
        #[serde(alias = "runtime")]
        #[serde(alias = "engine")]
        interpreter: String,
        args: Option<Vec<String>>,
        options: Option<indexmap::IndexMap<String, String>>,
    }

    let result = StringOrStruct::deserialize(deserializer);

    match result {
        Ok(StringOrStruct::Str(s)) => {
            let ik = EngineKind::from_str(&s).unwrap();
            Ok(Some(ik.into()))
        }
        Ok(StringOrStruct::Struct(s)) => Ok(Some(s)),
        Ok(StringOrStruct::Table(t)) => {
            let res = EngineKind::from_str(&t.interpreter);
            match res {
                Ok(ik) => {
                    let ik = match ik {
                        EngineKind::PluginEngine(mut pi) => {
                            pi.args = match t.args {
                                Some(args) => Some(args),
                                None => None,
                            };
                            if let Some(options) = t.options {
                                pi.options = options
                                    .into_iter()
                                    .map(|(k, v)| (k, DeserializeValue::from(v).into()))
                                    .collect();
                            }
                            EngineKind::PluginEngine(pi)
                        }
                        EngineKind::Internal(mut si) => {
                            if let Some(args) = t.args {
                                si.args = args;
                            }
                            EngineKind::Internal(si)
                        }
                    };
                    Ok(Some(ik))
                }
                Err(err) => {
                    tracing::error!("Error occurred while deserializing engine: {:?}", err);
                    Ok(Some(EngineKind::default()))
                }
            }
        }
        Err(err) => {
            tracing::error!("Error occurred while deserializing engine: {:?}", err);
            Ok(Some(EngineKind::default()))
        }
    }
}
