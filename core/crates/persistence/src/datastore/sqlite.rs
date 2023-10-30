use sqlx::{Connection, Database, SqliteConnection};

pub struct SqliteDatastore {
    connection: SqliteConnection,
}

impl SqliteDatastore {
    pub async fn new(db_path: Option<String>) -> Result<Self, sqlx::Error> {
        let db_path = match db_path {
            Some(path) => path,
            None => String::from("./data.db"),
        };

        let connection = sqlx::sqlite::SqliteConnection::connect(&db_path).await?;
        Ok(Self { connection })
    }
}
