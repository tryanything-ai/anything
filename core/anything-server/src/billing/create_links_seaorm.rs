use crate::custom_auth::User;
use crate::entities::accounts_billing;
use crate::AppState;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::{
    BillingPortalSession, CheckoutSession, CheckoutSessionMode, Client as StripeClient,
    CreateBillingPortalSession, CreateCheckoutSession, CreateCheckoutSessionLineItems, CustomerId,
};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CheckoutRequest {
    return_url: String,
}

#[derive(Deserialize)]
pub struct PortalRequest {
    return_url: String,
}

#[derive(Serialize)]
pub struct CheckoutResponse {
    checkout_url: String,
}

#[derive(Serialize)]
pub struct PortalResponse {
    portal_url: String,
}

pub async fn get_checkout_link(
    Path(account_id): Path<String>,
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<CheckoutRequest>,
) -> impl IntoResponse {
    println!(
        "[BILLING LINKS] Starting get_checkout_link for account_id: {}",
        account_id
    );

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Fetch the customer's Stripe ID from the accounts_billing table using SeaORM
    let billing_record = match accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(billing)) => billing,
        Ok(None) => {
            println!("[BILLING LINKS] No billing record found for account: {}", account_id);
            return (StatusCode::NOT_FOUND, "No billing setup found").into_response();
        }
        Err(err) => {
            println!("[BILLING LINKS] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let customer_stripe_id = match billing_record.stripe_customer_id {
        Some(stripe_id) => stripe_id,
        None => {
            println!("[BILLING LINKS] No Stripe customer ID found for account: {}", account_id);
            return (StatusCode::BAD_REQUEST, "No Stripe customer ID found").into_response();
        }
    };

    println!(
        "[BILLING LINKS] Found Stripe customer ID: {}",
        customer_stripe_id
    );

    // Create Stripe client
    let stripe_client = match std::env::var("STRIPE_SECRET_KEY") {
        Ok(key) => StripeClient::new(key),
        Err(_) => {
            println!("[BILLING LINKS] STRIPE_SECRET_KEY not found");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Stripe configuration error").into_response();
        }
    };

    // Create checkout session
    let customer_id = match customer_stripe_id.parse::<CustomerId>() {
        Ok(id) => id,
        Err(_) => {
            println!("[BILLING LINKS] Invalid Stripe customer ID format");
            return (StatusCode::BAD_REQUEST, "Invalid Stripe customer ID").into_response();
        }
    };

    let checkout_session = CreateCheckoutSession {
        mode: Some(CheckoutSessionMode::Subscription),
        customer: Some(customer_id),
        success_url: Some(&request.return_url),
        cancel_url: Some(&request.return_url),
        line_items: Some(vec![CreateCheckoutSessionLineItems {
            price: Some("price_1234567890".to_string()), // TODO: Use actual price ID
            quantity: Some(1),
            ..Default::default()
        }]),
        ..Default::default()
    };

    match CheckoutSession::create(&stripe_client, checkout_session).await {
        Ok(session) => {
            let checkout_url = session.url.unwrap_or_default();
            println!("[BILLING LINKS] Created checkout session: {}", checkout_url);
            
            Json(CheckoutResponse { checkout_url }).into_response()
        }
        Err(err) => {
            println!("[BILLING LINKS] Failed to create checkout session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create checkout session").into_response()
        }
    }
}

pub async fn get_billing_portal_link(
    Path(account_id): Path<String>,
    Extension(user): Extension<User>,
    State(state): State<Arc<AppState>>,
    Json(request): Json<PortalRequest>,
) -> impl IntoResponse {
    println!(
        "[BILLING LINKS] Starting get_billing_portal_link for account_id: {}",
        account_id
    );

    let account_uuid = match Uuid::parse_str(&account_id) {
        Ok(uuid) => uuid,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid account ID").into_response(),
    };

    // Fetch the customer's Stripe ID using SeaORM
    let billing_record = match accounts_billing::Entity::find()
        .filter(accounts_billing::Column::AccountId.eq(account_uuid))
        .one(&*state.db)
        .await
    {
        Ok(Some(billing)) => billing,
        Ok(None) => {
            println!("[BILLING LINKS] No billing record found for account: {}", account_id);
            return (StatusCode::NOT_FOUND, "No billing setup found").into_response();
        }
        Err(err) => {
            println!("[BILLING LINKS] Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let customer_stripe_id = match billing_record.stripe_customer_id {
        Some(stripe_id) => stripe_id,
        None => {
            println!("[BILLING LINKS] No Stripe customer ID found for account: {}", account_id);
            return (StatusCode::BAD_REQUEST, "No Stripe customer ID found").into_response();
        }
    };

    // Create Stripe client
    let stripe_client = match std::env::var("STRIPE_SECRET_KEY") {
        Ok(key) => StripeClient::new(key),
        Err(_) => {
            println!("[BILLING LINKS] STRIPE_SECRET_KEY not found");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Stripe configuration error").into_response();
        }
    };

    // Create billing portal session
    let customer_id = match customer_stripe_id.parse::<CustomerId>() {
        Ok(id) => id,
        Err(_) => {
            println!("[BILLING LINKS] Invalid Stripe customer ID format");
            return (StatusCode::BAD_REQUEST, "Invalid Stripe customer ID").into_response();
        }
    };

    let mut portal_session = CreateBillingPortalSession::new(customer_id);
    portal_session.return_url = Some(&request.return_url);

    match BillingPortalSession::create(&stripe_client, portal_session).await {
        Ok(session) => {
            let portal_url = session.url;
            println!("[BILLING LINKS] Created portal session: {}", portal_url);
            
            Json(PortalResponse { portal_url }).into_response()
        }
        Err(err) => {
            println!("[BILLING LINKS] Failed to create portal session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create portal session").into_response()
        }
    }
}
