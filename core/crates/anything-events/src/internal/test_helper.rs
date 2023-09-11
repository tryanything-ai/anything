#![allow(unused)]
use crate::{
    config::AnythingEventsConfig,
    context::Context,
    errors::EventsResult,
    models::{
        event::{CreateEvent, Event, EventId},
        tag::Tag,
    },
    post_office::PostOffice,
    repositories::{self, event_repo::EventRepoImpl, Repositories},
};
use chrono::Utc;
use crossbeam::channel::Sender;
use fake::Fake;
use serde_json::Value;
use sqlx::{sqlite::SqlitePoolOptions, Pool, SqlitePool};
use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};

pub fn get_test_config() -> AnythingEventsConfig {
    let mut config = AnythingEventsConfig::default();
    config.database.uri = "sqlite::memory:".to_string();
    config
}

pub async fn get_test_context() -> Context {
    let config = get_test_config();
    let pool = get_test_pool().await.unwrap();
    let event_repo = TestEventRepo::new_with_pool(&pool);
    let repositories = Arc::new(Repositories {
        event_repo: event_repo.event_repo,
    });
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
    }
}

pub async fn get_test_context_from_pool(pool: &SqlitePool) -> Context {
    let config = get_test_config();
    // let pool = Arc::new(get_test_pool().await.unwrap());
    let event_repo = TestEventRepo::new_with_pool(pool);
    let repositories = Arc::new(Repositories {
        event_repo: event_repo.event_repo,
    });
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
    }
}

pub async fn get_test_pool() -> EventsResult<Pool<sqlx::Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await?;

    let res = sqlx::query(include_str!("../../sql/schema.sql"))
        .execute(&pool)
        .await?;

    Ok(pool)
}

pub async fn select_all_events(pool: &SqlitePool) -> EventsResult<Vec<Event>> {
    let query = sqlx::query_as::<_, Event>(r#"SELECT * FROM events"#);

    let result = query.fetch_all(pool).await?;

    Ok(result)
}

pub async fn insert_new_event(pool: &SqlitePool, event: Event) -> EventsResult<i64> {
    let res = sqlx::query(
        r#"INSERT INTO events (source_id, event_name, payload, metadata) VALUES (?, ?, ?, ?)"#,
    )
    .bind(event.source_id)
    .bind(event.event_name)
    .bind(event.payload)
    .bind(event.metadata)
    // .bind(event.tags)
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

// pub async fn insert_new_tag(pool: &SqlitePool, tag: Tag) -> EventsResult<i64> {
//     let res = sqlx::query("INSERT INTO tags (name) VALUES (?)")
//         .bind(tag_name)
//         .execute(pool)
//         .await?;

//     let row_id = res.last_insert_rowid();

//     Ok(row_id)
// }

pub async fn insert_n_dummy_data(pool: &SqlitePool, count: i8) -> EventsResult<()> {
    for _i in 0..count {
        insert_dummy_data(pool).await?;
    }
    Ok(())
}

pub async fn insert_dummy_data(pool: &SqlitePool) -> EventsResult<EventId> {
    let payload = generate_dummy_hashmap();
    let metadata = generate_dummy_hashmap();
    // let tag_words: Vec<String> = fake::faker::lorem::en::Words(3..5).fake();
    // let tags = tag_words
    //     .into_iter()
    //     .map(|f| f.into())
    //     .collect::<Vec<Tag>>();

    let fake_event = Event {
        id: i64::default(),
        source_id: i64::default(),
        event_name: fake::faker::name::en::Name().fake(),
        // tags: TagList::default(),
        payload: Value::default(),
        metadata: Value::default(),
        timestamp: Utc::now(),
    };

    let _res = insert_new_event(pool, fake_event.clone()).await?;

    Ok(fake_event.id)
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

#[derive(Debug)]
pub struct TestEventRepo {
    pub pool: SqlitePool,
    pub event_repo: EventRepoImpl,
    pub post_office: PostOffice,
}

impl TestEventRepo {
    pub async fn new() -> Self {
        let pool = get_test_pool().await.expect("unable to get test pool");
        Self::new_with_pool(&pool)
    }

    pub fn new_with_pool(pool: &SqlitePool) -> Self {
        let event_repo = EventRepoImpl::new(&pool);
        Self {
            pool: pool.clone(),
            event_repo,
            post_office: PostOffice::open(),
        }
    }

    pub async fn with_sender(&self) -> Sender<Event> {
        self.post_office.post_mail().await.unwrap()
    }

    // pub async fn new_from_context(context: Context) -> Self {
    //     let pool = (&*context.pool).clone();
    //     let event_repo = EventRepoImpl::new(&pool);
    //     let context = get_test_context().await;
    //     Self {
    //         pool,
    //         event_repo,
    //         context,
    //     }
    // }

    pub fn dummy_create_event(&self) -> CreateEvent {
        CreateEvent {
            source_id: i64::default(),
            event_name: fake::faker::name::en::Name().fake(),
            payload: Value::default(),
            metadata: Value::default(),
        }
    }

    pub async fn insert_dummy_event(&self) -> EventsResult<Event> {
        let mut event = Event {
            id: i64::default(),
            event_name: fake::faker::name::en::Name().fake(),
            source_id: i64::default(),
            payload: Value::default(),
            metadata: Value::default(),
            timestamp: Utc::now(),
        };

        let res = sqlx::query(
            r#"INSERT INTO events 
            (source_id, event_name, payload, metadata)
            VALUES (?1, ?2, ?3, ?4)"#,
        )
        .bind(&event.source_id)
        .bind(&event.event_name)
        .bind(&event.payload)
        .bind(&event.metadata)
        // .bind(event.tags)
        .execute(&self.pool)
        .await
        .expect("unable to insert dummy data");

        event.id = res.last_insert_rowid();
        Ok(event)
    }
}
