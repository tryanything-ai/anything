use axum::{
    Router,
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use std::env;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

//https://stackoverflow.com/a/76347410
//https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html#method.insecure_disable_signature_validation
fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    println!("Decoding JWT");
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.insecure_disable_signature_validation();
    validation.validate_aud = false;
    // validation.validate_iss = false;
    // let mut validation = Validation::new(Algorithm::HS256);
    // validation.insecure_disable_signature_validation();

    // let data: Claims = decode(&token, &key, &validation)?;

    // let decoding_key = DecodingKey::from_secret(secret.as_ref());
    // // println!("Decoding Key: {:?}", decoding_key);
    // let validation = Validation::new(Algorithm::HS256);
    // println!("Validation: {:?}", validation);
    let token_data = decode::<Claims>(&token, &key, &validation)?;
    // println!("Token Data: {:?}", token_data);
    Ok(token_data.claims)
}


pub async fn middleware(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // println!("Inauth Middlware");
    // let secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");
    // println!("Secret: {}", secret);

       let jwt = match  headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
    //     Some(auth_header) if auth_header.starts_with("Bearer ") => &auth_header[7..],
    //     _ => return Err(StatusCode::UNAUTHORIZED),
    // };

    match decode_jwt(jwt) {
        Ok(claims) => {
            request.extensions_mut().insert(claims.sub.clone());
            request.extensions_mut().insert(jwt.to_string());
            let response = next.run(request).await;
            println!("Response after decode: {:?}", response);
            Ok(response)
        }
        Err(e) => {
            println!("Error decoding JWT: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        },
    }

    //  let jwt = match headers.get(AUTHORIZATION).and_then(|h| h.to_str().ok()) {
    //     Some(jwt) => jwt,
    //     None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    // };

    // match get_token(&headers) {
    //     Some(token) if token_is_valid(token) => {
    //         let response = next.run(request).await;
    //         Ok(response)
    //     }
    //     _ => {
    //         Err(StatusCode::UNAUTHORIZED)
    //     }
    // }
}

// fn get_token(headers: &HeaderMap) -> Option<&str> {
//     // ...
// }

// fn token_is_valid(token: &str) -> bool {
//     // ...
// }