use crate::AppState;
use crate::entities::{accounts_billing, users};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookPayload<T> {
    #[serde(flatten)]
    pub event_type: WebhookEventType<T>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum WebhookEventType<T> {
    #[serde(rename = "INSERT")]
    Insert {
        table: String,
        schema: String,
        record: T,
        old_record: Option<()>,
    },
    #[serde(rename = "UPDATE")]
    Update {
        table: String,
        schema: String,
        record: T,
        old_record: T,
    },
    #[serde(rename = "DELETE")]
    Delete {
        table: String,
        schema: String,
        record: Option<()>,
        old_record: T,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableRecord {
    pub account_id: Uuid,
    pub account_name: String,
    pub slug: Option<String>,
    pub personal_account: bool,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_by: Option<uuid::Uuid>,
    pub updated_by: Option<uuid::Uuid>,
    pub private_metadata: serde_json::Value,
    pub public_metadata: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpsertCustomerSubscriptionInput {
    pub account_id: Uuid,
    pub stripe_customer_id: String,
    pub stripe_subscription_id: Option<String>,
    pub plan: Option<String>,
    pub active: bool,
}

pub async fn accounts_webhook_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WebhookPayload<TableRecord>>,
) -> impl IntoResponse {
    println!("[BILLING ACCOUNTS SEAORM] Received accounts webhook");

    match payload.event_type {
        WebhookEventType::Insert { record, .. } => {
            println!("[BILLING ACCOUNTS SEAORM] Processing account creation: {}", record.account_id);
            
            if let Err(e) = handle_account_created_seaorm(&state, &record).await {
                eprintln!("[BILLING ACCOUNTS SEAORM] Error handling account creation: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        WebhookEventType::Update { record, .. } => {
            println!("[BILLING ACCOUNTS SEAORM] Processing account update: {}", record.account_id);
            
            if let Err(e) = handle_account_updated_seaorm(&state, &record).await {
                eprintln!("[BILLING ACCOUNTS SEAORM] Error handling account update: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        WebhookEventType::Delete { old_record, .. } => {
            println!("[BILLING ACCOUNTS SEAORM] Processing account deletion");
            
            if let Err(e) = handle_account_deleted_seaorm(&state, &old_record).await {
                eprintln!("[BILLING ACCOUNTS SEAORM] Error handling account deletion: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
    }

    StatusCode::OK
}

async fn handle_account_created_seaorm(
    state: &Arc<AppState>,
    record: &TableRecord,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[BILLING ACCOUNTS SEAORM] Creating billing record for account: {}", record.account_id);

    // Create a new billing record for the account
    let new_billing_record = accounts_billing::ActiveModel {
        account_id: Set(record.account_id),
        billing_status: Set(Some("trial".to_string())),
        plan: Set(Some("free".to_string())),
        active: Set(Some(true)),
        tasks_used: Set(Some(0)),
        tasks_limit: Set(Some(1000)), // Default free tier limit
        storage_used: Set(Some(0)),
        storage_limit: Set(Some(1024 * 1024 * 100)), // 100MB default
        created_at: Set(Some(chrono::Utc::now())),
        updated_at: Set(Some(chrono::Utc::now())),
        ..Default::default()
    };

    new_billing_record.insert(&*state.db).await?;
    println!("[BILLING ACCOUNTS SEAORM] Successfully created billing record");

    Ok(())
}

async fn handle_account_updated_seaorm(
    state: &Arc<AppState>,
    record: &TableRecord,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[BILLING ACCOUNTS SEAORM] Updating billing record for account: {}", record.account_id);

    // Update the billing record if it exists
    let billing_record = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(record.account_id))
        .one(&*state.db)
        .await?;

    if let Some(record) = billing_record {
        let mut active_model: accounts_billing::ActiveModel = record.into();
        active_model.updated_at = Set(Some(chrono::Utc::now()));
        
        active_model.update(&*state.db).await?;
        println!("[BILLING ACCOUNTS SEAORM] Successfully updated billing record");
    } else {
        println!("[BILLING ACCOUNTS SEAORM] No billing record found to update");
    }

    Ok(())
}

async fn handle_account_deleted_seaorm(
    state: &Arc<AppState>,
    record: &TableRecord,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[BILLING ACCOUNTS SEAORM] Deleting billing record for account: {}", record.account_id);

    // Soft delete or deactivate the billing record
    let billing_record = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(record.account_id))
        .one(&*state.db)
        .await?;

    if let Some(record) = billing_record {
        let mut active_model: accounts_billing::ActiveModel = record.into();
        active_model.active = Set(Some(false));
        active_model.billing_status = Set(Some("deleted".to_string()));
        active_model.updated_at = Set(Some(chrono::Utc::now()));
        
        active_model.update(&*state.db).await?;
        println!("[BILLING ACCOUNTS SEAORM] Successfully deactivated billing record");
    } else {
        println!("[BILLING ACCOUNTS SEAORM] No billing record found to delete");
    }

    Ok(())
}

/// Endpoint to manually create or update billing records
pub async fn upsert_customer_subscription(
    State(state): State<Arc<AppState>>,
    Json(input): Json<UpsertCustomerSubscriptionInput>,
) -> impl IntoResponse {
    println!("[BILLING ACCOUNTS SEAORM] Upserting customer subscription for account: {}", input.account_id);

    // Find existing billing record
    let billing_record = match accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(input.account_id))
        .one(&*state.db)
        .await
    {
        Ok(record) => record,
        Err(e) => {
            eprintln!("[BILLING ACCOUNTS SEAORM] Database error: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let result = match billing_record {
        Some(record) => {
            // Update existing record
            let mut active_model: accounts_billing::ActiveModel = record.into();
            active_model.stripe_customer_id = Set(Some(input.stripe_customer_id));
            active_model.stripe_subscription_id = Set(input.stripe_subscription_id);
            active_model.plan = Set(input.plan);
            active_model.active = Set(Some(input.active));
            active_model.updated_at = Set(Some(chrono::Utc::now()));
            
            active_model.update(&*state.db).await.map(|_| ())
        }
        None => {
            // Create new record
            let new_record = accounts_billing::ActiveModel {
                account_id: Set(input.account_id),
                stripe_customer_id: Set(Some(input.stripe_customer_id)),
                stripe_subscription_id: Set(input.stripe_subscription_id),
                plan: Set(input.plan),
                active: Set(Some(input.active)),
                billing_status: Set(Some("active".to_string())),
                tasks_used: Set(Some(0)),
                tasks_limit: Set(Some(10000)), // Premium limit
                storage_used: Set(Some(0)),
                storage_limit: Set(Some(1024 * 1024 * 1000)), // 1GB
                created_at: Set(Some(chrono::Utc::now())),
                updated_at: Set(Some(chrono::Utc::now())),
                ..Default::default()
            };
            
            new_record.insert(&*state.db).await.map(|_| ())
        }
    };

    match result {
        Ok(_) => {
            println!("[BILLING ACCOUNTS SEAORM] Successfully upserted customer subscription");
            Json(json!({
                "status": "success",
                "message": "Customer subscription upserted"
            })).into_response()
        }
        Err(e) => {
            eprintln!("[BILLING ACCOUNTS SEAORM] Error upserting customer subscription: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
