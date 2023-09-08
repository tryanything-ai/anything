use anyhow::Result;
use sqlx::{Pool, Sqlite};
use std::sync::{atomic::Ordering, Arc};
use tracing::trace;

use crate::constants::GIT_VERSION;

use super::server::Server;

pub async fn post_heartbeat(server: &Server, executor: &Pool<Sqlite>) -> Result<()> {
    // let waiting_for_trigger_id = *server
    //     .waiting_for_trigger_id
    //     .lock()
    //     .expect("unable to lock waiting_for_trigger_id");

    sqlx::query(
        "
        INSERT INTO scheduler(
            id,
            last_seen_datetime,
            version
        ) VALUES (
            $1,
            CURRENT_TIMESTAMP,
            $3
        )
        ON CONFLICT(id)
        DO UPDATE
        SET last_seen_datetime = CURRENT_TIMESTAMP
        ",
    )
    .bind(server.scheduler_id)
    // .bind(server.queued_triggers.load(Ordering::SeqCst) as i32)
    // .bind(waiting_for_trigger_id)
    .bind(GIT_VERSION)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn heartbeat(server: Arc<Server>) -> Result<()> {
    let store = server.context.store().clone();
    let pool = store.pool();

    loop {
        trace!("sending heartbeat");
        post_heartbeat(&server, pool).await?;

        tokio::time::sleep(std::time::Duration::from_secs(20)).await;
    }
}
