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
    inputs_schema: Option<&Vec<ValidationField>>,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&Vec<ValidationField>>,
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
    inputs_schema: Option<&Vec<ValidationField>>,
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
    inputs_schema: Option<&Vec<ValidationField>>,
    refresh_auth: bool,
    in_memory_tasks: Option<&HashMap<Uuid, Task>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] bundle_cached_inputs_with_tasks using SeaORM");

    let flow_session_uuid = Uuid::parse_str(flow_session_id)?;

    // Get tasks from database using SeaORM
    let database_tasks = if in_memory_tasks.is_none() {
        let task_models = tasks::Entity::find()
            .filter(tasks::Column::FlowSessionId.eq(flow_session_uuid))
            .filter(tasks::Column::TaskStatus.eq("completed"))
            .order_by(tasks::Column::CreatedAt, Order::Asc)
            .all(&*state.db)
            .await?;

        // Convert to Task structs
        let mut task_list = Vec::new();
        for task_model in task_models {
            let task = Task {
                task_id: task_model.task_id,
                account_id: task_model.account_id,
                flow_id: task_model.flow_id,
                flow_version_id: task_model.flow_version_id,
                flow_session_id: task_model.flow_session_id,
                action_id: task_model.action_id,
                action_label: task_model.action_label,
                r#type: task_model.r#type,
                plugin_name: task_model.plugin_name,
                plugin_version: task_model.plugin_version,
                stage: task_model.stage,
                config: task_model.config,
                output: task_model.output,
                context: task_model.context,
                error_message: task_model.error_message,
                retry_count: task_model.retry_count,
                max_retries: task_model.max_retries,
                trigger_session_id: task_model.trigger_session_id,
                trigger_session_status: task_model.trigger_session_status,
                trigger_id: task_model.trigger_id,
                flow_session_status: task_model.flow_session_status,
                task_status: task_model.task_status,
                parent_task_id: task_model.parent_task_id,
                assigned_worker_id: task_model.assigned_worker_id,
                started_at: task_model.started_at,
                completed_at: task_model.completed_at,
                created_at: task_model.created_at,
                updated_at: task_model.updated_at,
                created_by: task_model.created_by,
                updated_by: task_model.updated_by,
                execution_time_ms: task_model.execution_time_ms,
                current_step: task_model.current_step,
                total_steps: task_model.total_steps,
                progress_percentage: task_model.progress_percentage,
            };
            task_list.push(task);
        }
        Some(task_list)
    } else {
        None
    };

    // Get secrets
    let secrets = get_decrypted_secrets(state.clone(), account_id, refresh_auth).await?;
    println!("[BUNDLER SEAORM] Retrieved {} secrets", secrets.len());

    // Get accounts
    let auth_accounts = fetch_cached_auth_accounts(state.clone(), account_id, refresh_auth).await?;
    println!("[BUNDLER SEAORM] Retrieved {} auth accounts", auth_accounts.len());

    // Get files
    // TODO: Convert get_files to SeaORM when files module is updated
    let files = match get_files(state.clone(), account_id).await {
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
                    task.output.as_ref().map(|output| (task.action_id.clone(), output.clone()))
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
                if task.task_status == "completed" {
                    task.output.map(|output| (task.action_id, output))
                } else {
                    None
                }
            })
            .collect();
        data["tasks"] = json!(task_outputs);
    }

    // Apply templating if inputs and schema are provided
    if let (Some(inputs), Some(inputs_schema)) = (inputs, inputs_schema) {
        let mut templater = Templater::new();
        templater.add_context("data", data);

        let template_file_requirements = get_template_file_requirements(inputs)?;
        for requirement in template_file_requirements {
            println!("[BUNDLER SEAORM] Adding template file requirement: {}", requirement);
            // TODO: Get template file content using SeaORM
        }

        let rendered_inputs = templater.render_json_schema(inputs_schema, Some(inputs))?;
        Ok(rendered_inputs)
    } else {
        // Return the bundled data if no templating is needed
        Ok(data)
    }
}

pub fn bundle_plugin_config(
    rendered_inputs_definition: Value,
    plugin_config: Option<&Value>,
    plugin_config_schema: Option<&Vec<ValidationField>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("[BUNDLER SEAORM] Starting to bundle plugin config");

    if let (Some(config), Some(schema)) = (plugin_config, plugin_config_schema) {
        let mut templater = Templater::new();
        templater.add_context("data", rendered_inputs_definition);

        let rendered_config = templater.render_json_schema(schema, Some(config))?;
        Ok(rendered_config)
    } else {
        Ok(json!({}))
    }
}
