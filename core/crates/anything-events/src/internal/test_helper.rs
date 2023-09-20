#![allow(unused)]

use anything_core::posix::copy_recursively;
use anything_engine::context;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use crate::models::system_handler;
use crate::models::trigger::CreateTrigger;
use crate::repositories::flow_repo::{self, FlowRepoImpl};
use crate::repositories::trigger_repo::TriggerRepoImpl;
use crate::Server;
use crate::{
    config::AnythingEventsConfig,
    context::Context,
    errors::{DatabaseError, EventsError, EventsResult},
    models::{
        event::{CreateEvent, Event, EventId, SourceId},
        tag::Tag,
    },
    post_office::PostOffice,
    repositories::{self, event_repo::EventRepoImpl, Repositories},
};
use chrono::Utc;
use fake::Fake;
use postage::{
    dispatch::{Receiver, Sender},
    prelude::*,
};
use serde_json::Value;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Row, SqlitePool};
use std::{borrow::BorrowMut, collections::HashMap, panic, sync::Arc};

pub fn get_test_config() -> AnythingEventsConfig {
    let mut config = AnythingEventsConfig::default();
    config.database.uri = "sqlite::memory:".to_string();
    config.server.port = 0;
    config
}

pub async fn get_test_context() -> Context {
    let config = get_test_config();
    let pool = get_test_pool().await.expect("unable to get test pool");
    let event_repo = TestEventRepo::new_with_pool(&pool);
    let flow_repo = TestFlowRepo::new_with_pool(&pool);
    let trigger_repo = TestTriggerRepo::new_with_pool(&pool);
    let repositories = Arc::new(Repositories {
        event_repo: event_repo.event_repo,
        flow_repo: flow_repo.flow_repo,
        trigger_repo: trigger_repo.trigger_repo,
    });
    let system_handler = Arc::new(system_handler::SystemHandler::new(config.clone()));
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        system_handler,
    }
}

pub async fn get_test_context_with_config(config: AnythingEventsConfig) -> Context {
    // let config = get_test_config();
    let pool = get_test_pool().await.unwrap();
    let event_repo = TestEventRepo::new_with_pool(&pool);
    let flow_repo = TestFlowRepo::new_with_pool(&pool);
    let trigger_repo = TestTriggerRepo::new_with_pool(&pool);
    let repositories = Arc::new(Repositories {
        event_repo: event_repo.event_repo,
        flow_repo: flow_repo.flow_repo,
        trigger_repo: trigger_repo.trigger_repo,
    });
    let system_handler = Arc::new(system_handler::SystemHandler::new(config.clone()));
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        system_handler,
    }
}

pub async fn get_test_context_from_pool(pool: &SqlitePool) -> Context {
    let config = get_test_config();
    // let pool = Arc::new(get_test_pool().await.unwrap());
    let event_repo = TestEventRepo::new_with_pool(pool);
    let flow_repo = TestFlowRepo::new_with_pool(&pool);
    let trigger_repo = TestTriggerRepo::new_with_pool(&pool);
    let repositories = Arc::new(Repositories {
        event_repo: event_repo.event_repo,
        flow_repo: flow_repo.flow_repo,
        trigger_repo: trigger_repo.trigger_repo,
    });
    let system_handler = Arc::new(system_handler::SystemHandler::new(config.clone()));
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        system_handler,
    }
}

pub async fn get_test_pool() -> EventsResult<Pool<sqlx::Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|e| {
            EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
        })?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| EventsError::DatabaseError(DatabaseError::DBError(Box::new(e))))?;

    Ok(pool)
}

pub async fn get_test_server() -> EventsResult<Arc<Server>> {
    let context = get_test_context().await;
    let server = Server::new(context).await?;
    Ok(server)
}

