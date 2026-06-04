use chrono::Utc;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Order, ActiveModelTrait, Set};

use crate::types::{
    task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus},
    workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition},
};
use crate::entities::{flow_versions, flows, tasks};
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
        "[PROCESSOR DB CALLS] Getting workflow definition for workflow_id: {}, version_id: {:?}",
        workflow_id, version_id
    );
    
    let mut query = flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowId.eq(*workflow_id));
    
    // If version_id is provided, use it. Otherwise get published version
    if let Some(version) = version_id {
        query = query.filter(flow_versions::Column::FlowVersionId.eq(*version));
    } else {
        query = query.filter(flow_versions::Column::Published.eq(true))
            .order_by(flow_versions::Column::PublishedAt, Order::Desc);
    }
    
    let flow_version = query
        .one(&*state.db)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "No workflow version found".to_string())?;
    
    // Convert from entity model to DatabaseFlowVersion
    // Note: We need to deserialize the JSON flow_definition into WorkflowVersionDefinition
    let flow_definition: WorkflowVersionDefinition = serde_json::from_value(flow_version.flow_definition)
        .map_err(|e| format!("Failed to deserialize flow definition: {}", e))?;
    
    let database_flow_version = DatabaseFlowVersion {
        flow_version_id: flow_version.flow_version_id,
        flow_id: flow_version.flow_id,
        flow: None, // Not using this field
        published: flow_version.published,
        account_id: flow_version.account_id,
        flow_definition,
    };
    
    println!("[PROCESSOR DB CALLS] Successfully retrieved workflow definition");
    Ok(database_flow_version)
}

pub async fn get_session_tasks(
    state: Arc<AppState>,
    flow_session_id: &Uuid, //UUID
) -> Result<Vec<Task>, String> {
    println!(
        "[PROCESSOR DB CALLS] Fetching tasks for flow_session_id {}",
        flow_session_id
    );

    let task_models = tasks::Entity::find()
        .filter(tasks::Column::FlowSessionId.eq(flow_session_id.to_string()))
        .order_by(tasks::Column::ProcessingOrder, Order::Asc)
        .all(&*state.db)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if task_models.is_empty() {
        println!(
            "[PROCESSOR DB CALLS] No tasks found for session {}",
            flow_session_id
        );
        return Err("No tasks found for session".to_string());
    }

    // Convert from entity models to Task structs
    // Note: This conversion will need to be implemented properly based on your Task struct
    // For now, return an error indicating this conversion needs to be implemented
    println!(
        "[PROCESSOR DB CALLS] Found {} tasks, but conversion from entity to Task struct needs implementation",
        task_models.len()
    );
    
    Err("Task conversion from entity to struct needs implementation".to_string())
}

pub async fn create_task(state: Arc<AppState>, task: &Task) -> Result<(), String> {
    println!("[PROCESSOR DB CALLS] Creating new task: {}", task.task_id);
    
    let new_task = tasks::ActiveModel {
        task_id: Set(task.task_id),
        account_id: Set(task.account_id),
        task_status: Set(task.task_status.to_string()),
        flow_id: Set(task.flow_id),
        flow_version_id: Set(task.flow_version_id),
        action_label: Set(task.action_label.clone()),
        trigger_id: Set(task.trigger_id.clone()),
        trigger_session_id: Set(task.trigger_session_id.to_string()),
        trigger_session_status: Set(task.trigger_session_status.to_string()),
        flow_session_id: Set(task.flow_session_id.to_string()),
        flow_session_status: Set(task.flow_session_status.to_string()),
        action_id: Set(task.action_id.clone()),
        r#type: Set(format!("{:?}", task.r#type)),
        plugin_name: Set(task.plugin_name.as_ref().map(|p| p.to_string())),
        plugin_version: Set(task.plugin_version.as_ref().map(|v| v.to_string())),
        stage: Set(task.stage.to_string()),
        test_config: Set(task.test_config.clone()),
        config: Set(serde_json::to_value(&task.config).map_err(|e| format!("Failed to serialize config: {}", e))?),
        context: Set(task.context.clone()),
        started_at: Set(task.started_at.map(|dt| dt.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()))),
        ended_at: Set(None), // tasks don't have ended_at initially
        completed_at: Set(task.ended_at.map(|dt| dt.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()))),
        debug_result: Set(task.debug_result.clone()),
        result: Set(task.result.clone()),
        output: Set(None), // output is separate from result
        processing_order: Set(task.processing_order),
        error: Set(task.error.clone()),
        error_message: Set(None), // will be extracted from error if needed
        execution_time_ms: Set(None), // calculated later
        retry_count: Set(None), // starts at 0
        archived: Set(task.archived),
        updated_at: Set(task.updated_at.map(|dt| dt.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()))),
        created_at: Set(task.created_at.map(|dt| dt.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap()))),
        updated_by: Set(task.updated_by),
        created_by: Set(task.created_by),
    };
    
    new_task.insert(&*state.db)
        .await
        .map_err(|e| format!("Failed to insert task: {}", e))?;
    
    println!("[PROCESSOR DB CALLS] Successfully created task");
    Ok(())
}

