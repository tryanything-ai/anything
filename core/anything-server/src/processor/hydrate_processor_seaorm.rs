// Hydrate processor using SeaORM
// This module handles restoration of running workflows after server restart

use crate::{
    processor::{
        utils::create_workflow_graph, 
        processor::ProcessorMessage,
    },
    types::{
        task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus},
        workflow_types::DatabaseFlowVersion,
    },
    entities::{tasks, flow_versions},
    AppState,
};

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

pub async fn hydrate_processor(state: Arc<AppState>) {
    println!("[HYDRATE PROCESSOR SEAORM] Starting processor hydration");

    // Get all running flow sessions using SeaORM
    let running_tasks = match tasks::Entity::find()
        .filter(tasks::Column::FlowSessionStatus.eq("running"))
        .filter(tasks::Column::CreatedAt.lt(chrono::Utc::now()))
        .all(&*state.db)
        .await
    {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("[HYDRATE PROCESSOR SEAORM] Error fetching flow sessions: {:?}", e);
            return;
        }
    };

    if running_tasks.is_empty() {
        println!("[HYDRATE PROCESSOR SEAORM] No running flow sessions found to hydrate");
        return;
    }

    println!("[HYDRATE PROCESSOR SEAORM] Found {} running tasks to hydrate", running_tasks.len());

    // Group tasks by flow_session_id
    let mut flow_sessions: HashMap<Uuid, Vec<tasks::Model>> = HashMap::new();
    for task in running_tasks {
        flow_sessions
            .entry(task.flow_session_id)
            .or_insert_with(Vec::new)
            .push(task);
    }

    // Process each flow session
    for (flow_session_id, session_tasks) in flow_sessions {
        println!("[HYDRATE PROCESSOR SEAORM] Hydrating flow session: {}", flow_session_id);
        
        if let Err(e) = hydrate_flow_session(state.clone(), flow_session_id, session_tasks).await {
            println!("[HYDRATE PROCESSOR SEAORM] Failed to hydrate flow session {}: {:?}", flow_session_id, e);
        }
    }

    println!("[HYDRATE PROCESSOR SEAORM] Processor hydration completed");
}

async fn hydrate_flow_session(
    state: Arc<AppState>,
    flow_session_id: Uuid,
    session_tasks: Vec<tasks::Model>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if session_tasks.is_empty() {
        return Ok(());
    }

    // Get the first task to extract workflow information
    let first_task = &session_tasks[0];
    let workflow_id = first_task.flow_id;
    let workflow_version_id = first_task.flow_version_id;

    // Get the workflow version
    let flow_version = flow_versions::Entity::find()
        .filter(flow_versions::Column::FlowVersionId.eq(workflow_version_id))
        .one(&*state.db)
        .await?
        .ok_or("Flow version not found")?;

    // TODO: Convert to DatabaseFlowVersion and create ProcessorMessage
    // For now, just log the hydration attempt
    println!("[HYDRATE PROCESSOR SEAORM] Would hydrate {} tasks for workflow {}", 
             session_tasks.len(), workflow_id);

    Ok(())
}

// TODO: Implement additional hydration utilities when needed
pub async fn cleanup_stale_sessions(state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[HYDRATE PROCESSOR SEAORM] Cleaning up stale sessions");
    
    // Find sessions that have been running for too long (e.g., more than 1 hour)
    let stale_cutoff = chrono::Utc::now() - chrono::Duration::hours(1);
    
    let stale_tasks = tasks::Entity::find()
        .filter(tasks::Column::FlowSessionStatus.eq("running"))
        .filter(tasks::Column::CreatedAt.lt(stale_cutoff))
        .all(&*state.db)
        .await?;

    if !stale_tasks.is_empty() {
        println!("[HYDRATE PROCESSOR SEAORM] Found {} stale tasks to clean up", stale_tasks.len());
        
        // TODO: Update stale tasks to failed status
        // This would require updating the task status in the database
    }

    Ok(())
}
