use std::{fs, sync::Arc};
use tracing::error;

use anything_core::{
    error::{AnythingError, AnythingResult, DatabaseError},
    AnythingConfig,
};
use sqlx::{any::AnyPoolOptions, AnyPool};

use crate::Storage;

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pub config: Arc<AnythingConfig>,
    conn: Option<Arc<AnyPool>>,
}

#[async_trait::async_trait]
impl Storage for SqliteDatabase {
    fn new(config: &AnythingConfig) -> Self {
        Self {
            config: Arc::new(config.clone()),
            conn: None,
        }
    }

    async fn init(&mut self) -> AnythingResult<&Self> {
        let db_path = self.config.database_path();
        let db_dir = db_path.parent().unwrap();

        // If the parent directory does not exist, create it.
        if !db_dir.exists() {
            fs::create_dir_all(db_dir).unwrap();
        }

        if !db_path.exists() {
            // Create the database file.
            fs::File::create(db_path.clone()).unwrap();
        }

        // Create the database connection.
        let db_path = self.config.database_path();
        // let uri = db_path.into_os_string().into_string().unwrap();
        let uri = format!("sqlite:{}", db_path.to_str().unwrap());
        let conn = match AnyPoolOptions::new().connect(&uri).await {
            Ok(conn) => conn,
            Err(err) => {
                error!("unable to create database connection: {}", err);
                return Err(AnythingError::DB(DatabaseError::NotAvailable));
            }
        };

        self.conn = Some(Arc::new(conn));

        Ok(self)
    }

    async fn get_connection(&self) -> AnythingResult<&sqlx::AnyPool> {
        // Should be safe to unwrap here
        match self.conn {
            Some(ref conn) => return Ok(&conn),
            None => Err(AnythingError::DB(DatabaseError::NotAvailable)),
        }
    }

    async fn available(&self) -> bool {
        self.config.database_path().exists()
    }
}
