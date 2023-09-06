use std::fmt::Display;
use std::marker::PhantomData;
use std::str;

use serde_json::Value as JsonValue;
use sqlx::{database::HasArguments, encode::IsNull, Type};
use sqlx::{Decode, Encode};

use super::ident::Identifier;

pub struct StreamQuery<E: Clone> {
    pub filter: Option<StreamFilter>,
    pub tags: Option<Vec<String>>,
    event_type: PhantomData<E>,
}

impl<E: Clone> StreamQuery<E> {
    pub fn filter(&self) -> Option<&StreamFilter> {
        self.filter.as_ref()
    }

    pub fn tags(&self) -> Option<&Vec<String>> {
        self.tags.as_ref()
    }
}

pub enum StreamFilter {
    Events {
        names: &'static [&'static str],
    },
    // TODO: figure out why String doesn't implement Encode
    // Eq {
    //     ident: Identifier,
    //     value: Text,
    // },
    Tags {
        tags: Vec<Text>,
    },
    And {
        left: Box<StreamFilter>,
        right: Box<StreamFilter>,
    },
    Or {
        left: Box<StreamFilter>,
        right: Box<StreamFilter>,
    },
}

/// Represents a filter evaluator used to evaluate stream filters.
pub trait FilterEvaluator {
    /// The result type produced by evaluating a filter.
    type Result;
    /// Evaluates the given filter and returns the result.
    fn eval(&mut self, filter: &StreamFilter) -> Self::Result;
}

#[derive(Debug, Clone, sqlx::Decode)]
pub struct Text(String);

impl Text {
    pub fn string(self) -> String {
        self.0
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Text> for String {
    fn into(self) -> Text {
        Text(self)
    }
}
impl From<&String> for Text {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<&[u8]> for Text {
    fn from(value: &[u8]) -> Self {
        let s = match str::from_utf8(value) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Self(s.to_string())
    }
}

impl From<&u64> for Text {
    fn from(value: &u64) -> Self {
        Self(format!("{}", value.clone()))
    }
}

impl From<JsonValue> for Text {
    fn from(value: JsonValue) -> Self {
        Self(value.to_string())
    }
}

#[cfg(feature = "sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Text {
    fn encode_by_ref(
        &self,
        args: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        args.push(sqlx::sqlite::SqliteArgumentValue::Text(
            std::borrow::Cow::from(self.0.clone()),
        ));
        sqlx::encode::IsNull::No
    }
}

// Encode<'args, DB> + Send + Type<DB>
#[cfg(feature = "sqlite")]
impl<'q> sqlx::Type<sqlx::Sqlite> for Text {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        use sqlx::TypeInfo;

        match ty.name() {
            "String" => true,
            _ => <String as sqlx::Type<sqlx::Sqlite>>::compatible(ty),
        }
    }
}

// stream_query::Text: Encode<'_, D>`

// impl<'q> Encode<'q, sqlx::Sqlite> for Text {
//     fn encode_by_ref(&self, args: &mut Vec<sqlx::sqlite::SqliteArgumentValue<'q>>) -> IsNull {
//         let bit_cast = self.0.as_bytes();
//         bit_cast.encode_by_ref(bit_cast)
//     }
// }

// impl<'r> sqlx::Encode<'r, sqlx::Sqlite> for Text {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <sqlx::Sqlite as HasArguments<'r>>::ArgumentBuffer,
//     ) -> IsNull {
//         // TODO: What am I really supposed to do here?
//         match buf.write(self.0.as_bytes()) {
//             Ok(_) => IsNull::No,
//             Err(_) => IsNull::Yes,
//         }
//     }
// }
