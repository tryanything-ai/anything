use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};

pub type TagId = i64;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Default, Clone)]
#[serde(transparent)]
pub struct Tag(String);

impl From<String> for Tag {
    fn from(value: String) -> Self {
        Self(value)
    }
}

// impl sqlx::Type<sqlx::Sqlite> for Tag {
//     fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
//         <String as sqlx::Type<sqlx::Sqlite>>::type_info()
//     }
// }

#[derive(Serialize, Deserialize, sqlx::FromRow, Default, Debug, Clone)]
pub struct TagList(Vec<Tag>);

// struct IterWrapper<'a> {
//     // this needs to own the iterator, not a reference to it
//     // in order to avoid returning a borrowed value
//     inner: std::slice::Iter<'a, String>,
// }

// impl<'a> Iterator for IterWrapper<'a> {
//     type Item = &'a String;

//     fn next(&mut self) -> Option<Self::Item> {
//         // no semicolon here so the result is implicitly returned
//         // your old error happened because the semicolon causes the value to not be returned
//         self.inner.next()
//     }
// }

impl<DB: sqlx::Database> sqlx::Type<DB> for Tag
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for Tag
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value).unwrap_or_default())
    }
}

impl<'q, DB: sqlx::Database> Encode<'q, DB> for Tag
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let val = serde_json::to_string(self).unwrap_or_default();
        <String as Encode<DB>>::encode(val, buf)
    }
}

// TagList
impl<DB: sqlx::Database> sqlx::Type<DB> for TagList
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for TagList
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value).unwrap_or_default())
    }
}

impl<'q, DB: sqlx::Database> Encode<'q, DB> for TagList
where
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let val = serde_json::to_string(self).unwrap_or_default();
        <String as Encode<DB>>::encode(val, buf)
    }
}
