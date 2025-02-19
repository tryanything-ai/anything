use crate::system_plugins::registry::load_schema_templates;
use crate::types::react_flow_types::Edge;
use crate::types::workflow_types::WorkflowVersionDefinition;
use serde_json::Value;


pub fn create_workflow_from_template(template_id: Option<String>) -> Result<WorkflowVersionDefinition, Box<dyn std::error::Error>> {
    match template_id {
        Some(id) => match id.as_str() {
            "webhook" => create_webhook_js_workflow(),
            "tool" => create_tool_workflow(),
            "cron" => create_cron_http_workflow(),
            "input_output" => create_input_output_workflow(),
            _ => Err("Invalid template ID".into())
        },
        None => create_cron_http_workflow()
    }
}

pub fn create_cron_http_workflow() -> Result<WorkflowVersionDefinition, Box<dyn std::error::Error>>
{
    // Load all templates from registry
    let templates = load_schema_templates()?;

    // Find cron trigger template
    let cron_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/cron")
        .ok_or("Cron template not found")?;

    // Find http action template
    let http_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/http")
        .ok_or("HTTP template not found")?;

    // Convert templates to Actions and set positions
    let mut cron_action: Value = cron_template["action_template_definition"].clone();
    cron_action["presentation"]["position"]["x"] = serde_json::json!(300);
    cron_action["presentation"]["position"]["y"] = serde_json::json!(100);

    let mut http_action: Value = http_template["action_template_definition"].clone();
    http_action["presentation"]["position"]["x"] = serde_json::json!(300);
    http_action["presentation"]["position"]["y"] = serde_json::json!(250);

    // Create edge connecting them
    let edge = Edge {
        id: "cron->http".to_string(),
        r#type: "anything".to_string(),
        source: "cron".to_string(),
        target: "http".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    // Create workflow definition
    let workflow = WorkflowVersionDefinition {
        actions: vec![
            serde_json::from_value(cron_action)?,
            serde_json::from_value(http_action)?,
        ],
        edges: vec![edge],
    };

    Ok(workflow)
}

pub fn create_webhook_js_workflow() -> Result<WorkflowVersionDefinition, Box<dyn std::error::Error>>
{
    // Load all templates from registry
    let templates = load_schema_templates()?;

    // Find webhook trigger template
    let webhook_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/webhook")
        .ok_or("Webhook template not found")?;

    // Find javascript action template
    let js_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/javascript")
        .ok_or("JavaScript template not found")?;

    // Find response action template
    let response_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/response")
        .ok_or("Response template not found")?;

    // Convert templates to Actions and set positions
    let mut webhook_action: Value = webhook_template["action_template_definition"].clone();
    webhook_action["presentation"]["position"]["x"] = serde_json::json!(300);
    webhook_action["presentation"]["position"]["y"] = serde_json::json!(100);

    let mut js_action: Value = js_template["action_template_definition"].clone();
    js_action["presentation"]["position"]["x"] = serde_json::json!(300);
    js_action["presentation"]["position"]["y"] = serde_json::json!(250);

    let mut response_action: Value = response_template["action_template_definition"].clone();
    response_action["presentation"]["position"]["x"] = serde_json::json!(300);
    response_action["presentation"]["position"]["y"] = serde_json::json!(400);

    // Create edges connecting them
    let webhook_to_js = Edge {
        id: "webhook->js".to_string(),
        r#type: "anything".to_string(),
        source: "webhook".to_string(),
        target: "javascript".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    let js_to_response = Edge {
        id: "js->response".to_string(),
        r#type: "anything".to_string(),
        source: "javascript".to_string(),
        target: "response".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    // Create workflow definition
    let workflow = WorkflowVersionDefinition {
        actions: vec![
            serde_json::from_value(webhook_action)?,
            serde_json::from_value(js_action)?,
            serde_json::from_value(response_action)?,
        ],
        edges: vec![webhook_to_js, js_to_response],
    };

    Ok(workflow)
}

pub fn create_input_output_workflow() -> Result<WorkflowVersionDefinition, Box<dyn std::error::Error>> {
    // Load all templates from registry
    let templates = load_schema_templates()?;
    
    // Find input action template
    let input_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/input")
        .ok_or("Input template not found")?;

    // Find http action template
    let http_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/http")
        .ok_or("HTTP template not found")?;

    // Find javascript action template
    let js_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/javascript")
        .ok_or("JavaScript template not found")?;

    // Find output action template
    let output_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/output")
        .ok_or("Output template not found")?;

    // Convert templates to Actions and set positions
    let mut input_action: Value = input_template["action_template_definition"].clone();
    input_action["presentation"]["position"]["x"] = serde_json::json!(300);
    input_action["presentation"]["position"]["y"] = serde_json::json!(100);

    let mut http_action: Value = http_template["action_template_definition"].clone();
    http_action["presentation"]["position"]["x"] = serde_json::json!(300);
    http_action["presentation"]["position"]["y"] = serde_json::json!(250);

    let mut js_action: Value = js_template["action_template_definition"].clone();
    js_action["presentation"]["position"]["x"] = serde_json::json!(300);
    js_action["presentation"]["position"]["y"] = serde_json::json!(400);

    let mut output_action: Value = output_template["action_template_definition"].clone();
    output_action["presentation"]["position"]["x"] = serde_json::json!(300);
    output_action["presentation"]["position"]["y"] = serde_json::json!(550);

    // Create edges connecting them
    let input_to_http = Edge {
        id: "input->http".to_string(),
        r#type: "anything".to_string(),
        source: "input".to_string(),
        target: "http".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    let http_to_js = Edge {
        id: "http->js".to_string(),
        r#type: "anything".to_string(),
        source: "http".to_string(),
        target: "javascript".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    let js_to_output = Edge {
        id: "js->output".to_string(),
        r#type: "anything".to_string(),
        source: "javascript".to_string(),
        target: "output".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    // Create workflow definition
    let workflow = WorkflowVersionDefinition {
        actions: vec![
            serde_json::from_value(input_action)?,
            serde_json::from_value(http_action)?,
            serde_json::from_value(js_action)?,
            serde_json::from_value(output_action)?,
        ],
        edges: vec![input_to_http, http_to_js, js_to_output],
    };

    Ok(workflow)
}


pub fn create_tool_workflow() -> Result<WorkflowVersionDefinition, Box<dyn std::error::Error>> {
    // Load all templates from registry
    let templates = load_schema_templates()?;
    
    // Find input action template
    let input_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/agent_tool_call")
        .ok_or("Input template not found")?;

    // Find http action template
    // let http_template = templates
    //     .iter()
    //     .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/http")
    //     .ok_or("HTTP template not found")?;

    // Find javascript action template
    // let js_template = templates
    //     .iter()
    //     .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/javascript")
    //     .ok_or("JavaScript template not found")?;

    // Find output action template
    let output_template = templates
        .iter()
        .find(|t| t["action_template_definition"]["plugin_name"] == "@anything/agent_tool_call_response")
        .ok_or("Output template not found")?;

    // Convert templates to Actions and set positions
    let mut input_action: Value = input_template["action_template_definition"].clone();
    input_action["presentation"]["position"]["x"] = serde_json::json!(300);
    input_action["presentation"]["position"]["y"] = serde_json::json!(100);

    // let mut http_action: Value = http_template["action_template_definition"].clone();
    // http_action["presentation"]["position"]["x"] = serde_json::json!(300);
    // http_action["presentation"]["position"]["y"] = serde_json::json!(250);

    // let mut js_action: Value = js_template["action_template_definition"].clone();
    // js_action["presentation"]["position"]["x"] = serde_json::json!(300);
    // js_action["presentation"]["position"]["y"] = serde_json::json!(400);

    let mut output_action: Value = output_template["action_template_definition"].clone();
    output_action["presentation"]["position"]["x"] = serde_json::json!(300);
    output_action["presentation"]["position"]["y"] = serde_json::json!(550);

    // Create edges connecting them
    let input_to_http = Edge {
        id: "agent_tool_call->agent_tool_call_response".to_string(),
        r#type: "anything".to_string(),
        source: "agent_tool_call".to_string(),
        target: "agent_tool_call_response".to_string(),
        source_handle: Some("b".to_string()),
        target_handle: Some("a".to_string()),
    };

    // let http_to_js = Edge {
    //     id: "http->js".to_string(),
    //     r#type: "anything".to_string(),
    //     source: "http".to_string(),
    //     target: "javascript".to_string(),
    //     source_handle: Some("b".to_string()),
    //     target_handle: Some("a".to_string()),
    // };

    // let js_to_output = Edge {
    //     id: "js->agent_tool_call_response".to_string(),
    //     r#type: "anything".to_string(),
    //     source: "javascript".to_string(),
    //     target: "agent_tool_call_response".to_string(),
    //     source_handle: Some("b".to_string()),
    //     target_handle: Some("a".to_string()),
    // };

    // Create workflow definition
    let workflow = WorkflowVersionDefinition {
        actions: vec![
            serde_json::from_value(input_action)?,
            // serde_json::from_value(http_action)?,
            // serde_json::from_value(js_action)?,
            serde_json::from_value(output_action)?,
        ],
        edges: vec![input_to_http],
    };

    Ok(workflow)
}
