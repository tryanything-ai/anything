use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub jwt: String,
    pub account_id: String,
}

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    aud: String,
    iss: String,
}

//https://stackoverflow.com/a/76347410
//https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html#method.insecure_disable_signature_validation
//https://github.com/orgs/supabase/discussions/20763#discussioncomment-9502807 ( audience = authenticated )
fn decode_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    println!("Decoding JWT");
    let key = DecodingKey::from_secret(secret.as_ref());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["authenticated"]);
    let token_data = decode::<Claims>(&token, &key, &validation)?;
    Ok(token_data.claims)
}

pub async fn middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    println!("Running Auth Middlware");
    let secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    match decode_jwt(jwt, &secret) {
        Ok(claims) => {
            let user = User {
                jwt: jwt.to_string(),
                account_id: claims.sub.clone(),
            };

            request.extensions_mut().insert(user);
            let response = next.run(request).await;

            Ok(response)
        }
        Err(e) => {
            println!("Error decoding JWT: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
