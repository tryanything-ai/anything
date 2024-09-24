use crate::supabase_auth_middleware::User;
use crate::AppState;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use uuid::Uuid;

use slugify::slugify;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMarketplaceFlowTemplateInput {
    flow_template_id: String,
    account_id: String,
    app_flow_id: String,
    flow_template_name: String,
    flow_template_description: String,
    public: bool,
    publisher_id: String,
    anonymous_publish: bool,
    slug: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMarketplaceFlowTemplateVersionInput {
    flow_template_version_id: Option<Uuid>,
    account_id: Uuid,
    flow_template_version_name: String,
    flow_definition: Value,
    public: bool,
    flow_template_version: String,
    publisher_id: Uuid,
    flow_template_id: Uuid,
    commit_message: Option<String>,
    app_flow_version_id: String,
}

pub async fn publish_workflow_to_marketplace(
    Path((account_id, workflow_id, workflow_version_id)): Path<(String, String, String)>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("[PUBLISH FLOW AS TEMPLATE] Starting publish workflow to marketplace");

    let anything_client = &state.anything_client;
    let marketplace_client = &state.marketplace_client;

    println!(
        "[PUBLISH FLOW AS TEMPLATE] Fetching workflow with ID: {}",
        workflow_id
    );
    //Get workflow
    let workflow_response = match anything_client
        .from("flows")
        .auth(user.jwt.clone())
        .select("*")
        .eq("flow_id", &workflow_id)
        .eq("account_id", &account_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to fetch workflow: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let workflow_body = match workflow_response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to read workflow response body: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let workflow: Value = match serde_json::from_str(&workflow_body) {
        Ok(item) => item,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to parse workflow JSON: {:?}",
                e
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    println!("[PUBLISH FLOW AS TEMPLATE] Got flow: {:?}", workflow);

    println!(
        "[PUBLISH FLOW AS TEMPLATE] Fetching workflow version with ID: {}",
        workflow_version_id
    );
    //Get Specified Version
    let workflow_version_response = match anything_client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .select("*")
        .eq("flow_version_id", &workflow_version_id)
        .eq("account_id", &account_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to fetch workflow version: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let workflow_version_body = match workflow_version_response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to read workflow version response body: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let workflow_version: Value = match serde_json::from_str(&workflow_version_body) {
        Ok(item) => item,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to parse workflow version JSON: {:?}",
                e
            );
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    println!(
        "[PUBLISH FLOW AS TEMPLATE] Got flow_versions: {:?}",
        workflow_version
    );

    if workflow.is_null() {
        println!("[PUBLISH FLOW AS TEMPLATE] Workflow is null");
        return (
            StatusCode::NOT_FOUND,
            "You must have a published workflow to publish as template",
        )
            .into_response();
    }

    if workflow_version.is_null() {
        println!("[PUBLISH FLOW AS TEMPLATE] Workflow version is null");
        return (
            StatusCode::NOT_FOUND,
            "You must have a published workflow version to publish as template",
        )
            .into_response();
    }

    println!("[PUBLISH FLOW AS TEMPLATE] Generating unique marketplace slug");
    //Generate Unique Slug
    let template_slug = generate_unique_marketplace_slug(
        &marketplace_client,
        workflow["flow_name"].as_str().unwrap(),
        user.jwt.as_str(),
    )
    .await;

    println!(
        "[PUBLISH FLOW AS TEMPLATE] Generated template slug: {}",
        template_slug.clone()
    );

    let mut flow_template_id = Uuid::new_v4().to_string();
    let mut marketplace_item = Value::Null;

    //If the flow has never been published before make a flow template
    if workflow["marketplace_flow_template_id"].is_null() {
        println!("[PUBLISH FLOW AS TEMPLATE] Flow has not been published before, creating new flow template");
        //Create an input for the marketplace template
        let input = CreateMarketplaceFlowTemplateInput {
            flow_template_id: flow_template_id.clone(),
            account_id: user.account_id.clone(), //Publishing as individual user
            app_flow_id: workflow_id.clone(),
            flow_template_name: workflow["flow_name"].as_str().unwrap().to_string(),
            flow_template_description: workflow["description"].as_str().unwrap().to_string(),
            public: true,
            publisher_id: user.account_id.clone(),
            anonymous_publish: false,
            slug: template_slug.clone(),
        };

        println!("[PUBLISH FLOW AS TEMPLATE] Creating marketplace template");
        //Create the marketplace template
        let marketplace_response = match marketplace_client
            .from("flow_templates")
            .auth(user.jwt.clone())
            .insert(serde_json::to_string(&input).unwrap())
            .execute()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!(
                    "[PUBLISH FLOW AS TEMPLATE] Failed to create marketplace template: {:?}",
                    e
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to execute request",
                )
                    .into_response();
            }
        };

        let marketplace_body = match marketplace_response.text().await {
            Ok(body) => body,
            Err(e) => {
                println!(
                    "[PUBLISH FLOW AS TEMPLATE] Failed to read marketplace response body: {:?}",
                    e
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read response body",
                )
                    .into_response();
            }
        };

        marketplace_item = match serde_json::from_str(&marketplace_body) {
            Ok(item) => item,
            Err(e) => {
                println!(
                    "[PUBLISH FLOW AS TEMPLATE] Failed to parse marketplace JSON: {:?}",
                    e
                );
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
            }
        };

        println!("[PUBLISH FLOW AS TEMPLATE] Updating flow with marketplace_flow_template_id");
        // Update the flow with the marketplace_flow_template_id
        let update_flow_input = json!({
            "marketplace_flow_template_id": flow_template_id
        });

        let update_flow_response = match anything_client
            .from("flows")
            .auth(user.jwt.clone())
            .eq("flow_id", workflow_id)
            .update(update_flow_input.to_string())
            .execute()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!("[PUBLISH FLOW AS TEMPLATE] Failed to update flow with marketplace_flow_template_id: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to update flow with marketplace_flow_template_id",
                )
                    .into_response();
            }
        };

        let _update_flow_body = match update_flow_response.text().await {
            Ok(body) => body,
            Err(e) => {
                println!(
                    "[PUBLISH FLOW AS TEMPLATE] Failed to read update flow response body: {:?}",
                    e
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to read update flow response body",
                )
                    .into_response();
            }
        };
    } else {
        println!("[PUBLISH FLOW AS TEMPLATE] Flow has been published before, using existing flow_template_id");
        flow_template_id = workflow["marketplace_flow_template_id"]
            .as_str()
            .unwrap()
            .to_string();
    }

    println!("[PUBLISH FLOW AS TEMPLATE] Creating flow version template");
    // Create the flow version template
    let flow_version_template_input = CreateMarketplaceFlowTemplateVersionInput {
        flow_template_version_id: Some(Uuid::parse_str(&workflow_version_id).unwrap()),
        account_id: user.account_id.parse().unwrap(),
        flow_template_version_name: "0.1.0".to_string(),
        flow_definition: workflow_version["flow_definition"].clone(),
        public: true,
        flow_template_version: "0.1.0".to_string(),
        publisher_id: user.account_id.parse().unwrap(),
        flow_template_id: flow_template_id.parse().unwrap(),
        commit_message: None,
        app_flow_version_id: workflow_version_id.clone(),
    };

    let flow_version_response = match marketplace_client
        .from("flow_template_versions")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&flow_version_template_input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to create flow version template: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create flow version template",
            )
                .into_response();
        }
    };

    let flow_version_body = match flow_version_response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to read flow version response body: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read flow version response body",
            )
                .into_response();
        }
    };

    let flow_version_item: Value = match serde_json::from_str(&flow_version_body) {
        Ok(item) => item,
        Err(e) => {
            println!(
                "[PUBLISH FLOW AS TEMPLATE] Failed to parse flow version JSON: {:?}",
                e
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to parse flow version JSON",
            )
                .into_response();
        }
    };

    println!("[PUBLISH FLOW AS TEMPLATE] Combining response");
    // Combine the marketplace item and flow version item
    let combined_response = json!({
        "flow_template": marketplace_item[0],
        "flow_template_version": flow_version_item[0],
        "marketplace_url": format!("https://tryanything.xyz/templates/{}", template_slug.clone())
    });

    println!("[PUBLISH FLOW AS TEMPLATE] Publishing complete, returning response");
    Json(combined_response).into_response()
}

