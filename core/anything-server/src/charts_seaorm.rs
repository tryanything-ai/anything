use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::custom_auth::User;
use crate::entities::tasks;
use crate::AppState;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};

// Get workflow tasks chart using SeaORM
pub async fn get_workflow_tasks_chart(
    Path((account_id, workflow_id, start_date, end_date, time_unit, timezone)): Path<(
        String,
        String,
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_workflow_tasks_chart with SeaORM for workflow: {}", workflow_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    let workflow_uuid = match Uuid::parse_str(&workflow_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid workflow ID").into_response(),
    };

    // Parse dates (simplified - in production you'd want better date parsing)
    let start_datetime = format!("{}T00:00:00Z", start_date);
    let end_datetime = format!("{}T23:59:59Z", end_date);

    // Get tasks for the workflow in the date range
    let tasks_data = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::FlowId.eq(workflow_uuid))
        .filter(tasks::Column::CreatedAt.gte(start_datetime))
        .filter(tasks::Column::CreatedAt.lte(end_datetime))
        .order_by_asc(tasks::Column::CreatedAt)
        .all(&*state.db)
        .await
    {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Group tasks by date and status (simplified aggregation)
    let mut chart_data = Vec::new();
    let mut current_date = String::new();
    let mut daily_count = 0;
    let mut daily_success = 0;
    let mut daily_failed = 0;

    for task in tasks_data {
        let task_date = task.created_at
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        if task_date != current_date {
            if !current_date.is_empty() {
                chart_data.push(json!({
                    "date": current_date,
                    "total": daily_count,
                    "success": daily_success,
                    "failed": daily_failed
                }));
            }
            current_date = task_date;
            daily_count = 0;
            daily_success = 0;
            daily_failed = 0;
        }

        daily_count += 1;
        match task.task_status.as_str() {
            "completed" => daily_success += 1,
            "failed" | "error" => daily_failed += 1,
            _ => {}
        }
    }

    // Add the last day
    if !current_date.is_empty() {
        chart_data.push(json!({
            "date": current_date,
            "total": daily_count,
            "success": daily_success,
            "failed": daily_failed
        }));
    }

    println!("Successfully generated chart data with {} data points", chart_data.len());
    Json(json!({
        "chart_data": chart_data,
        "workflow_id": workflow_id,
        "time_range": {
            "start": start_date,
            "end": end_date,
            "unit": time_unit,
            "timezone": timezone
        }
    })).into_response()
}

// Get account tasks chart using SeaORM
pub async fn get_account_tasks_chart(
    Path((account_id, start_date, end_date, time_unit, timezone)): Path<(
        String,
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling get_account_tasks_chart with SeaORM for account: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Parse dates (simplified - in production you'd want better date parsing)
    let start_datetime = format!("{}T00:00:00Z", start_date);
    let end_datetime = format!("{}T23:59:59Z", end_date);

    // Get all tasks for the account in the date range
    let tasks_data = match tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(account_uuid))
        .filter(tasks::Column::CreatedAt.gte(start_datetime))
        .filter(tasks::Column::CreatedAt.lte(end_datetime))
        .order_by_asc(tasks::Column::CreatedAt)
        .all(&*state.db)
        .await
    {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Group tasks by date and status (simplified aggregation)
    let mut chart_data = Vec::new();
    let mut current_date = String::new();
    let mut daily_count = 0;
    let mut daily_success = 0;
    let mut daily_failed = 0;

    for task in tasks_data {
        let task_date = task.created_at
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());

        if task_date != current_date {
            if !current_date.is_empty() {
                chart_data.push(json!({
                    "date": current_date,
                    "total": daily_count,
                    "success": daily_success,
                    "failed": daily_failed
                }));
            }
            current_date = task_date;
            daily_count = 0;
            daily_success = 0;
            daily_failed = 0;
        }

        daily_count += 1;
        match task.task_status.as_str() {
            "completed" => daily_success += 1,
            "failed" | "error" => daily_failed += 1,
            _ => {}
        }
    }

    // Add the last day
    if !current_date.is_empty() {
        chart_data.push(json!({
            "date": current_date,
            "total": daily_count,
            "success": daily_success,
            "failed": daily_failed
        }));
    }

    println!("Successfully generated account chart data with {} data points", chart_data.len());
    Json(json!({
        "chart_data": chart_data,
        "account_id": account_id,
        "time_range": {
            "start": start_date,
            "end": end_date,
            "unit": time_unit,
            "timezone": timezone
        }
    })).into_response()
}
