use axum::{extract::State, http::StatusCode, Json};
use tracing::trace;

use crate::messages::WorkerHeartbeat;

use super::AppState;

pub async fn post_heartbeat(
    State(state): State<AppState>,
    Json(beat): Json<WorkerHeartbeat>,
) -> (StatusCode, Json<String>) {
    println!("HERE");
    let store = state.context.store().clone();
    let pool = store.pool();

    // TODO - should heartbeats be JWT protected?

    trace!(uuid=?beat.uuid, "received heartbeat");

    let res = sqlx::query(
        "INSERT INTO worker(
            id,
            last_seen_datetime,
            version
        )
        VALUES($1, $2, $3)
        ON CONFLICT(id)
        DO UPDATE
        SET last_seen_datetime = $3,
            version = $6",
    )
    .bind(beat.uuid)
    .bind(beat.last_seen_datetime)
    .bind(&beat.version)
    .execute(pool)
    .await
    .unwrap();

    println!("HERE: {:?}", res);

    (StatusCode::OK, Json("Ok".to_string()))
}
