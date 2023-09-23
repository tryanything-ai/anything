#![allow(unused)]

use anything_core::posix::copy_recursively;
use anything_engine::context;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use crate::models::flow::{CreateFlow, FlowId, FlowVersionId};
use crate::models::system_handler;
use crate::models::trigger::CreateTrigger;
use crate::repositories::flow_repo::{self, FlowRepoImpl};
use crate::repositories::trigger_repo::TriggerRepoImpl;
use crate::{
    config::AnythingEventsConfig,
    context::Context,
    errors::{EventsError, EventsResult},
    models::{
        event::{CreateEvent, Event, EventId, SourceId},
        tag::Tag,
    },
    post_office::PostOffice,
    repositories::{self, event_repo::EventRepoImpl, Repositories},
};
use crate::{CreateFlowVersion, Flow, FlowVersion, Server, Trigger};
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
    config.database.uri = Some("sqlite::memory:".to_string());
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
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        post_office: Arc::new(PostOffice::open()),
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
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        post_office: Arc::new(PostOffice::open()),
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
    Context {
        pool: Arc::new(pool.clone()),
        config,
        repositories,
        post_office: Arc::new(PostOffice::open()),
    }
}

pub async fn get_test_pool() -> EventsResult<Pool<sqlx::Sqlite>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| EventsError::MigrationError(e))?;

    Ok(pool)
}

pub async fn get_test_server() -> EventsResult<Arc<Server>> {
    let context = get_test_context().await;
    let server = Server::new(context).await?;
    Ok(server)
}

