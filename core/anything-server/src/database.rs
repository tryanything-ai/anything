use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn create_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("SUPABASE_URL").map(|url| format!("{}/rest/v1", url)))
        .expect("DATABASE_URL or SUPABASE_URL must be set");
    
    Database::connect(&database_url).await
}

pub async fn create_connection_with_url(url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(url).await
}
