use crate::{config::AnythingEventsConfig, errors::EventsResult, store::store::Store};
use fake::Fake;
use sqlx::{sqlite::SqlitePoolOptions, Pool, SqlitePool};
use std::collections::HashMap;

use crate::models::Event;

pub fn get_test_config() -> AnythingEventsConfig {
    let mut config = AnythingEventsConfig::default();
    config.database.uri = "sqlite::memory:".to_string();
    config
}

pub async fn get_pool() -> EventsResult<Pool<sqlx::Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    Ok(pool)
}

pub async fn get_sqlite_store_adapter() -> EventsResult<Store> {
    // let store = SqliteStoreAdapter::new(pool);
    let store = Store::from_config(&get_test_config()).await?;
    store.init().await?;
    Ok(store)
}

pub async fn select_all_events(pool: &SqlitePool) -> EventsResult<Vec<Event>> {
    let res = sqlx::query_as::<_, Event>(
        r#"SELECT id, event_name, payload, metadata, tags, timestamp FROM events"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(res)
}

pub async fn insert_new_event(pool: &SqlitePool, event: Event) -> EventsResult<i64> {
    let res = sqlx::query(
        r#"INSERT INTO events (event_name, payload, metadata, tags) VALUES (?, ?, ?, ?)"#,
    )
    .bind(event.event_name)
    .bind(event.payload)
    .bind(event.metadata)
    .bind(event.tags)
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

pub async fn insert_n_dummy_data(pool: &SqlitePool, count: i8) -> EventsResult<()> {
    for _i in 0..count {
        insert_dummy_data(pool).await?;
    }
    Ok(())
}

pub async fn insert_dummy_data(pool: &SqlitePool) -> EventsResult<()> {
    let payload = generate_dummy_hashmap();
    let metadata = generate_dummy_hashmap();
    let tags = fake::faker::lorem::en::Words(3..5).fake();

    let fake_event = Event::new(
        fake::faker::name::en::Name().fake(),
        serde_json::json!(payload),
        tags,
    );
    let fake_event = fake_event.with_metadata(metadata);

    let _res = insert_new_event(pool, fake_event).await?;

    Ok(())
}

fn generate_dummy_hashmap() -> HashMap<String, String> {
    let mut payload: HashMap<String, String> = HashMap::default();

    for _ in 1..4 {
        let key = fake::faker::name::en::Name().fake();
        let value = fake::faker::name::en::Name().fake();
        payload.insert(key, value);
    }
    payload
}
