use chrono::Utc;
use serde_json::Value;
use std::collections::HashSet;
use std::{sync::Arc};
use tracing::debug;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder, Order};

use crate::system_plugins::http::http_plugin::parse_headers;
use crate::types::{
    task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus},
    workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
};
use crate::entities::{tasks, flow_versions};
use crate::AppState;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFlowSesssionInput {
    pub flow_session_status: String,
    pub trigger_session_status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTaskInput {
    pub task_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

pub async fn get_workflow_definition(
    state: Arc<AppState>,
    workflow_id: &Uuid,
    version_id: Option<&Uuid>, // Make version_id optional since webhooks don't have it
) -> Result<DatabaseFlowVersion, String> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Getting workflow definition for workflow_id: {}, version_id: {:?}",
        workflow_id, version_id
    );

    let mut query = flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(*workflow_id));

    if let Some(version_id) = version_id {
        query = query.filter(flow_versions::Column::FlowVersionId.eq(*version_id));
    } else {
        // If no version_id specified, get the latest published version
        query = query
            .filter(flow_versions::Column::Published.eq(true))
            .order_by(flow_versions::Column::CreatedAt, Order::Desc);
    }

    let flow_version = match query.one(&*state.db).await {
        Ok(Some(version)) => version,
        Ok(None) => {
            return Err("No matching flow version found".to_string());
        }
        Err(err) => {
            println!("[PROCESSOR DB CALLS SEAORM] Database error: {:?}", err);
            return Err("Database error".to_string());
        }
    };

    // Convert to DatabaseFlowVersion format
    let workflow_def: WorkflowVersionDefinition = serde_json::from_value(flow_version.flow_definition)
        .map_err(|e| format!("Failed to parse workflow definition: {}", e))?;
    
    let database_flow_version = DatabaseFlowVersion {
        flow_version_id: flow_version.flow_version_id,
        account_id: flow_version.account_id,
        flow_id: flow_version.flow_id,
        flow: None, // Not used in current implementation
        published: flow_version.published,
        flow_definition: workflow_def,
    };

    println!("[PROCESSOR DB CALLS SEAORM] Successfully retrieved workflow definition");
    Ok(database_flow_version)
}

pub async fn get_session_tasks(
    state: Arc<AppState>,
    flow_session_id: &Uuid,
) -> Result<Vec<tasks::Model>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Getting session tasks for flow_session_id: {}",
        flow_session_id
    );

    let task_models = tasks::Entity::find()
        .filter(tasks::Column::FlowSessionId.eq(*flow_session_id))
        .order_by(tasks::Column::CreatedAt, Order::Asc)
        .all(&*state.db)
        .await?;

    // Convert task models to Task structs
    // TODO: This conversion is complex due to type differences between entity and Task struct
    // For now, we'll use a simplified approach
    let mut task_list = Vec::new();
    for task_model in task_models {
        // Note: This conversion is simplified and may need adjustment based on actual Task struct requirements
        println!("[PROCESSOR DB CALLS SEAORM] Converting task model to Task struct: {}", task_model.task_id);
        
        // For now, we'll create a minimal task representation
        // TODO: Implement proper type conversions when Task struct is fully defined
        task_list.push(task_model); // Temporarily store the model itself
    }

    println!(
        "[PROCESSOR DB CALLS SEAORM] Successfully retrieved {} session tasks",
        task_list.len()
    );
    Ok(task_list)
}

pub async fn insert_task(
    state: Arc<AppState>,
    task: &tasks::Model,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Inserting task: {}",
        task.task_id
    );

    // Convert task model to active model for insertion
    let new_task: tasks::ActiveModel = task.clone().into();

    new_task.insert(&*state.db).await?;

    println!("[PROCESSOR DB CALLS SEAORM] Successfully inserted task");
    Ok(())
}

pub async fn update_task(
    state: Arc<AppState>,
    task_id: &Uuid,
    update_input: UpdateTaskInput,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Updating task: {} with status: {}",
        task_id, update_input.task_status
    );

    // Find the existing task
    let existing_task = match tasks::Entity::find_by_id(*task_id).one(&*state.db).await? {
        Some(task) => task,
        None => return Err("Task not found".into()),
    };

    // Create an active model for updating
    let mut task_update: tasks::ActiveModel = existing_task.into();
    
    task_update.task_status = Set(update_input.task_status);
    task_update.updated_at = Set(Utc::now());

    if let Some(started_at) = update_input.started_at {
        task_update.started_at = Set(Some(started_at));
    }

    if let Some(ended_at) = update_input.ended_at {
        task_update.completed_at = Set(Some(ended_at));
    }

    if let Some(result) = update_input.result {
        task_update.output = Set(Some(result));
    }

    if let Some(context) = update_input.context {
        task_update.context = Set(Some(context));
    }

    if let Some(error) = update_input.error {
        task_update.error_message = Set(Some(error.to_string()));
    }

    task_update.update(&*state.db).await?;

    println!("[PROCESSOR DB CALLS SEAORM] Successfully updated task");
    Ok(())
}

pub async fn update_session_status(
    state: Arc<AppState>,
    flow_session_id: &Uuid,
    session_input: UpdateFlowSesssionInput,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Updating session status for flow_session_id: {}",
        flow_session_id
    );

    // Update all tasks in this session with the new status
    let task_models = tasks::Entity::find()
        .filter(tasks::Column::FlowSessionId.eq(*flow_session_id))
        .all(&*state.db)
        .await?;

    for task_model in task_models {
        let mut task_update: tasks::ActiveModel = task_model.into();
        task_update.flow_session_status = Set(session_input.flow_session_status.clone());
        task_update.trigger_session_status = Set(session_input.trigger_session_status.clone());
        task_update.updated_at = Set(Utc::now());
        
        task_update.update(&*state.db).await?;
    }

    println!("[PROCESSOR DB CALLS SEAORM] Successfully updated session status");
    Ok(())
}

// Helper function to get latest workflow version if needed
pub async fn get_latest_published_workflow_version(
    state: Arc<AppState>,
    workflow_id: &Uuid,
) -> Result<Option<flow_versions::Model>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Getting latest published version for workflow: {}",
        workflow_id
    );

    let version = flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(*workflow_id))
        .filter(flow_versions::Column::Published.eq(true))
        .order_by(flow_versions::Column::CreatedAt, Order::Desc)
        .one(&*state.db)
        .await?;

    println!(
        "[PROCESSOR DB CALLS SEAORM] Found latest version: {}",
        version.is_some()
    );
    Ok(version)
}

// Test functions for SeaORM connection
pub async fn test_database_connection(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESSOR DB CALLS SEAORM] Testing database connection");
    
    // Try a simple query to test the connection
    let _count = tasks::Entity::find().count(&*state.db).await?;
    
    println!("[PROCESSOR DB CALLS SEAORM] Database connection test successful");
    Ok(())
}

pub async fn get_task_count_by_account(
    state: Arc<AppState>,
    account_id: &Uuid,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "[PROCESSOR DB CALLS SEAORM] Getting task count for account: {}",
        account_id
    );

    let count = tasks::Entity::find()
        .filter(tasks::Column::AccountId.eq(*account_id))
        .count(&*state.db)
        .await?;

    println!(
        "[PROCESSOR DB CALLS SEAORM] Found {} tasks for account",
        count
    );
    Ok(count)
}