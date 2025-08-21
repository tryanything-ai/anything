use crate::AppState;
use crate::custom_auth::User;
use crate::entities::accounts_billing;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_account_billing_status(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    println!("Handling get_account_billing_status for account: {}", account_id);

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Get billing status using SeaORM
    let billing_status = match accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(billing)) => billing,
        Ok(None) => {
            // No billing record found - create default response
            let default_response = json!({
                "account_id": account_id,
                "billing_status": "no_billing_setup",
                "plan": "free",
                "usage": {
                    "tasks_used": 0,
                    "tasks_limit": 100,
                    "storage_used": 0,
                    "storage_limit": 1024
                },
                "active": false
            });
            return Json(default_response).into_response();
        }
        Err(err) => {
            println!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    // Convert to response format
    let response = json!({
        "account_id": billing_status.account_id,
        "billing_status": billing_status.billing_status,
        "plan": billing_status.plan,
        "stripe_customer_id": billing_status.stripe_customer_id,
        "stripe_subscription_id": billing_status.stripe_subscription_id,
        "active": billing_status.active,
        "created_at": billing_status.created_at,
        "updated_at": billing_status.updated_at,
        "usage": {
            "tasks_used": billing_status.tasks_used.unwrap_or(0),
            "tasks_limit": billing_status.tasks_limit.unwrap_or(100),
            "storage_used": billing_status.storage_used.unwrap_or(0),
            "storage_limit": billing_status.storage_limit.unwrap_or(1024)
        }
    });

    Json(response).into_response()
}
