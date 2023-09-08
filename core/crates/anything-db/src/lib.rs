use anything_core::{config::AnythingConfig, error::AnythingResult};

pub mod sqlite;

#[async_trait::async_trait]
pub trait Storage: Send + Sync + Sized + Clone {
    fn new(config: &AnythingConfig) -> Self;
    async fn init(&mut self) -> AnythingResult<&Self>;
    async fn get_connection(&self) -> AnythingResult<&sqlx::AnyPool>;
    async fn available(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct Database<T> {
    engine: Box<T>,
}

impl<T> Database<T>
where
    T: Storage + Send + Sync + Sized + Clone,
{
    pub fn new(config: &AnythingConfig) -> Self {
        sqlx::any::install_default_drivers();
        let engine = Box::new(T::new(config));

        Self { engine }
    }

    pub fn get_database(&self) -> &T {
        &self.engine
    }

    pub async fn conn(&self) -> AnythingResult<&sqlx::AnyPool> {
        self.engine.get_connection().await
    }

    pub async fn available(&self) -> bool {
        self.engine.available().await
    }

    pub async fn init(&mut self) -> AnythingResult<&Self> {
        self.engine.init().await?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use std::env::{self};
    use temp_dir::TempDir;

    use super::*;

    #[allow(unused)]
    #[derive(sqlx::FromRow, Debug)]
    struct TestModel {
        pub id: i64,
        pub name: String,
    }

    #[tokio::test]
    async fn test_init_creates_db() {
        let db: Database<sqlite::SqliteDatabase> = Database::new(&test_config());
        assert_eq!(db.available().await, false);
    }

    #[tokio::test]
    async fn test_init_creates_db_dir() {
        let mut db: Database<sqlite::SqliteDatabase> = Database::new(&test_config());
        let _ = db.init().await;
        assert_eq!(db.available().await, true);
    }

    #[tokio::test]
    async fn test_execute_sql() {
        let mut storage: Database<sqlite::SqliteDatabase> = Database::new(&test_config());
        let uninitialized_pool = storage.conn().await;
        assert!(uninitialized_pool.is_err());

        let storage = storage.init().await;
        let storage = storage.unwrap();
        let pool = storage.conn().await;
        assert!(pool.is_ok());
        let pool = pool.unwrap();

        // let pool = conn.acquire().await.unwrap();
        // let res = sqlx::query(sql).execute(pool).await;

        sqlx::query("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);").execute(pool).await.unwrap();
        sqlx::query("INSERT INTO test (name) VALUES (?)")
            .bind("test")
            .execute(pool)
            .await
            .unwrap();

        let rows = sqlx::query_as::<_, TestModel>("SELECT * FROM test")
            .fetch_all(pool)
            .await;
        assert!(rows.is_ok());
        let rows = rows.unwrap();
        assert!(rows.len() > 0);
        assert_eq!(rows.get(0).unwrap().name, "test");
    }

    pub fn test_config() -> AnythingConfig {
        env::set_var("RUN_MODE", "test");
        let mut config = AnythingConfig::new().unwrap();
        let tmp_dir = TempDir::new().unwrap().path().to_path_buf();
        config.root_dir = Some(tmp_dir.into());
        config.db = None;
        config
    }
}
