use crate::system_variables::get_system_variables;
use crate::types::json_schema::JsonSchema;
use crate::types::task_types::Task;

use crate::AppState;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Order};

use crate::bundler::accounts::fetch_cached_auth_accounts;
use crate::bundler::secrets::get_decrypted_secrets;
use crate::files::utils::get_files;
use crate::templater::{utils::get_template_file_requirements, Templater};
use crate::types::task_types::TaskStatus;
use crate::entities::tasks;

use crate::types::json_schema::ValidationField;

pub async fn bundle_tasks_cached_context(
    state: Arc<AppState>,
    task: &Task,
    refresh_auth: bool,
) -> Result<(Value, Value), Box<dyn Error + Send + Sync>> {
    bundle_tasks_cached_context_with_tasks(state, task, refresh_auth, None).await
}

pub async fn bundle_tasks_cached_context_with_tasks(
    state: Arc<AppState>,
    task: &Task,
    refresh_auth: bool,
    in_memory_tasks: Option<&HashMap<Uuid, Task>>,
) -> Result<(Value, Value), Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] Starting to bundle context from parts");

    let rendered_inputs_definition =
        bundle_tasks_cached_inputs_with_tasks(state, task, refresh_auth, in_memory_tasks)
            .await?;

    let plugin_config = task.config.plugin_config.as_ref();
    let plugin_config_schema = task.config.plugin_config_schema.as_ref();

    let rendered_plugin_config_definition = bundle_plugin_config(
        rendered_inputs_definition.clone(),
        plugin_config,
        plugin_config_schema,
    )?;

    Ok((
        rendered_inputs_definition,
        rendered_plugin_config_definition,
    ))
}

pub async fn bundle_tasks_cached_inputs(
    state: Arc<AppState>,
    task: &Task,
    refresh_auth: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    bundle_tasks_cached_inputs_with_tasks(state, task, refresh_auth, None).await
}

pub async fn bundle_tasks_cached_inputs_with_tasks(
    state: Arc<AppState>,
    task: &Task,
    refresh_auth: bool,
    in_memory_tasks: Option<&HashMap<Uuid, Task>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] Starting to bundle context from parts");

    let account_id = task.account_id.to_string();
    let flow_session_id = task.flow_session_id.to_string();
    let inputs = task.config.inputs.as_ref();
    let inputs_schema = task.config.inputs_schema.as_ref();

    let rendered_inputs_definition = bundle_cached_inputs_with_tasks(
        state,
        &account_id,
        &flow_session_id,
        inputs,
        inputs_schema,
        refresh_auth,
        in_memory_tasks,
    )
    .await?;

    Ok(rendered_inputs_definition)
}

pub async fn bundle_context_from_parts(
    state: Arc<AppState>,
    account_id: &str,
    flow_session_id: &str,
    inputs: Option<&Value>,
    inputs_schema: Option<&JsonSchema>,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&JsonSchema>,
    refresh_auth: bool,
) -> Result<(Value, Value), Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] Starting to bundle context from parts");

    let rendered_inputs_definition = bundle_cached_inputs_with_tasks(
        state,
        account_id,
        flow_session_id,
        inputs,
        inputs_schema,
        refresh_auth,
        None,
    )
    .await?;

    let rendered_plugin_config_definition = bundle_plugin_config(
        rendered_inputs_definition.clone(),
        plugin_config,
        plugin_config_schema,
    )?;

    Ok((
        rendered_inputs_definition,
        rendered_plugin_config_definition,
    ))
}

pub async fn bundle_cached_inputs(
    state: Arc<AppState>,
    account_id: &str,
    workflow_id: &str,
    workflow_version_id: &str,
    action_id: &str,
    inputs: Option<&Value>,
    inputs_schema: Option<&JsonSchema>,
    context: Value,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] bundle_cached_inputs using SeaORM");
    
    // TODO: This function needs the workflow definition to properly bundle inputs
    // For now, return a placeholder that includes the context
    let bundled_result = json!({
        "message": "bundle_cached_inputs converted to SeaORM",
        "account_id": account_id,
        "workflow_id": workflow_id,
        "workflow_version_id": workflow_version_id,
        "action_id": action_id,
        "context": context,
        "inputs": inputs.unwrap_or(&json!({})),
        "status": "seaorm_placeholder"
    });

    Ok(bundled_result)
}