pub async fn select_all_events(pool: &SqlitePool) -> EventsResult<Vec<Event>> {
    let query = sqlx::query_as::<_, Event>(r#"SELECT * FROM events"#);

    let result = query.fetch_all(pool).await.map_err(|e| {
        EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e)))
    })?;

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
    .await
    .map_err(|e| EventsError::DatabaseError(crate::errors::DatabaseError::DBError(Box::new(e))))?;

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
        event_id: uuid::Uuid::new_v4().to_string(),
        source_id: String::default(),
        event_name: fake::faker::name::en::Name().fake(),
        event_type: String::default(),
        // tags: TagList::default(),
        payload: Value::default(),
        metadata: Value::default(),
        timestamp: Utc::now(),
    };

    let _res = insert_new_event(pool, fake_event.clone()).await?;

    Ok(fake_event.event_id)
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
pub struct TestFlowRepo {
    pub pool: SqlitePool,
    pub flow_repo: FlowRepoImpl,
    pub post_office: PostOffice,
}

impl TestFlowRepo {
    pub async fn new() -> Self {
        let pool = get_test_pool().await.expect("unable to get test pool");
        Self::new_with_pool(&pool)
    }

    pub fn new_with_pool(pool: &SqlitePool) -> Self {
        let flow_repo = FlowRepoImpl::new(&pool);
        Self {
            pool: pool.clone(),
            flow_repo,
            post_office: PostOffice::open(),
        }
    }
}

#[derive(Debug)]
pub struct TestTriggerRepo {
    pub pool: SqlitePool,
    pub trigger_repo: TriggerRepoImpl,
    pub post_office: PostOffice,
}

impl TestTriggerRepo {
    pub async fn new() -> Self {
        let pool = get_test_pool().await.expect("unable to get test pool");
        Self::new_with_pool(&pool)
    }

    pub fn new_with_pool(pool: &SqlitePool) -> Self {
        let trigger_repo = TriggerRepoImpl::new(&pool);
        Self {
            pool: pool.clone(),
            trigger_repo,
            post_office: PostOffice::open(),
        }
    }

    pub fn dummy_create_trigger(&self) -> CreateTrigger {
        CreateTrigger {
            event_name: fake::faker::name::en::Name().fake(),
            payload: Value::default(),
            metadata: None,
        }
    }
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

    pub async fn with_receiver(&self) -> Receiver<Event> {
        self.post_office.receive_mail().await.unwrap()
    }

    pub fn dummy_create_event(&self) -> CreateEvent {
        CreateEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            source_id: uuid::Uuid::new_v4().to_string(),
            event_name: fake::faker::name::en::Name().fake(),
            event_type: String::default(),
            payload: Value::default(),
            metadata: Value::default(),
        }
    }

    pub async fn insert_dummy_event(&self) -> EventsResult<Event> {
        let mut event = Event {
            id: i64::default(),
            event_id: uuid::Uuid::new_v4().to_string(),
            event_name: fake::faker::name::en::Name().fake(),
            event_type: String::default(),
            source_id: String::default(),
            payload: Value::default(),
            metadata: Value::default(),
            timestamp: Utc::now(),
        };

        let res = sqlx::query(
            r#"INSERT INTO events 
            (event_id, source_id, event_name, payload, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING id"#,
        )
        .bind(&event.event_id)
        .bind(&event.source_id)
        .bind(&event.event_name)
        .bind(&event.payload)
        .bind(&event.metadata)
        // .bind(event.tags)
        .fetch_one(&self.pool)
        .await
        .expect("unable to insert dummy data");

        let id = res.get("id");
        event.id = id;

        Ok(event)
    }
}

pub fn setup_temp_dir(fixture_dir_path: PathBuf) -> EventsResult<PathBuf> {
    // Create a temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    // Ensure the fixture directory exists
    assert!(fixture_dir_path.exists(), "Fixture directory not found");

    // Copy the contents of the fixture directory to the temporary directory
    copy_recursively(fixture_dir_path, temp_dir.path()).expect("Failed to copy directory");

    // Return the temporary directory path
    Ok(temp_dir.into_path())
}

pub fn get_fixtures_dir() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests");
    d.push("fixtures");
    d
}

pub fn setup_test_directory() -> EventsResult<AnythingEventsConfig> {
    let simple_fixture_dir = get_fixtures_dir().join("simple");
    let temp_dir = setup_temp_dir(simple_fixture_dir)?;
    let mut config = AnythingEventsConfig::default();
    config.root_dir = temp_dir.clone();
    Ok(config)
}
