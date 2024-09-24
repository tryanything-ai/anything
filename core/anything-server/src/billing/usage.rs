use crate::AppState;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::Value;
use std::sync::Arc;

use crate::supabase_auth_middleware::User;

pub async fn get_account_billing_status(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(account_id): Path<String>,
) -> impl IntoResponse {
    let client = &state.anything_client;

    let response = match client
        .from("accounts_billing")
        .auth(&user.jwt) // Pass a reference to the JWT
        .select("*")
        .eq("account_id", account_id)
        .single()
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

    let item: Value = match serde_json::from_str(&body) {
        Ok(item) => item,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response()
        }
    };

    Json(item).into_response()
}
