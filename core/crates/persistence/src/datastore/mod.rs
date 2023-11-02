pub mod connections;
pub(crate) mod sqlite;
pub mod types;

pub use types::*;

pub use connections::create_sqlite_datastore_from_config_and_file_store;

#[cfg(feature = "sqlite")]
pub(crate) use sqlite::SqliteDatastore as Datastore;

#[cfg(feature = "postgres")]
pub use postgres::PostgresDatastore as Datastore;