// Workflows
pub async fn get_marketplace_workflows(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[MARKETPLACE] Fetching workflow templates");

    let response = match client
        .from("flow_templates")
        .select("*, flow_template_versions(*), tags(*), profiles(*)")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[MARKETPLACE] Failed to execute request: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[MARKETPLACE] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(e) => {
            println!("[MARKETPLACE] Failed to parse JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    // println!("[MARKETPLACE] Query result: {:?}", items);

    Json(items).into_response()
}

pub async fn get_marketplace_workflow_by_slug(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[MARKETPLACE] Fetching workflow template by slug: {}", slug);

    let response = match client
        .from("flow_templates")
        .select("*, flow_template_versions(*), tags(*), profiles(*)")
        .eq("slug", &slug)
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[MARKETPLACE] Failed to execute request: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response();
        }
    };

    let body = match response.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("[MARKETPLACE] Failed to read response body: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response();
        }
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(e) => {
            println!("[MARKETPLACE] Failed to parse JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    if let Some(workflow) = items.as_array().and_then(|arr| arr.first()) {
        println!("[MARKETPLACE] Found workflow: {:?}", workflow);
        Json(workflow.clone()).into_response()
    } else {
        println!("[MARKETPLACE] No workflow found for slug: {}", slug);
        (StatusCode::NOT_FOUND, "Workflow not found").into_response()
    }
}

pub async fn generate_unique_marketplace_slug(
    client: &Postgrest,
    base_slug: &str,
    user_jwt: &str,
) -> String {
    let mut slug = slugify!(base_slug);
    let mut counter = 1;

    //never go over 100. just like sanity check.
    for _ in 0..100 {
        let response = match client
            .from("marketplace_templates")
            .select("slug")
            .eq("slug", &slug)
            .auth(user_jwt)
            .execute()
            .await
        {
            Ok(response) => response,
            Err(_) => return slug, // If there's an error, assume the slug is unique
        };

        let body = match response.text().await {
            Ok(body) => body,
            Err(_) => return slug, // If there's an error reading the body, assume the slug is unique
        };

        let existing_slugs: Vec<Value> = match serde_json::from_str(&body) {
            Ok(items) => items,
            Err(_) => return slug, // If there's an error parsing the JSON, assume the slug is unique
        };

        if existing_slugs.is_empty() {
            break;
        }

        slug = slugify!(format!("{}-{}", base_slug, counter).as_str());
        counter += 1;
    }

    slug
}
