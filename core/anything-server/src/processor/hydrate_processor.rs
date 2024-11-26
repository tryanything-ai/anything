use crate::{
    processor::{flow_session_cache::FlowSessionData, processor::ProcessorMessage},
    task_types::Task,
    workflow_types::DatabaseFlowVersion,
    AppState,
};

use dotenv::dotenv;
use postgrest::Postgrest;
use std::{collections::HashMap, env, sync::Arc};
use uuid::Uuid;

pub async fn hydrate_processor(state: Arc<AppState>) {
    println!("[HYDRATE PROCESSOR] Starting processor hydration");

    dotenv().ok();
    let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
        .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

    let client = state.anything_client.clone();

    // Get all running flow sessions before the current time
    let response = match client
        .from("tasks")
        .auth(supabase_service_role_api_key.clone())
        .select("*")
        .eq("flow_session_status", "running")
        .lt("created_at", chrono::Utc::now().to_rfc3339())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[HYDRATE PROCESSOR] Error fetching flow sessions: {:?}", e);
            return;
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[HYDRATE PROCESSOR] Error getting response text: {:?}", e);
            return;
        }
    };

    let tasks: Vec<Task> = match serde_json::from_str(&body) {
        Ok(tasks) => tasks,
        Err(e) => {
            println!("[HYDRATE PROCESSOR] Error parsing tasks: {:?}", e);
            return;
        }
    };

    println!("[HYDRATE PROCESSOR] Found {} running* tasks", tasks.len());

    let mut seen_sessions = HashMap::new();

    for task in tasks {
        let session_id = task.flow_session_id;
        let flow_version_id = task.flow_version_id;

        if !seen_sessions.contains_key(&session_id) {
            let tasks_future =
                get_flow_session_tasks(&client, &session_id, &supabase_service_role_api_key);
            let workflow_future =
                get_workflow_definition(&client, &flow_version_id, &supabase_service_role_api_key);

            match tokio::try_join!(tasks_future, workflow_future) {
                Ok((session_tasks, workflow_def)) => {
                    seen_sessions.insert(session_id.clone(), true);

                    //Put workflow in the cache
                    let flow_session_data = FlowSessionData {
                        workflow: workflow_def.clone(),
                        tasks: session_tasks.into_iter().map(|t| (t.task_id, t)).collect(),
                        flow_session_id: Uuid::parse_str(&session_id).unwrap(),
                        workflow_id: workflow_def.clone().unwrap().flow_id,
                        workflow_version_id: Some(flow_version_id),
                    };

                    println!("[HYDRATE PROCESSOR] Setting flow session data in cache");
                    // Set the flow session data in cache
                    {
                        let mut cache = state.flow_session_cache.write().await;
                        cache.set(&Uuid::parse_str(&session_id).unwrap(), flow_session_data);
                    }

                    //Send message to processor to start the workflow
                    let processor_message = ProcessorMessage {
                        workflow_id: workflow_def.unwrap().flow_id,
                        version_id: Some(flow_version_id),
                        flow_session_id: Uuid::parse_str(&session_id).unwrap(),
                        trigger_task: None,
                    };

                    if let Err(e) = state.processor_sender.send(processor_message).await {
                        println!("[HYDRATE PROCESSOR] Failed to send message to processor: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    println!(
                        "[HYDRATE PROCESSOR] Error getting data for session {}: {:?}",
                        session_id, e
                    );
                }
            }
        }
    }

    println!("[HYDRATE PROCESSOR] Completed processor hydration");
}

async fn get_workflow_definition(
    client: &Postgrest,
    version_id: &Uuid,
    api_key: &str,
) -> Result<Option<DatabaseFlowVersion>, Box<dyn std::error::Error + Send + Sync>> {
    let response = client
        .from("flow_versions")
        .auth(api_key)
        .select("*")
        .eq("flow_version_id", version_id.to_string())
        .single()
        .execute()
        .await?;

    let body = response.text().await?;
    let version: DatabaseFlowVersion = serde_json::from_str(&body)?;

    Ok(Some(version))
}

async fn get_flow_session_tasks(
    client: &Postgrest,
    session_id: &str,
    api_key: &str,
) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
    let response = client
        .from("tasks")
        .auth(api_key)
        .select("*")
        .eq("flow_session_id", session_id)
        .execute()
        .await?;

    let body = response.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&body)?;

    Ok(tasks)
}
