use crate::supabase_auth_middleware::User;
use crate::AppState;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use stripe::{
    BillingPortalSession, CheckoutSession, CheckoutSessionMode, Client as StripeClient,
    CreateBillingPortalSession, CreateCheckoutSession, CreateCheckoutSessionLineItems, CustomerId,
};

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

    // Fetch the customer's Stripe ID from the accounts_billing table
    let customer_stripe_id = match state
        .anything_client
        .from("accounts_billing")
        .auth(&user.jwt) // Pass a reference to the JWT
        .select("stripe_customer_id")
        .eq("account_id", account_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(body) => match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(value) => {
                    let stripe_id = value
                        .get("stripe_customer_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    println!(
                        "[BILLING LINKS] Retrieved stripe_customer_id: {:?}",
                        stripe_id
                    );
                    stripe_id
                }
                Err(e) => {
                    println!("[BILLING LINKS] Error parsing JSON: {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
            Err(e) => {
                println!("[BILLING LINKS] Error reading response body: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        Err(e) => {
            println!("[BILLING LINKS] Error querying accounts_billing: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let customer_stripe_id = customer_stripe_id.unwrap_or_default();
    // If no Stripe customer exists, return an error
    if customer_stripe_id.is_empty() {
        println!("[BILLING LINKS] No Stripe customer ID found for account_id");
        return Err(StatusCode::BAD_REQUEST);
    }

    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        println!("[BILLING LINKS] Error fetching STRIPE_SECRET_KEY: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let client = StripeClient::new(stripe_secret_key);

    // TODO: Fetch the correct price IDs based on your pricing strategy
    let metered_price = "price_1PwpfXFBAuZoeGEU0iJhmcxF".to_string(); //Usage Based $0
    let fixed_base_price = "price_1Pwpe2FBAuZoeGEUh9zS63rH".to_string(); // $9.99/month

    println!(
        "[BILLING LINKS] Creating Stripe checkout session with price IDs: {} and {}",
        metered_price, fixed_base_price
    );

    let mut params = CreateCheckoutSession::new();
    params.cancel_url = Some(&request.return_url);
    params.success_url = Some(&request.return_url);
    params.customer = Some(customer_stripe_id.parse::<CustomerId>().unwrap());
    params.mode = Some(CheckoutSessionMode::Subscription);
    params.line_items = Some(vec![
        CreateCheckoutSessionLineItems {
            price: Some(metered_price), // no quantity allowed on metered prices
            ..Default::default()
        },
        CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price: Some(fixed_base_price),
            ..Default::default()
        },
    ]);

    let checkout_session = CheckoutSession::create(&client, params)
        .await
        .map_err(|e| {
            println!(
                "[BILLING LINKS] Error creating Stripe checkout session: {:?}",
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let checkout_url = checkout_session.url.unwrap_or_default();
    println!(
        "[BILLING LINKS] Checkout session created successfully. URL: {}",
        checkout_url
    );

    Ok(Json(CheckoutResponse { checkout_url }))
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

    // Fetch the customer's Stripe ID from the accounts_billing table
    let customer_stripe_id = match state
        .anything_client
        .from("accounts_billing")
        .auth(&user.jwt) // Pass a reference to the JWT
        .select("stripe_customer_id")
        .eq("account_id", account_id)
        .single()
        .execute()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(body) => match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(value) => {
                    let stripe_id = value
                        .get("stripe_customer_id")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    println!(
                        "[BILLING LINKS] Retrieved stripe_customer_id: {:?}",
                        stripe_id
                    );
                    stripe_id
                }
                Err(e) => {
                    println!("[BILLING LINKS] Error parsing JSON: {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            },
            Err(e) => {
                println!("[BILLING LINKS] Error reading response body: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        Err(e) => {
            println!("[BILLING LINKS] Error querying accounts_billing: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let customer_stripe_id = customer_stripe_id.unwrap_or_default();
    // If no Stripe customer exists, return an error
    if customer_stripe_id.is_empty() {
        println!("[BILLING LINKS] No Stripe customer ID found for account_id");
        return Err(StatusCode::BAD_REQUEST);
    }

    let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").map_err(|e| {
        println!("[BILLING LINKS] Error fetching STRIPE_SECRET_KEY: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let client = StripeClient::new(stripe_secret_key);

    let mut params =
        CreateBillingPortalSession::new(customer_stripe_id.parse::<CustomerId>().unwrap());
    params.return_url = Some(&request.return_url);

    let billing_portal_session = BillingPortalSession::create(&client, params)
        .await
        .map_err(|e| {
            println!(
                "[BILLING LINKS] Error creating Stripe billing portal session: {:?}",
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let billing_portal_url = billing_portal_session.url;
    println!(
        "[BILLING LINKS] Billing portal session created successfully. URL: {}",
        billing_portal_url
    );

    Ok(Json(PortalResponse {
        portal_url: billing_portal_url,
    }))
}
