// use crate::{
//     processor::{
//         utils::create_workflow_graph, db_calls::update_flow_session_status,
//         flow_session_cache::FlowSessionData, processor::ProcessorMessage,
//     },
//     types::{
//         task_types::{FlowSessionStatus, Task, TaskStatus, TriggerSessionStatus},
//         workflow_types::DatabaseFlowVersion,
//     },
//     AppState,
// };

// use dotenv::dotenv;
// use postgrest::Postgrest;
// use std::{
//     collections::{HashMap, HashSet},
//     env,
//     sync::Arc,
// };
// use uuid::Uuid;

// pub async fn hydrate_processor(state: Arc<AppState>) {
//     println!("[HYDRATE PROCESSOR] Starting processor hydration");

//     dotenv().ok();
//     let supabase_service_role_api_key = env::var("SUPABASE_SERVICE_ROLE_API_KEY")
//         .expect("SUPABASE_SERVICE_ROLE_API_KEY must be set");

//     let client = state.anything_client.clone();

//     // Get all running flow sessions before the current time
//     let response = match client
//         .from("tasks")
//         .auth(supabase_service_role_api_key.clone())
//         .select("*")
//         .eq("flow_session_status", "running")
//         .lt("created_at", chrono::Utc::now().to_rfc3339())
//         .execute()
//         .await
//     {
//         Ok(response) => response,
//         Err(e) => {
//             println!("[HYDRATE PROCESSOR] Error fetching flow sessions: {:?}", e);
//             return;
//         }
//     };

//     let body = match response.text().await {
//         Ok(body) => body,
//         Err(e) => {
//             println!("[HYDRATE PROCESSOR] Error getting response text: {:?}", e);
//             return;
//         }
//     };

//     let tasks: Vec<Task> = match serde_json::from_str(&body) {
//         Ok(tasks) => tasks,
//         Err(e) => {
//             println!("[HYDRATE PROCESSOR] Error parsing tasks: {:?}", e);
//             return;
//         }
//     };

//     println!(
//         "[HYDRATE PROCESSOR] Found {} tasks to manage in hydrate",
//         tasks.len()
//     );

//     let mut seen_sessions = HashMap::new();

//     for task in tasks {
//         let session_id = task.flow_session_id;
//         let flow_version_id = task.flow_version_id;
//         let trigger_session_id = task.trigger_session_id;

//         if !seen_sessions.contains_key(&session_id) {
//             let tasks_future =
//                 get_flow_session_tasks(&client, &session_id, &supabase_service_role_api_key);
//             let workflow_future =
//                 get_workflow_definition(&client, &flow_version_id, &supabase_service_role_api_key);

//             match tokio::try_join!(tasks_future, workflow_future) {
//                 Ok((session_tasks, workflow_def)) => {
//                     seen_sessions.insert(session_id.clone(), true);

//                     let mut workflow_failed = false;

//                     // Check if the workflow is completed but for some reason not marked as so
//                     if let Some(workflow) = &workflow_def {
//                         let graph = create_workflow_graph(&workflow.flow_definition);
//                         let mut seen_actions = HashSet::new();

//                         // Add all task action_ids we have to seen set
//                         for task in &session_tasks {
//                             if task.task_status == TaskStatus::Failed {
//                                 workflow_failed = true;
//                                 break;
//                             }
//                             seen_actions.insert(task.action_id.clone());
//                         }

//                         // Check if any nodes in graph are missing from our tasks
//                         let mut finished_processing_graph = true;
//                         for (action_id, _) in &graph {
//                             if !seen_actions.contains(action_id) {
//                                 finished_processing_graph = false;
//                                 println!(
//                                     "[HYDRATE PROCESSOR] Missing task for action {}",
//                                     action_id
//                                 );
//                                 break;
//                             }
//                         }