pub async fn select_all_events(pool: &SqlitePool) -> EventsResult<Vec<Event>> {
    let query = sqlx::query_as::<_, Event>(r#"SELECT * FROM events"#);

    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

    Ok(result)
}

pub async fn insert_new_event(pool: &SqlitePool, event: Event) -> EventsResult<i64> {
    let res = sqlx::query(
        r#"
        INSERT INTO events (flow_id, trigger_id, name, context, started_at, ended_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING id
        "#,
    )
    .bind(event.flow_id)
    .bind(event.trigger_id)
    .bind(event.name)
    .bind(event.context)
    .bind(event.started_at)
    .bind(event.ended_at)
    // .bind(event.tags)
    .execute(pool)
    .await
    .map_err(|e| EventsError::DatabaseError(e))?;

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
        id: uuid::Uuid::new_v4().to_string(),
        flow_id: Some(uuid::Uuid::new_v4().to_string()),
        trigger_id: Some(String::default()),
        name: fake::faker::name::en::Name().fake(),
        // tags: TagList::default(),
        context: Value::default(),
        started_at: Some(Utc::now()),
        ended_at: Some(Utc::now()),
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

    pub async fn with_sender(&self) -> Sender<Flow> {
        self.post_office.post_mail().await.unwrap()
    }

    pub async fn with_receiver(&self) -> Receiver<Flow> {
        self.post_office.receive_mail().await.unwrap()
    }

    pub fn dummy_create_flow(&self) -> CreateFlow {
        CreateFlow {
            flow_name: fake::faker::name::en::Name().fake(),
            version: Some("0.0.1".to_string()),
            active: Some(false),
        }
    }

    pub fn dummy_create_flow_version(&self, flow_id: String) -> CreateFlowVersion {
        CreateFlowVersion {
            flow_id,
            flow_definition: "{}".to_string(),
            published: Some(false),
            version: Some("0.0.1".to_string()),
            description: Some("test".to_string()),
        }
    }

    pub async fn insert_create_flow(
        &self,
        create_flow: CreateFlow,
    ) -> EventsResult<(FlowId, FlowVersionId)> {
        let flow_id = uuid::Uuid::new_v4().to_string();
        let version_id = uuid::Uuid::new_v4().to_string();
        let row = sqlx::query(
            r#"
        INSERT INTO flows (flow_id, flow_name, active, latest_version_id, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING flow_id
        "#,
        )
        .bind(flow_id.clone())
        .bind(create_flow.flow_name)
        .bind(create_flow.active)
        .bind(version_id.clone())
        .bind(Utc::now().timestamp())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        let flow_id = row.get(0);

        let all = sqlx::query_as::<_, Flow>("SELECT * from flows")
            .fetch_all(&self.pool)
            .await;

        Ok((flow_id, version_id))
    }

    pub async fn insert_create_flow_version(
        &self,
        flow_id: String,
        version_id: String,
        create_flow: CreateFlowVersion,
    ) -> EventsResult<FlowVersionId> {
        // Create flow version
        let row = sqlx::query(
            r#"
        INSERT INTO flow_versions (version_id, flow_id, flow_version, description, flow_definition, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING version_id
            "#,
        )
        .bind(version_id.clone())
        .bind(flow_id.clone())
        .bind(create_flow.version)
        .bind(create_flow.description)
        .bind("{}")
        .bind(Utc::now().timestamp())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            EventsError::DatabaseError(e)
        })?;

        Ok(row.get("version_id"))
    }

    pub async fn find_flow_by_id(&self, flow_id: String) -> EventsResult<Flow> {
        let flow = sqlx::query_as::<_, Flow>(r#"SELECT * FROM flows WHERE flow_id = ?1"#)
            .bind(flow_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| EventsError::DatabaseError(e));

        flow
    }

    pub async fn get_flow_versions(&self, flow_id: String) -> EventsResult<Vec<FlowVersion>> {
        let flow_versions = sqlx::query_as::<_, FlowVersion>(
            r#"
        SELECT * FROM flow_versions WHERE flow_id = ?1
        "#,
        )
        .bind(flow_id.clone())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(flow_versions)
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

    pub async fn with_sender(&self) -> Sender<Trigger> {
        self.post_office.post_mail().await.unwrap()
    }

    pub async fn with_receiver(&self) -> Receiver<Trigger> {
        self.post_office.receive_mail().await.unwrap()
    }

    pub fn dummy_create_trigger(&self) -> CreateTrigger {
        CreateTrigger {
            event_name: fake::faker::name::en::Name().fake(),
            payload: Value::default(),
            metadata: None,
            trigger_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub async fn insert_create_trigger(
        &self,
        create_trigger: CreateTrigger,
    ) -> EventsResult<FlowVersionId> {
        // Create flow version
        let row = sqlx::query(
            r#"
        INSERT INTO triggers (trigger_id, event_name, payload, metadata, timestamp)
        VALUES (?1, ?2, ?3, ?4, ?5)
        RETURNING trigger_id
        "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(create_trigger.event_name)
        .bind(create_trigger.payload)
        .bind(create_trigger.metadata)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;
        Ok(row.get("trigger_id"))
    }

    pub async fn get_all_triggers(&self) -> EventsResult<Vec<Trigger>> {
        let triggers = sqlx::query_as::<_, Trigger>(r#"SELECT * FROM triggers"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(triggers)
    }

    pub async fn select_trigger_by_id(&self, trigger_id: String) -> EventsResult<Trigger> {
        let trigger = sqlx::query_as::<_, Trigger>(
            r#"
            SELECT * FROM triggers WHERE trigger_id = ?1
        "#,
        )
        .bind(trigger_id.clone())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| EventsError::DatabaseError(e))?;

        Ok(trigger)
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
            name: fake::faker::name::en::Name().fake(),
            flow_id: Some(uuid::Uuid::new_v4().to_string()),
            trigger_id: Some(uuid::Uuid::new_v4().to_string()),
            context: Value::default(),
            started_at: Some(Utc::now()),
            ended_at: Some(Utc::now()),
        }
    }

    pub async fn insert_dummy_event(&self, event: CreateEvent) -> EventsResult<Event> {
        let event = Event {
            id: uuid::Uuid::new_v4().to_string(),
            name: event.name.clone(),
            flow_id: event.flow_id.clone(),
            trigger_id: event.trigger_id.clone(),
            context: event.context.clone(),
            started_at: event.started_at.clone(),
            ended_at: event.ended_at.clone(),
        };

        let mut cloned_event = event.clone();

        let row = sqlx::query(
            r#"
            INSERT INTO events (id, flow_id, trigger_id, name, context, started_at, ended_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING id
            "#,
        )
        .bind(event.id)
        .bind(event.flow_id)
        .bind(event.trigger_id)
        .bind(event.name)
        .bind(event.context)
        .bind(event.started_at)
        .bind(event.ended_at)
        // .bind(event.tags)
        .fetch_one(&self.pool)
        .await
        .expect("unable to insert dummy data");

        let id = row.get("id");
        cloned_event.id = id;

        Ok(cloned_event)
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

pub async fn setup_test_directory() -> EventsResult<Context> {
    let simple_fixture_dir = get_fixtures_dir().join("simple");
    let temp_dir = setup_temp_dir(simple_fixture_dir)?;
    let mut config = AnythingEventsConfig::default();
    config.root_dir = temp_dir.clone();
    let context = get_test_context_with_config(config.clone()).await;
    Ok(context)
}
