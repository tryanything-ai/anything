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
use serde_json::Value;
use std::sync::Arc;

use uuid::Uuid;

use slugify::slugify;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMarketplaceTemplateInput {
    flow_template_id: String,
    account_id: String,
    flow_template_name: String,
    flow_template_description: String,
    public: bool,
    publisher_id: String,
    anonymous_publish: bool,
    slug: String,
}

pub async fn publish_workflow_to_marketplace(
    Path(workflow_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    println!("Handling publish workflow to marketplace");

    let anything_client = &state.marketplace_client;
    let marketplace_client = &state.marketplace_client;

    //Get latest published flow_version for this workflow
    let response = match anything_client
        .from("flow_versions")
        .auth(user.jwt.clone())
        .select("*,flows(*)")
        .eq("flow_id", &workflow_id)
        .eq("published", "true")
        .order("created_at.desc")
        .limit(1)
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let workflow: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    println!("Got flow_versions: {:?}", workflow);

    if workflow.is_null() {
        return (
            StatusCode::NOT_FOUND,
            "You must have a published workflow to publish as template",
        )
            .into_response();
    }

    //Generate Unique Slug
    let template_slug = generate_unique_marketplace_slug(
        &marketplace_client,
        workflow["flows"][0]["name"].as_str().unwrap(),
        user.jwt.as_str(),
    )
    .await;

    println!("Template slug: {}", template_slug);

    //Create an input for the marketplace template
    let input = CreateMarketplaceTemplateInput {
        flow_template_id: Uuid::new_v4().to_string(),
        account_id: user.account_id.clone(),
        flow_template_name: workflow["flows"][0]["name"].as_str().unwrap().to_string(),
        flow_template_description: workflow["flows"][0]["description"]
            .as_str()
            .unwrap()
            .to_string(),
        public: true,
        publisher_id: user.account_id.clone(),
        anonymous_publish: false,
        slug: template_slug,
    };

    //Create the marketplace template
    let marketplace_response = match marketplace_client
        .from("flow_templates")
        .auth(user.jwt.clone())
        .insert(serde_json::to_string(&input).unwrap())
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to execute request",
            )
                .into_response()
        }
    };

    let marketplace_body = match marketplace_response.text().await {
        Ok(body) => body,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read response body",
            )
                .into_response()
        }
    };

    let marketplace_item: Value = match serde_json::from_str(&marketplace_body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(marketplace_item).into_response()
}

// Actions
pub async fn get_marketplace_workflows(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[MARKETPLACE] Fetching wo templates");

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

    println!("[MARKETPLACE] Query result: {:?}", items);

    Json(items).into_response()
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
