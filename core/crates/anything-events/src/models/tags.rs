use serde::{Deserialize, Serialize};
use sqlx::{Decode, Encode};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Tags(pub Vec<String>);

impl Tags {}

struct IterWrapper<'a> {
    // this needs to own the iterator, not a reference to it
    // in order to avoid returning a borrowed value
    inner: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for IterWrapper<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        // no semicolon here so the result is implicitly returned
        // your old error happened because the semicolon causes the value to not be returned
        self.inner.next()
    }
}

impl IntoIterator for Tags {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<DB: sqlx::Database> sqlx::Type<DB> for Tags
where
    String: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <String as sqlx::Type<DB>>::type_info()
    }
}

impl<'r, DB: sqlx::Database> sqlx::Decode<'r, DB> for Tags
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

impl<'q, DB: sqlx::Database> Encode<'q, DB> for Tags
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
