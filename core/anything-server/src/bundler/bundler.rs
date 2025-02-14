use crate::system_variables::get_system_variables;
use crate::types::json_schema::JsonSchema;
use crate::types::task_types::Task;

use crate::AppState;
use postgrest::Postgrest;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::bundler::accounts::fetch_cached_auth_accounts;
use crate::bundler::secrets::get_decrypted_secrets;
use crate::templater::Templater;
use crate::types::task_types::TaskStatus;

use uuid::Uuid;

use crate::types::json_schema::ValidationFieldType;

pub async fn bundle_tasks_cached_context(
    state: Arc<AppState>,
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<(Value, Value), Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context from parts");

    let rendered_variables_definition =
        bundle_tasks_cached_variables(state, client, task, refresh_auth).await?;

    let plugin_config = task.config.plugin_config.as_ref();
    let plugin_config_schema = task.config.plugin_config_schema.as_ref();

    let rendered_plugin_config_definition = bundle_plugin_config(
        rendered_variables_definition.clone(),
        plugin_config,
        plugin_config_schema,
    )?;

    Ok((
        rendered_variables_definition,
        rendered_plugin_config_definition,
    ))
}

pub async fn bundle_tasks_cached_variables(
    state: Arc<AppState>,
    client: &Postgrest,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context from parts");

    let account_id = task.account_id.to_string();
    let flow_session_id = task.flow_session_id.to_string();
    let variables = task.config.inputs.as_ref();
    let variables_schema = task.config.inputs_schema.as_ref();

    let rendered_variables_definition = bundle_cached_variables(
        state,
        client,
        &account_id,
        &flow_session_id,
        variables,
        variables_schema,
        refresh_auth,
    )
    .await?;

    Ok(rendered_variables_definition)
}

pub async fn bundle_context_from_parts(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
    flow_session_id: &str,
    variables: Option<&Value>,
    variables_schema: Option<&JsonSchema>,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&JsonSchema>,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle context from parts");

    let rendered_variables_definition = bundle_cached_variables(
        state,
        client,
        account_id,
        flow_session_id,
        variables,
        variables_schema,
        // plugin_config,
        // plugin_config_schema,
        refresh_auth,
    )
    .await?;

    bundle_plugin_config(
        rendered_variables_definition,
        plugin_config,
        plugin_config_schema,
    )
}

pub async fn bundle_cached_variables(
    state: Arc<AppState>,
    client: &Postgrest,
    account_id: &str,
    flow_session_id: &str,
    variables: Option<&Value>,
    variables_schema: Option<&JsonSchema>,
    // input: Option<&Value>,
    // input_schema: Option<&JsonSchema>,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER] Starting to bundle variables");

    // Pre-allocate with known capacity
    let mut render_variables_context = HashMap::with_capacity(4);

    // Parallel fetch of secrets, accounts, and cached task results
    let (secrets_result, accounts_result, tasks_result) = tokio::join!(
        get_decrypted_secrets(state.clone(), client, account_id), //cached secrets
        fetch_cached_auth_accounts(state.clone(), client, account_id, refresh_auth), //cached accounts
        fetch_completed_cached_tasks(state.clone(), flow_session_id) //cached task results
    );

    // Process accounts
    let mut accounts = HashMap::new();
    for account in accounts_result? {
        let slug = account.account_auth_provider_account_slug.clone();
        println!("[BUNDLER] Inserting account with slug: {}", slug);
        accounts.insert(slug, serde_json::to_value(account)?);
    }
    render_variables_context.insert("accounts".to_string(), serde_json::to_value(accounts)?);

    // Process secrets
    let mut secrets = HashMap::new();
    for secret in secrets_result? {
        let secret_name = secret.secret_name.clone();
        println!("[BUNDLER] Inserting secret with name: {}", secret_name);
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

    // Extract and set validations from schemas
    let mut templater = Templater::new();

    if let Some(variables) = variables {
        templater.add_template("task_variables_definition", variables.clone());

        let variable_validations = extract_template_key_validations_from_schema(variables_schema);
        let context_value = serde_json::to_value(&render_variables_context)?;
        let rendered = templater.render(
            "task_variables_definition",
            &context_value,
            variable_validations,
        )?;

        println!("[BUNDLER] Rendered variables output: {}", rendered);
        Ok(rendered)
    } else {
        println!("[BUNDLER] No variables found in task config");
        Ok(json!({}))
    }
}

async fn fetch_completed_cached_tasks(
    state: Arc<AppState>,
    flow_session_id: &str,
) -> Result<Vec<Task>, Box<dyn Error + Send + Sync>> {
    let cache = state.flow_session_cache.read().await;
    let session_id = Uuid::parse_str(flow_session_id).unwrap();
    let tasks = if let Some(session_data) = cache.get(&session_id) {
        session_data
            .tasks
            .values()
            .filter(|task| task.task_status == TaskStatus::Completed)
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    Ok(tasks)
}

pub fn bundle_plugin_config(
    rendered_variables: Value,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&JsonSchema>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut render_input_context: HashMap<String, Value> = HashMap::new();
    render_input_context.insert("variables".to_string(), rendered_variables);

    // Create a new Templater instance for rendering inputs
    let mut templater = Templater::new();

    // Convert context HashMap to Value
    let inputs_context_value = serde_json::to_value(render_input_context.clone())?;

    // Add the task definition as a template and render if it exists
    if let Some(plugin_config) = plugin_config {
        println!(
            "[BUNDLER] Task plugin config definition: {}",
            plugin_config.clone()
        );
        templater.add_template("task_plugin_config_definition", plugin_config.clone());

        let plugin_config_validations =
            extract_template_key_validations_from_schema(plugin_config_schema);
        // Render the task definition with the context
        let rendered_plugin_config_definition = templater.render(
            "task_plugin_config_definition",
            &inputs_context_value,
            plugin_config_validations,
        )?;
        println!(
            "[BUNDLER] Rendered plugin config output: {}",
            rendered_plugin_config_definition
        );
        Ok(rendered_plugin_config_definition)
    } else {
        println!("[BUNDLER] No plugin config found in task config, returning empty object");
        Ok(json!({}))
    }
}

fn extract_template_key_validations_from_schema(
    schema: Option<&JsonSchema>,
) -> HashMap<String, ValidationFieldType> {
    let mut template_key_validations = HashMap::new();

    if let Some(schema) = schema {
        if let Some(properties) = &schema.properties {
            for (property_name, property_schema) in properties {
                if let Some(validation) = &property_schema.x_any_validation {
                    template_key_validations
                        .insert(property_name.clone(), validation.r#type.clone());
                }
            }
        }
    }

    template_key_validations
}