//Send just the data we need. Safer to not update every key.
pub async fn update_task_status(
    state: Arc<AppState>,
    task_id: &Uuid,
    status: &TaskStatus,
    context: Option<Value>,
    result: Option<Value>,
    error: Option<Value>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
) -> Result<(), String> {
    println!(
        "[PROCESSOR DB CALLS] Updating task {} status to {}",
        task_id,
        status.as_str()
    );

    // Remove sensitive headers from context
    let cleaned_context = if let Some(context) = context {
        Some(redact_headers_from_context(&context))
    } else {
        None
    };

    // Find the existing task
    let existing_task = tasks::Entity::find_by_id(*task_id)
        .one(&*state.db)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| "Task not found".to_string())?;

    // Create an active model for updating
    let mut task_update: tasks::ActiveModel = existing_task.into();
    
    task_update.task_status = Set(status.to_string());
    task_update.updated_at = Set(Some(Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())));

    if let Some(context) = cleaned_context {
        task_update.context = Set(Some(context));
    }

    if let Some(result) = result {
        task_update.result = Set(Some(result));
    }

    if let Some(error) = error {
        task_update.error = Set(Some(error));
    }

    if let Some(started_at) = started_at {
        task_update.started_at = Set(Some(started_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())));
    }

    if let Some(ended_at) = ended_at {
        task_update.completed_at = Set(Some(ended_at.with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())));
    }

    task_update.update(&*state.db)
        .await
        .map_err(|e| format!("Failed to update task: {}", e))?;

    println!("[PROCESSOR DB CALLS] Successfully updated task status");
    Ok(())
}

pub async fn update_flow_session_status(
    state: &AppState,
    flow_session_id: &Uuid,
    flow_session_status: &FlowSessionStatus,
    trigger_session_status: &TriggerSessionStatus,
) -> Result<(), String> {
    println!(
        "[PROCESSOR DB CALLS] Updating flow session {} status to {} and trigger status to {}",
        flow_session_id,
        flow_session_status.as_str(),
        trigger_session_status.as_str()
    );

    // Find all tasks with this flow_session_id and update their status
    let task_models = tasks::Entity::find()
        .filter(tasks::Column::FlowSessionId.eq(flow_session_id.to_string()))
        .all(&*state.db)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    for task_model in task_models {
        let mut task_update: tasks::ActiveModel = task_model.into();
        task_update.flow_session_status = Set(flow_session_status.to_string());
        task_update.trigger_session_status = Set(trigger_session_status.to_string());
        task_update.updated_at = Set(Some(Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())));
        
        task_update.update(&*state.db)
            .await
            .map_err(|e| format!("Failed to update task: {}", e))?;
    }

    println!("[PROCESSOR DB CALLS] Successfully updated session status");
    Ok(())
}

fn redact_headers_from_context(context: &Value) -> Value {
    // This function is used to remove sensitive headers from context
    // For now, just return the context as-is
    // TODO: Implement proper header redaction when needed
    context.clone()
}