pub async fn bundle_cached_inputs_with_tasks(
    state: Arc<AppState>,
    account_id: &str,
    flow_session_id: &str,
    inputs: Option<&Value>,
    inputs_schema: Option<&JsonSchema>,
    refresh_auth: bool,
    in_memory_tasks: Option<&HashMap<Uuid, Task>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] bundle_cached_inputs_with_tasks using SeaORM");

    let flow_session_uuid = Uuid::parse_str(flow_session_id)?;

    // Get completed tasks - use in-memory when available, database as fallback
    let database_tasks = if in_memory_tasks.is_none() {
        // Fallback to database fetch when no in-memory tasks are available
        println!("[BUNDLER SEAORM] Fetching completed tasks from database (fallback)");
        
        // TODO: Complete Task entity mapping once task struct definition is finalized
        // For now, return empty vec as placeholder to match original bundler approach
        let tasks: Vec<Task> = Vec::new();
        Some(tasks)
    } else {
        None
    };

    // Get secrets
    let secrets = get_decrypted_secrets(state.clone(), account_id).await?;
    println!("[BUNDLER SEAORM] Retrieved {} secrets", secrets.len());

    // Get accounts
    let auth_accounts = fetch_cached_auth_accounts(state.clone(), account_id, refresh_auth).await?;
    println!("[BUNDLER SEAORM] Retrieved {} auth accounts", auth_accounts.len());

    // Get files
    let files = match get_files(state.clone(), account_id, Vec::new()).await {
        Ok(files) => files,
        Err(e) => {
            println!("[BUNDLER SEAORM] Warning: Failed to get files: {:?}", e);
            Vec::new()
        }
    };
    println!("[BUNDLER SEAORM] Retrieved {} files", files.len());

    // Get system variables
    let system_variables = get_system_variables();
    println!("[BUNDLER SEAORM] Retrieved {} system variables", system_variables.len());

    // Bundle the inputs
    let mut data = json!({
        "secrets": secrets,
        "auth_accounts": auth_accounts,
        "files": files,
        "system_variables": system_variables,
    });

    // Add tasks from in-memory or database
    if let Some(in_memory_tasks) = in_memory_tasks {
        let task_outputs: HashMap<String, Value> = in_memory_tasks
            .values()
            .filter_map(|task| {
                if task.task_status == TaskStatus::Completed {
                    task.result.as_ref().map(|output| (task.action_id.clone(), output.clone()))
                } else {
                    None
                }
            })
            .collect();
        data["tasks"] = json!(task_outputs);
    } else if let Some(db_tasks) = database_tasks {
        let task_outputs: HashMap<String, Value> = db_tasks
            .into_iter()
            .filter_map(|task| {
                if task.task_status == TaskStatus::Completed {
                    task.result.map(|output| (task.action_id, output))
                } else {
                    None
                }
            })
            .collect();
        data["tasks"] = json!(task_outputs);
    }

    // Apply templating if inputs and schema are provided
    if let Some(inputs) = inputs {
        let mut templater = Templater::new();
        templater.add_template("task_inputs_definition", inputs.clone());

        let template_file_requirements = get_template_file_requirements(inputs)?;
        for requirement in template_file_requirements {
            println!("[BUNDLER SEAORM] Adding template file requirement: {:?}", requirement);
            // TODO: Get template file content using SeaORM
        }

        let input_validations = extract_template_key_validations_from_schema(inputs_schema);
        let rendered_inputs = templater.render("task_inputs_definition", &data, input_validations)?;
        Ok(rendered_inputs)
    } else {
        // Return the bundled data if no templating is needed
        Ok(data)
    }
}

pub fn bundle_plugin_config(
    rendered_inputs_definition: Value,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&JsonSchema>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] Starting to bundle plugin config");

    let mut render_input_context: HashMap<String, Value> = HashMap::new();
    render_input_context.insert("inputs".to_string(), rendered_inputs_definition);

    // Create a new Templater instance for rendering inputs
    let mut templater = Templater::new();

    // Convert context HashMap to Value
    let inputs_context_value = serde_json::to_value(render_input_context.clone())?;

    // Add the task definition as a template and render if it exists
    if let Some(plugin_config) = plugin_config {
        println!(
            "[BUNDLER SEAORM] Task plugin config definition: {}",
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
        Ok(rendered_plugin_config_definition)
    } else {
        println!("[BUNDLER SEAORM] No plugin config found in task config, returning empty object");
        Ok(json!({}))
    }
}

fn extract_template_key_validations_from_schema(
    schema: Option<&JsonSchema>,
) -> HashMap<String, ValidationField> {
    let mut template_key_validations = HashMap::new();

    if let Some(schema) = schema {
        if let Some(properties) = &schema.properties {
            for (property_name, property_schema) in properties {
                if let Some(validation) = &property_schema.x_any_validation {
                    template_key_validations.insert(property_name.clone(), validation.clone());
                }
            }
        }
    }

    template_key_validations
}
