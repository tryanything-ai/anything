use crate::auth::init::AccountAuthProviderAccount;
use crate::bundler::accounts::{get_auth_accounts, get_refreshed_auth_accounts};
use crate::bundler::bundle_inputs;
use crate::bundler::secrets::get_decrypted_secrets;
use crate::new_processor::parsing_utils::get_bundle_context_inputs;
use crate::system_variables::get_system_variables;
use crate::task_types::Task;
use crate::templater::Templater;
use crate::AppState;
use postgrest::Postgrest;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tracing::debug;
use uuid::Uuid;

pub async fn bundle_cached_context(
    state: Arc<AppState>,
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context from parts");


    let (account_id, flow_session_id, variables_config, inputs_config) =
        get_bundle_context_inputs(task);
        
    let rendered_variables_definition = bundle_cached_variables(
        state,
        client,
        &account_id,
        &flow_session_id,
        variables_config,
        refresh_auth,
    )
    .await?;

    bundle_inputs(rendered_variables_definition, inputs_config)
}

pub async fn bundle_cached_variables(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
    flow_session_id: &str,
    variables_config: Option<&Value>,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    debug!("[BUNDLER] Starting to bundle variables");

    // Pre-allocate with known capacity
    let mut render_variables_context = HashMap::with_capacity(4);

    // Parallel fetch of secrets, accounts, and cached task results
    let (secrets_result, accounts_result, tasks_result) = tokio::join!(
        get_decrypted_secrets(state.clone(), client, account_id), //cached secrets
        fetch_auth_accounts(state.clone(), client, account_id, refresh_auth), //cached accounts
        fetch_cached_tasks(state.clone(), flow_session_id) //cached task results
    );

    // Process accounts
    let mut accounts = HashMap::new();
    for account in accounts_result? {
        let slug = account.account_auth_provider_account_slug.clone();
        debug!("[BUNDLER] Inserting account with slug: {}", slug);
        accounts.insert(slug, serde_json::to_value(account)?);
    }
    render_variables_context.insert("accounts".to_string(), serde_json::to_value(accounts)?);

    // Process secrets
    let mut secrets = HashMap::new();
    for secret in secrets_result? {
        let secret_name = secret.secret_name.clone();
        debug!("[BUNDLER] Inserting secret with name: {}", secret_name);
        secrets.insert(secret_name, serde_json::to_value(secret.secret_value)?);
    }
    render_variables_context.insert("secrets".to_string(), serde_json::to_value(secrets)?);

    // Process tasks
    let tasks_result = tasks_result?;
    let mut tasks_map = HashMap::with_capacity(tasks_result.len());
    for task in tasks_result {
        tasks_map.insert(task.action_id.to_string(), serde_json::to_value(task)?);
    }
    render_variables_context.insert("actions".to_string(), serde_json::to_value(tasks_map)?);

    // Add system variables
    render_variables_context.insert(
        "system".to_string(),
        serde_json::to_value(get_system_variables())?,
    );

    // Process variables config if present
    if let Some(variables) = variables_config {
        let mut templater = Templater::new();
        templater.add_template("task_variables_definition", variables.clone());

        let context_value = serde_json::to_value(&render_variables_context)?;
        let rendered = templater.render("task_variables_definition", &context_value)?;

        debug!("[BUNDLER] Rendered variables output: {}", rendered);
        Ok(rendered)
    } else {
        debug!("[BUNDLER] No variables found in task config");
        Ok(json!({}))
    }
}

async fn fetch_auth_accounts(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
    refresh_auth: bool,
) -> Result<Vec<AccountAuthProviderAccount>, Box<dyn Error + Send + Sync>> {
    if refresh_auth {
        get_refreshed_auth_accounts(state, client, account_id).await
    } else {
        get_auth_accounts(state, client, account_id).await
    }
}

async fn fetch_cached_tasks(
    state: Arc<AppState>,
    flow_session_id: &str,
) -> Result<Vec<Task>, Box<dyn Error + Send + Sync>> {
    let cache = state.flow_session_cache.read().await;
    let session_id = Uuid::parse_str(flow_session_id).unwrap();
    let tasks = if let Some(session_data) = cache.get(&session_id) {
        session_data.tasks.values().cloned().collect()
    } else {
        Vec::new()
    };
    Ok(tasks)
}