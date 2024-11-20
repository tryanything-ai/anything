use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::supabase_jwt_middleware::User;
use crate::AppState;

use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_tz::Tz;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize)]
struct ChartDataPoint {
    date: String,
    #[serde(flatten)]
    status_counts: HashMap<String, i32>,
}

fn parse_date_with_timezone(date_str: &str, tz: &Tz) -> DateTime<Tz> {
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(tz))
        .unwrap_or_else(|_| tz.from_utc_datetime(&Utc::now().naive_utc()))
}

pub async fn get_workflow_tasks_chart(
    Path((account_id, workflow_id, start_date, end_date, _timeunit, timezone)): Path<(
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
    let client = &state.anything_client;

    let tz: Tz = match Tz::from_str(&timezone) {
        Ok(tz) => tz,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid timezone").into_response(),
    };

    let start = parse_date_with_timezone(&start_date, &tz);
    let end = parse_date_with_timezone(&end_date, &tz);

    let query = client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .eq("flow_id", &workflow_id)
        .select("task_status, created_at")
        .gte("created_at", start.with_timezone(&Utc).to_rfc3339())
        .lte("created_at", end.with_timezone(&Utc).to_rfc3339());

    let response = match query.execute().await {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let tasks: Vec<Value> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    let all_statuses: Vec<String> = tasks
        .iter()
        .filter_map(|task| task["task_status"].as_str())
        .map(|s| s.to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let mut date_status_counts: HashMap<DateTime<Tz>, HashMap<String, i32>> = HashMap::new();

    let mut current = start.date().and_hms(0, 0, 0);
    while current <= end.max(Utc::now().with_timezone(&tz)) {
        let mut status_counts = HashMap::new();
        for status in &all_statuses {
            status_counts.insert(status.clone(), 0);
        }
        date_status_counts.insert(current, status_counts);
        current += Duration::days(1);
    }

    for task in tasks {
        let status = task["task_status"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let created_at = task["created_at"].as_str().unwrap_or("");
        if let Ok(date) = DateTime::parse_from_rfc3339(created_at) {
            let date_in_tz = date.with_timezone(&tz);
            let day_start = date_in_tz.date().and_hms(0, 0, 0);
            if let Some(date_counts) = date_status_counts.get_mut(&day_start) {
                *date_counts.entry(status).or_insert(0) += 1;
            }
        }
    }

    let mut chart_data: Vec<ChartDataPoint> = date_status_counts
        .into_iter()
        .map(|(date, status_counts)| ChartDataPoint {
            date: date.format("%Y-%m-%d").to_string(),
            status_counts,
        })
        .collect();

    chart_data.sort_by(|a, b| a.date.cmp(&b.date));

    Json(json!({ "chartData": chart_data })).into_response()
}

pub async fn get_account_tasks_chart(
    Path((account_id, start_date, end_date, _timeunit, timezone)): Path<(
        String,
        String,
        String,
        String,
        String,
    )>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let tz: Tz = match Tz::from_str(&timezone) {
        Ok(tz) => tz,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid timezone").into_response(),
    };

    let start = parse_date_with_timezone(&start_date, &tz);
    let end = parse_date_with_timezone(&end_date, &tz);

    let query = client
        .from("tasks")
        .auth(user.jwt)
        .eq("account_id", &account_id)
        .select("task_status, created_at")
        .gte("created_at", start.with_timezone(&Utc).to_rfc3339())
        .lte("created_at", end.with_timezone(&Utc).to_rfc3339());

    let response = match query.execute().await {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let tasks: Vec<Value> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    let all_statuses: Vec<String> = tasks
        .iter()
        .filter_map(|task| task["task_status"].as_str())
        .map(|s| s.to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let mut date_status_counts: HashMap<DateTime<Tz>, HashMap<String, i32>> = HashMap::new();

    let mut current = start.date().and_hms(0, 0, 0);
    while current <= end.max(Utc::now().with_timezone(&tz)) {
        let mut status_counts = HashMap::new();
        for status in &all_statuses {
            status_counts.insert(status.clone(), 0);
        }
        date_status_counts.insert(current, status_counts);
        current += Duration::days(1);
    }

    for task in tasks {
        let status = task["task_status"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let created_at = task["created_at"].as_str().unwrap_or("");
        if let Ok(date) = DateTime::parse_from_rfc3339(created_at) {
            let date_in_tz = date.with_timezone(&tz);
            let day_start = date_in_tz.date().and_hms(0, 0, 0);
            if let Some(date_counts) = date_status_counts.get_mut(&day_start) {
                *date_counts.entry(status).or_insert(0) += 1;
            }
        }
    }

    let mut chart_data: Vec<ChartDataPoint> = date_status_counts
        .into_iter()
        .map(|(date, status_counts)| ChartDataPoint {
            date: date.format("%Y-%m-%d").to_string(),
            status_counts,
        })
        .collect();

    chart_data.sort_by(|a, b| a.date.cmp(&b.date));

    Json(json!({ "chartData": chart_data })).into_response()
}
