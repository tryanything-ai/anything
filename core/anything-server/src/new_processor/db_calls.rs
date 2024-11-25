use chrono::Utc;
use dotenv::dotenv;
use serde_json::Value;
use std::{env, sync::Arc};
use uuid::Uuid;

use crate::task_engine::UpdateTaskInput;
use crate::task_types::{Task, TaskStatus};
use crate::workflow_types::{CreateTaskInput, DatabaseFlowVersion};
use crate::AppState;

pub async fn get_workflow_definition(
    state: Arc<AppState>,
    workflow_id: &Uuid,
    version_id: Option<&Uuid>, // Make version_id optional since webhooks don't have it
) -> Result<DatabaseFlowVersion, String> {
    //Super User Access
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Get flow version from database
    let mut query = state
        .anything_client
        .from("flow_versions")
        .eq("flow_id", workflow_id.to_string());

    // If version_id is provided, use it. Otherwise get published version
    if let Some(version) = version_id {
        query = query.eq("flow_version_id", version.to_string());
    } else {
        query = query.eq("published", "true");
    }

    let response = query
        .auth(&supabase_service_role_api_key)
        .select("*")
        .single()
        .execute()
        .await
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let workflow_version: DatabaseFlowVersion = serde_json::from_str(&response_body)
        .map_err(|_| String::from("No workflow version found"))?;

    Ok(workflow_version)
}

pub async fn get_session_tasks(
    state: Arc<AppState>,
    flow_session_id: &Uuid, //UUID
) -> Result<Vec<Task>, String> {
    println!(
        "[DB_CALLS] Fetching tasks for flow_session_id {}",
        flow_session_id
    );

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .select("*")
        .eq("flow_session_id", flow_session_id.to_string())
        .order("processing_order.asc")
        .execute()
        .await
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let tasks: Vec<Task> = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse tasks: {}", e))?;

    if tasks.is_empty() {
        return Err("No tasks found for session".to_string());
    }

    Ok(tasks)
}
pub async fn create_task(state: Arc<AppState>, task: &CreateTaskInput) -> Result<Task, String> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let response = state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .insert(
            serde_json::to_value(task)
                .map_err(|e| format!("Failed to serialize task: {}", e))?
                .to_string(),
        )
        .execute()
        .await
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    let response_body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    let tasks: Vec<Task> = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse created task: {}", e))?;

    tasks
        .into_iter()
        .next()
        .ok_or_else(|| "No task was created".to_string())
}

//Send just the data we need. Safer to not update every key.
pub async fn update_task_status(
    state: Arc<AppState>,
    task_id: &Uuid,
    status: &TaskStatus,
    result: Option<Value>,
) -> Result<(), String> {
    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let started_at = if status.as_str() == TaskStatus::Running.as_str() {
        Some(Utc::now())
    } else {
        None
    };

    let ended_at = if status.as_str() != TaskStatus::Running.as_str() {
        Some(Utc::now())
    } else {
        None
    };

    let input = UpdateTaskInput {
        task_status: status.as_str().to_string(),
        started_at,
        ended_at,
        result,
    };

    state
        .anything_client
        .from("tasks")
        .auth(supabase_service_role_api_key)
        .eq("task_id", &task_id.to_string())
        .update(
            serde_json::to_string(&input)
                .map_err(|e| format!("Failed to serialize input: {}", e))?,
        )
        .execute()
        .await
        .map_err(|e| format!("Failed to execute request: {}", e))?;

    Ok(())
}
