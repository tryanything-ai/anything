use crate::AppState;
use crate::entities::accounts_billing;
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde_json::json;
use std::env;
use std::sync::Arc;
use stripe::{Event, EventObject, EventType};

pub struct StripeEvent(Event);

#[async_trait]
impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        let stripe_webhook_secret =
            env::var("STRIPE_WEBHOOK_SECRET").expect("STRIPE_WEBHOOK_SECRET must be set");

        Ok(Self(
            stripe::Webhook::construct_event(
                &payload,
                signature.to_str().unwrap(),
                &stripe_webhook_secret,
            )
            .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

// Subscription events docs
// https://docs.stripe.com/billing/subscriptions/overview#subscription-events
pub async fn handle_webhook(
    State(state): State<Arc<AppState>>,
    event: StripeEvent,
) -> impl IntoResponse {
    println!("[STRIPE WEBHOOKS SEAORM] Received Stripe webhook");

    match event.0.type_ {
        EventType::CustomerSubscriptionCreated => {
            if let EventObject::Subscription(subscription) = event.0.data.object {
                println!(
                    "[STRIPE WEBHOOKS SEAORM] Handling subscription.created for subscription: {}",
                    subscription.id
                );
                let customer = subscription.customer;
                println!("[STRIPE WEBHOOKS SEAORM] Customer ID: {:?}", customer.id());

                // Update the accounts_billing table using SeaORM
                match update_billing_status_seaorm(
                    &state,
                    &customer.id().to_string(),
                    true,
                    "active",
                ).await {
                    Ok(_) => println!(
                        "[STRIPE WEBHOOKS SEAORM] Successfully updated accounts_billing for customer: {}",
                        customer.id()
                    ),
                    Err(e) => {
                        eprintln!("[STRIPE WEBHOOKS SEAORM] Error updating accounts_billing: {:?}", e)
                    }
                }
            }
        }
        EventType::CustomerSubscriptionUpdated => {
            if let EventObject::Subscription(subscription) = event.0.data.object {
                println!(
                    "[STRIPE WEBHOOKS SEAORM] Handling subscription.updated for subscription: {}",
                    subscription.id
                );
                let customer = subscription.customer;
                let status = format!("{:?}", subscription.status).to_lowercase();
                let is_active = matches!(subscription.status, stripe::SubscriptionStatus::Active);

                // Update the accounts_billing table using SeaORM
                match update_billing_status_seaorm(
                    &state,
                    &customer.id().to_string(),
                    is_active,
                    &status,
                ).await {
                    Ok(_) => println!(
                        "[STRIPE WEBHOOKS SEAORM] Successfully updated accounts_billing for customer: {}",
                        customer.id()
                    ),
                    Err(e) => {
                        eprintln!("[STRIPE WEBHOOKS SEAORM] Error updating accounts_billing: {:?}", e)
                    }
                }
            }
        }
        EventType::CustomerSubscriptionDeleted => {
            if let EventObject::Subscription(subscription) = event.0.data.object {
                println!(
                    "[STRIPE WEBHOOKS SEAORM] Handling subscription.deleted for subscription: {}",
                    subscription.id
                );
                let customer = subscription.customer;

                // Update the accounts_billing table using SeaORM
                match update_billing_status_seaorm(
                    &state,
                    &customer.id().to_string(),
                    false,
                    "canceled",
                ).await {
                    Ok(_) => println!(
                        "[STRIPE WEBHOOKS SEAORM] Successfully updated accounts_billing for customer: {}",
                        customer.id()
                    ),
                    Err(e) => {
                        eprintln!("[STRIPE WEBHOOKS SEAORM] Error updating accounts_billing: {:?}", e)
                    }
                }
            }
        }
        _ => {
            println!("[STRIPE WEBHOOKS SEAORM] Unhandled event type: {:?}", event.0.type_);
        }
    }

    StatusCode::OK
}

/// Update billing status using SeaORM
async fn update_billing_status_seaorm(
    state: &Arc<AppState>,
    stripe_customer_id: &str,
    is_paying: bool,
    status: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("[STRIPE WEBHOOKS SEAORM] Updating billing status for customer: {}", stripe_customer_id);

    // Find the accounts_billing record by stripe_customer_id
    let billing_record = accounts_billing::Entity::find()
        .filter(accounts_billing::Column::StripeCustomerId.eq(stripe_customer_id))
        .one(&*state.db)
        .await?;

    match billing_record {
        Some(record) => {
            // Update existing record
            let mut active_model: accounts_billing::ActiveModel = record.into();
            active_model.active = Set(Some(is_paying));
            active_model.billing_status = Set(Some(status.to_string()));
            active_model.updated_at = Set(Some(chrono::Utc::now()));
            
            active_model.update(&*state.db).await?;
            println!("[STRIPE WEBHOOKS SEAORM] Updated existing billing record");
        }
        None => {
            println!("[STRIPE WEBHOOKS SEAORM] No billing record found for customer: {}", stripe_customer_id);
            // Optionally create a new record if it doesn't exist
            // This might happen in edge cases where Stripe webhook arrives before account creation
        }
    }

    Ok(())
}

/// Handle invoice payment events
pub async fn handle_invoice_webhook(
    State(state): State<Arc<AppState>>,
    event: StripeEvent,
) -> impl IntoResponse {
    println!("[STRIPE WEBHOOKS SEAORM] Received Stripe invoice webhook");

    match event.0.type_ {
        EventType::InvoicePaymentSucceeded => {
            if let EventObject::Invoice(invoice) = event.0.data.object {
                println!(
                    "[STRIPE WEBHOOKS SEAORM] Handling invoice.payment_succeeded for invoice: {}",
                    invoice.id
                );
                
                if let Some(customer_id) = invoice.customer {
                    // Update payment status for successful payment
                    match update_billing_status_seaorm(
                        &state,
                        &customer_id.id(),
                        true,
                        "paid",
                    ).await {
                        Ok(_) => println!(
                            "[STRIPE WEBHOOKS SEAORM] Successfully updated payment status for customer: {}",
                            customer_id.id()
                        ),
                        Err(e) => {
                            eprintln!("[STRIPE WEBHOOKS SEAORM] Error updating payment status: {:?}", e)
                        }
                    }
                }
            }
        }
        EventType::InvoicePaymentFailed => {
            if let EventObject::Invoice(invoice) = event.0.data.object {
                println!(
                    "[STRIPE WEBHOOKS SEAORM] Handling invoice.payment_failed for invoice: {}",
                    invoice.id
                );
                
                if let Some(customer_id) = invoice.customer {
                    // Update payment status for failed payment
                    match update_billing_status_seaorm(
                        &state,
                        &customer_id.id(),
                        false,
                        "payment_failed",
                    ).await {
                        Ok(_) => println!(
                            "[STRIPE WEBHOOKS SEAORM] Successfully updated payment failure status for customer: {}",
                            customer_id.id()
                        ),
                        Err(e) => {
                            eprintln!("[STRIPE WEBHOOKS SEAORM] Error updating payment failure status: {:?}", e)
                        }
                    }
                }
            }
        }
        _ => {
            println!("[STRIPE WEBHOOKS SEAORM] Unhandled invoice event type: {:?}", event.0.type_);
        }
    }

    StatusCode::OK
}
