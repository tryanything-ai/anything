use std::marker::PhantomData;
use std::str;

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

pub struct Text(String);

impl Text {
    pub fn string(self) -> String {
        self.0
    }
}

// std::string::String` to implement `Into<stream_query::Text>
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

#[cfg(feature = "sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for Text {
    fn encode_by_ref(
        &self,
        args: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        // args.push(sqlx::sqlite::SqliteArgumentValue::Text(self.0));
        // args.push(self.0);
        sqlx::encode::IsNull::No
    }
}

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