//                         if finished_processing_graph {
//                             // We have all tasks - mark flow session as completed
//                             println!(
//                                 "[HYDRATE PROCESSOR] Marking flow session {} as {}",
//                                 session_id,
//                                 if workflow_failed {
//                                     "failed"
//                                 } else {
//                                     "completed"
//                                 }
//                             );
//                             //THis is basically cleanup. this should not happen often but if it does this will "cure" it
//                             if let Err(e) = update_flow_session_status(
//                                 &state,
//                                 &Uuid::parse_str(&session_id).unwrap(),
//                                 if workflow_failed {
//                                     &FlowSessionStatus::Failed
//                                 } else {
//                                     &FlowSessionStatus::Completed
//                                 },
//                                 if workflow_failed {
//                                     &TriggerSessionStatus::Failed
//                                 } else {
//                                     &TriggerSessionStatus::Completed
//                                 },
//                             )
//                             .await
//                             {
//                                 println!(
//                                     "[HYDRATE PROCESSOR] Failed to update flow session status: {}",
//                                     e
//                                 );
//                             }
//                             //get out of loop
//                             continue;
//                         } else {
//                             println!(
//                                 "[HYDRATE PROCESSOR] Starting up processor for flow session {}",
//                                 session_id
//                             );
//                         }
//                     }

//                     //Put workflow in the cache
//                     let flow_session_data = FlowSessionData {
//                         workflow: workflow_def.clone(),
//                         tasks: session_tasks.into_iter().map(|t| (t.task_id, t)).collect(),
//                         flow_session_id: Uuid::parse_str(&session_id).unwrap(),
//                         workflow_id: workflow_def.clone().unwrap().flow_id,
//                         workflow_version_id: Some(flow_version_id),
//                     };

//                     println!("[HYDRATE PROCESSOR] Setting flow session data in cache");
//                     // Set the flow session data in cache
//                     {
//                         let mut cache = state.flow_session_cache.write().await;
//                         cache.set(&Uuid::parse_str(&session_id).unwrap(), flow_session_data);
//                     }

//                     //Send message to processor to start the workflow
//                     let processor_message = ProcessorMessage {
//                         workflow_id: workflow_def.unwrap().flow_id,
//                         version_id: Some(flow_version_id),
//                         flow_session_id: Uuid::parse_str(&session_id).unwrap(),
//                         trigger_session_id: Uuid::parse_str(&trigger_session_id).unwrap(),
//                         trigger_task: None,
//                     };

//                     if let Err(e) = state.processor_sender.send(processor_message).await {
//                         println!(
//                             "[HYDRATE PROCESSOR] Failed to send message to processor: {}",
//                             e
//                         );
//                         return;
//                     }
//                 }
//                 Err(e) => {
//                     println!(
//                         "[HYDRATE PROCESSOR] Error getting data for session {}: {:?}",
//                         session_id, e
//                     );
//                 }
//             }
//         }
//     }

//     println!("[HYDRATE PROCESSOR] Completed processor hydration");
// }

// async fn get_workflow_definition(
//     client: &Postgrest,
//     version_id: &Uuid,
//     api_key: &str,
// ) -> Result<Option<DatabaseFlowVersion>, Box<dyn std::error::Error + Send + Sync>> {
//     let response = client
//         .from("flow_versions")
//         .auth(api_key)
//         .select("*")
//         .eq("flow_version_id", version_id.to_string())
//         .single()
//         .execute()
//         .await?;

//     let body = response.text().await?;
//     let version: DatabaseFlowVersion = serde_json::from_str(&body)?;

//     Ok(Some(version))
// }

// async fn get_flow_session_tasks(
//     client: &Postgrest,
//     session_id: &str,
//     api_key: &str,
// ) -> Result<Vec<Task>, Box<dyn std::error::Error + Send + Sync>> {
//     let response = client
//         .from("tasks")
//         .auth(api_key)
//         .select("*")
//         .eq("flow_session_id", session_id)
//         .execute()
//         .await?;

//     let body = response.text().await?;
//     let tasks: Vec<Task> = serde_json::from_str(&body)?;

//     Ok(tasks)
// }
