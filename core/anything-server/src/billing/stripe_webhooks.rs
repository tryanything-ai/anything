use crate::AppState;
use axum::{
    async_trait,
    body::Body,
    extract::{FromRequest, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
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
    StripeEvent(event): StripeEvent,
) -> Result<impl IntoResponse, StatusCode> {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                println!(
                    "[STRIPE WEBHOOKS] Received checkout session completed webhook with id: {:?}",
                    session.id
                );
            }
        }
        EventType::AccountUpdated => {
            if let EventObject::Account(account) = event.data.object {
                println!(
                    "[STRIPE WEBHOOKS] Received account updated webhook for account: {:?}",
                    account.id
                );
            }
        }
        EventType::CustomerSubscriptionCreated => {
            if let EventObject::Subscription(subscription) = event.data.object {
                println!(
                    "[STRIPE WEBHOOKS] Received customer subscription created webhook for subscription: {:?}",
                    subscription.id
                );
                let customer = subscription.customer;
                println!("[STRIPE WEBHOOKS] Customer ID: {:?}", customer.id());

                // Update the accounts_billing table
                let query = state
                    .anything_client
                    .from("accounts_billing")
                    .update(
                        json!({
                            "paying_customer": true,
                            "customer_status": "active",
                        })
                        .to_string(),
                    )
                    .eq("stripe_customer_id", customer.id());

                match query.execute().await {
                    Ok(_) => println!(
                        "[STRIPE WEBHOOKS] Successfully updated accounts_billing for customer: {}",
                        customer.id()
                    ),
                    Err(e) => {
                        eprintln!("[STRIPE WEBHOOKS] Error updating accounts_billing: {:?}", e)
                    }
                }
            } else {
                eprintln!("[STRIPE WEBHOOKS] No customer found in subscription");
            }
        }
        _ => println!(
            "[STRIPE WEBHOOKS] Unknown event encountered in webhook: {:?}",
            event.type_
        ),
    }

    Ok(StatusCode::OK)
}
