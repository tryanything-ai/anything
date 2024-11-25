use postgrest::Postgrest;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::VecDeque;

use dotenv::dotenv;
use std::env;

use crate::task_types::FlowSessionStatus;
use crate::task_types::Task;
use crate::task_types::TaskStatus;
use crate::task_types::{ActionType, TriggerSessionStatus};
use crate::workflow_types::{
    Action, CreateTaskInput, FlowVersion, TaskConfig, WorkflowVersionDefinition,
};

pub async fn process_trigger_task(
    client: &Postgrest,
    task: &Task,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    println!("[PROCESS TRIGGER TASK] Processing trigger task");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    // Fetch the flow version from the database
    let response = client
        .from("flow_versions")
        .auth(&supabase_service_role_api_key)
        .select("*")
        .eq("flow_version_id", task.flow_version_id.to_string())
        .limit(1)
        .execute()
        .await
        .map_err(|e| {
            println!("[PROCESS TRIGGER TASK] Error executing request: {:?}", e);
            e
        })?;

    let body = response.text().await.map_err(|e| {
        println!(
            "[PROCESS TRIGGER TASK] Error reading response body: {:?}",
            e
        );
        e
    })?;

    let flow_versions: Vec<FlowVersion> = serde_json::from_str(&body).map_err(|e| {
        println!("[PROCESS TRIGGER TASK] Error parsing JSON: {:?}", e);
        e
    })?;

    let flow_version = flow_versions.into_iter().next().ok_or_else(|| {
        let error_msg = format!("No flow version found for id: {}", task.flow_version_id);
        println!("[PROCESS TRIGGER TASK] {}", error_msg);
        error_msg
    })?;

    // Create the execution plan
    let new_tasks = create_execution_plan(task, flow_version).await?;

    // Insert the new tasks into the database
    client
        .from("tasks")
        .auth(&supabase_service_role_api_key)
        .insert(serde_json::to_string(&new_tasks)?)
        .execute()
        .await
        .map_err(|e| {
            println!("[PROCESS TRIGGER TASK] Error inserting new tasks: {:?}", e);
            e
        })?;

    println!("[PROCESS TRIGGER TASK] Trigger task processed successfully");

    // We set the result to the body of the webhook. this is already set in the task creation but doing again here.
    if let Some(plugin_id) = &task.plugin_id {
        if plugin_id == "webhook" {
            let config = task.result.clone().unwrap_or(serde_json::json!({}));
            return Ok(config);
        }
    }

    Ok(serde_json::json!({
        "message": format!("Trigger task {} processed successfully", task.task_id)
    }))
}

async fn create_execution_plan(
    task: &Task,
    flow_version: FlowVersion,
) -> Result<Vec<CreateTaskInput>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[EXECUTION_PLANNER] Creating execution plan");

    // Deserialize the flow definition into a Workflow struct
    let workflow: WorkflowVersionDefinition = serde_json::from_value(flow_version.flow_definition)
        .map_err(|e| {
            println!("[EXECUTION_PLANNER] Error deserializing workflow: {:?}", e);
            e
        })?;

    // Traverse the workflow to get the list of actions in BFS order, excluding the trigger
    let result = bfs_traversal(&workflow)?;

    let mut events = Vec::new();

    for (index, action) in result.iter().enumerate() {
        let task_config = TaskConfig {
            variables: serde_json::json!(action.variables),
            input: serde_json::json!(action.input),
        };

        let event = CreateTaskInput {
            account_id: task.account_id.to_string(),
            task_status: TaskStatus::Pending.as_str().to_string(),
            flow_id: task.flow_id.to_string(),
            flow_version_id: task.flow_version_id.to_string(),
            action_label: action.label.clone(),
            trigger_id: task.trigger_id.clone(),
            trigger_session_id: task.trigger_session_id.clone(),
            trigger_session_status: TriggerSessionStatus::Pending.as_str().to_string(),
            flow_session_id: task.flow_session_id.clone(),
            flow_session_status: FlowSessionStatus::Pending.as_str().to_string(),
            action_id: action.action_id.clone(),
            r#type: action.r#type.clone(),
            plugin_id: action.plugin_id.clone(),
            stage: task.stage.clone().as_str().to_string(),
            config: serde_json::json!(task_config),
            result: None,
            test_config: None,
            processing_order: (index + 1) as i32,
        };

        events.push(event);
    }

    println!("[EXECUTION_PLANNER] Execution plan created successfully");
    Ok(events)
}

fn bfs_traversal(
    workflow: &WorkflowVersionDefinition,
) -> Result<Vec<&Action>, Box<dyn std::error::Error + Send + Sync>> {
    println!("[EXECUTION_PLANNER] Starting BFS traversal");
    let mut work_list = Vec::new();

    // Create a map of node ids to their outgoing edges
    let mut graph = HashMap::new();
    for edge in &workflow.edges {
        graph
            .entry(&edge.source)
            .or_insert_with(Vec::new)
            .push(&edge.target);
    }

    // Use a BFS queue
    let mut queue = VecDeque::new();

    // Find the trigger action and enqueue its neighbors
    let trigger = workflow
        .actions
        .iter()
        .find(|action| matches!(action.r#type, ActionType::Trigger))
        .ok_or_else(|| {
            let error_msg = "Trigger not found in workflow".to_string();
            println!("[EXECUTION_PLANNER] Error: {}", error_msg);
            error_msg
        })?;

    if let Some(neighbors) = graph.get(&trigger.action_id) {
        for neighbor_id in neighbors {
            if let Some(neighbor) = workflow
                .actions
                .iter()
                .find(|action| &action.action_id == *neighbor_id)
            {
                queue.push_back(neighbor);
            }
        }
    }

    // BFS traversal
    while let Some(current) = queue.pop_front() {
        // Add current node to the work list, skipping the trigger action
        if !matches!(current.r#type, ActionType::Trigger) {
            work_list.push(current);
        }

        // Enqueue neighbors
        if let Some(neighbors) = graph.get(&current.action_id) {
            for neighbor_id in neighbors {
                if let Some(neighbor) = workflow
                    .actions
                    .iter()
                    .find(|action| &action.action_id == *neighbor_id)
                {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    println!("[EXECUTION_PLANNER] BFS traversal completed successfully");
    Ok(work_list)
}
