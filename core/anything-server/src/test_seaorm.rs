use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::processor::db_calls_seaorm;
use crate::AppState;

pub async fn test_seaorm_connection(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    match db_calls_seaorm::test_database_connection(state).await {
        Ok(()) => Ok(Json(json!({
            "status": "success",
            "message": "SeaORM database connection working!"
        }))),
        Err(e) => {
            eprintln!("SeaORM test failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn test_seaorm_query(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    // Test with a dummy account ID
    let test_account_id = uuid::Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .unwrap();
    
    match db_calls_seaorm::get_task_count_by_account(state, &test_account_id).await {
        Ok(count) => Ok(Json(json!({
            "status": "success",
            "message": "SeaORM query working!",
            "task_count": count
        }))),
        Err(e) => {
            eprintln!("SeaORM query test failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
