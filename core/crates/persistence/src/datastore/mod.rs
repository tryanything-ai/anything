pub mod connections;
pub(crate) mod sqlite;
pub(crate) mod types;

pub(crate) use types::*;

#[cfg(feature = "sqlite")]
pub(crate) use sqlite::SqliteDatastore as Datastore;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatastore as Datastore;
