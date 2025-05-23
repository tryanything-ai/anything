#[cfg(test)]
mod enhanced_processor_tests {
    use crate::processor::enhanced_processor::*;
    use crate::processor::processor::ProcessorMessage;
    use crate::types::action_types::ActionType;
    use crate::types::workflow_types::{DatabaseFlowVersion, WorkflowVersionDefinition};
    use serde_json::Value;
    use tokio::sync::mpsc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_enhanced_processor_starts_and_stops() {
        // Create a mock AppState
        let state = create_test_app_state().await;

        // Create channels
        let (tx, rx) = mpsc::channel::<ProcessorMessage>(10);

        // Start the enhanced processor in a task
        let processor_handle = tokio::spawn(async move { enhanced_processor(state, rx).await });

        // Drop the sender to close the channel
        drop(tx);

        // The processor should exit cleanly
        let result = processor_handle.await;
        assert!(result.is_ok());

        // The inner result should also be Ok
        assert!(result.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_enhanced_processor_processes_message() {
        use std::time::Duration;

        // Create a mock AppState
        let state = create_test_app_state().await;

        // Create channels
        let (tx, rx) = mpsc::channel::<ProcessorMessage>(10);

        // Start the enhanced processor in a task
        let processor_handle = tokio::spawn(async move { enhanced_processor(state, rx).await });

        // Create a test message with all required fields
        let message = ProcessorMessage {
            workflow_id: Uuid::new_v4(),
            workflow_version: DatabaseFlowVersion {
                flow_version_id: Uuid::new_v4(),
                flow_id: Uuid::new_v4(),
                flow: Some(serde_json::json!({})),
                published: false,
                account_id: Uuid::new_v4(),
                flow_definition: WorkflowVersionDefinition {
                    actions: vec![],
                    edges: vec![],
                },
            },
            flow_session_id: Uuid::new_v4(),
            trigger_session_id: Uuid::new_v4(),
            trigger_task: None,
            task_id: Some(Uuid::new_v4()),
        };

        // Send the message
        tx.send(message).await.unwrap();

        // Give it a moment to process
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Close the channel
        drop(tx);

        // The processor should exit cleanly
        let result = processor_handle.await;
        assert!(result.is_ok());
    }

    async fn create_test_app_state() -> std::sync::Arc<crate::AppState> {
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::sync::{watch, Mutex, RwLock, Semaphore};

        // Create a proper S3 config with behavior version
        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region("us-east-1")
            .load()
            .await;

        let s3_client = aws_sdk_s3::Client::new(&aws_config);

        // This is a simplified mock - in a real test you'd want to mock all the dependencies
        Arc::new(crate::AppState {
            workflow_processor_semaphore: Arc::new(Semaphore::new(2)),
            anything_client: Arc::new(postgrest::Postgrest::new("http://test")),
            marketplace_client: Arc::new(postgrest::Postgrest::new("http://test")),
            public_client: Arc::new(postgrest::Postgrest::new("http://test")),
            r2_client: Arc::new(s3_client),
            http_client: Arc::new(reqwest::Client::new()),
            auth_states: RwLock::new(HashMap::new()),
            trigger_engine_signal: watch::channel("".to_string()).0,
            processor_sender: mpsc::channel::<ProcessorMessage>(100).0,
            task_updater_sender: mpsc::channel::<crate::status_updater::StatusUpdateMessage>(100).0,
            flow_completions: Arc::new(Mutex::new(HashMap::new())),
            api_key_cache: Arc::new(RwLock::new(HashMap::new())),
            account_access_cache: Arc::new(RwLock::new(
                crate::account_auth_middleware::AccountAccessCache::new(
                    std::time::Duration::from_secs(60),
                ),
            )),
            bundler_secrets_cache: RwLock::new(
                crate::bundler::secrets::secrets_cache::SecretsCache::new(
                    std::time::Duration::from_secs(60),
                ),
            ),
            bundler_accounts_cache: RwLock::new(
                crate::bundler::accounts::accounts_cache::AccountsCache::new(
                    std::time::Duration::from_secs(60),
                ),
            ),
            flow_session_cache: Arc::new(RwLock::new(
                crate::processor::flow_session_cache::FlowSessionCache::new(
                    std::time::Duration::from_secs(60),
                ),
            )),
            shutdown_signal: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }
}
