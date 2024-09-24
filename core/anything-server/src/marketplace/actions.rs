use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::supabase_auth_middleware::User;

use crate::AppState;
use std::env;
use uuid::Uuid;

use crate::marketplace::workflows::generate_unique_marketplace_slug;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateMarketplaceActionTemplateInput {
    action_template_id: String,
    account_id: String,
    app_action_template_id: Option<String>,
    action_template_name: String,
    action_template_description: Option<String>,
    action_template_definition: Value,
    public: bool,
    r#type: String,
    publisher_id: String,
    anonymous_publish: bool,
    slug: String,
    archived: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAppActionTemplateInput {
    action_template_id: String,
    account_id: String,
    marketplace_action_template_id: Option<String>,
    action_template_name: String,
    action_template_description: Option<String>,
    action_template_definition: Value,
    r#type: String,
    archived: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishActionTemplateInput {
    publish_to_team: bool,
    publish_to_marketplace: bool,
    publish_to_marketplace_anonymously: bool,
    action_template_definition: Value,
}

// Actions
pub async fn get_actions_from_marketplace(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let client = &state.marketplace_client;

    println!("[ACTION-TEMPLATES] Fetching action templates");

    let response = match client
        .from("action_templates")
        .select("*")
        .order("action_template_name.desc")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            println!("[ACTIONS] Failed to execute request: {:?}", e);
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
            println!("[ACTIONS] Failed to read response body: {:?}", e);
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
            println!("[ACTIONS] Failed to parse JSON: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response();
        }
    };

    println!("[ACTIONS] Query result: {:?}", items);

    Json(items).into_response()
}

pub async fn publish_action_template(
    Path(account_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(payload): Json<PublishActionTemplateInput>,
) -> impl IntoResponse {
    println!("Handling publish workflow to marketplace");

    let anything_client = &state.anything_client;

    let mut response = json!({
        "app_template": null,
        "marketplace_template": null
    });

    let app_action_template_id = Uuid::new_v4().to_string();
    let marketplace_action_template_id = app_action_template_id.clone();

    //Publish to app only if requested
    if payload.publish_to_team {
        //Create an input for the app template
        let app_input = CreateAppActionTemplateInput {
            action_template_id: app_action_template_id.clone(),
            marketplace_action_template_id: if payload.publish_to_marketplace {
                Some(marketplace_action_template_id.clone())
            } else {
                None
            },
            account_id: account_id.clone(),
            action_template_name: payload.action_template_definition["label"]
                .as_str()
                .unwrap()
                .to_string(),
            action_template_description: payload.action_template_definition["description"]
                .as_str()
                .map(|s| s.to_string()),
            action_template_definition: payload.action_template_definition.clone(),
            r#type: payload.action_template_definition["type"]
                .as_str()
                .unwrap()
                .to_string(),
            archived: Some(false),
        };

        //Create the app template
        let app_response = match anything_client
            .from("action_templates")
            .auth(user.jwt.clone())
            .insert(serde_json::to_string(&app_input).unwrap())
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

        let app_template: Value =
            serde_json::from_str(&app_response.text().await.unwrap()).unwrap();
        response["app_template"] = json!(app_template);
    }

    // Publish to marketplace if requested
    if payload.publish_to_marketplace {
        let marketplace_client = &state.marketplace_client;

        //fetch the marketplace profile for the jwt user
        let marketplace_profile = match marketplace_client
            .from("profiles")
            .auth(user.jwt.clone())
            .select("*")
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

        let profile: Value =
            serde_json::from_str(&marketplace_profile.text().await.unwrap()).unwrap();
        let publisher_id = profile[0]["profile_id"].as_str().unwrap().to_string();

        //Generate Unique Slug
        let action_template_slug = generate_unique_marketplace_slug(
            &marketplace_client,
            payload.action_template_definition["label"]
                .as_str()
                .unwrap(),
            user.jwt.as_str(),
        )
        .await;

        //Create an input for the marketplace template
        let marketplace_input = CreateMarketplaceActionTemplateInput {
            action_template_id: Uuid::new_v4().to_string(),
            account_id: account_id.clone(),
            app_action_template_id: if payload.publish_to_team {
                Some(marketplace_action_template_id.clone())
            } else {
                None
            },
            action_template_name: payload.action_template_definition["label"]
                .as_str()
                .unwrap()
                .to_string(),
            action_template_description: payload.action_template_definition["description"]
                .as_str()
                .map(|s| s.to_string()),
            action_template_definition: payload.action_template_definition.clone(),
            public: true,
            r#type: payload.action_template_definition["type"]
                .as_str()
                .unwrap()
                .to_string(),
            publisher_id: publisher_id,
            anonymous_publish: payload.publish_to_marketplace_anonymously,
            slug: action_template_slug,
            archived: Some(false),
        };

        println!("Template slug: {}", &marketplace_input.slug);

        //Create the marketplace template
        let marketplace_response = match marketplace_client
            .from("action_templates")
            .auth(user.jwt.clone())
            .insert(serde_json::to_string(&marketplace_input).unwrap())
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

        let marketplace_template: Value =
            serde_json::from_str(&marketplace_response.text().await.unwrap()).unwrap();
        response["marketplace_template"] = json!(marketplace_template);
    }

    Json(response).into_response()
}
