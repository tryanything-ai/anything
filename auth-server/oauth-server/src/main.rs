use axum::{
    extract::{Form, Query},
    response::{Redirect, Json},
    routing::post,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use oauth2::{
    AuthorizationCode as OAuth2AuthorizationCode,
    ClientId, ClientSecret, RedirectUrl,
    TokenResponse, basic::BasicClient,
    AuthUrl, TokenUrl
};

mod oauth;use axum::{
    extract::{Form, Query},
    response::{Redirect, Json},
    routing::post,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use oauth2::{
    AuthorizationCode as OAuth2AuthorizationCode,
    ClientId, ClientSecret, RedirectUrl,
    TokenResponse, basic::BasicClient,
    AuthUrl, TokenUrl
};

mod oauth;

#[derive(Deserialize)]
struct AuthRequest {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    state: Option<String>,
}

#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    let auth_codes = Arc::new(Mutex::new(Vec::<oauth::AuthorizationCode>::new()));
    let access_tokens = Arc::new(Mutex::new(Vec::<oauth::AccessToken>::new()));

    let auth_codes_filter = axum::extract::Extension(auth_codes.clone());
    let access_tokens_filter = axum::extract::Extension(access_tokens.clone());

    let app = Router::new()
        .route("/authorize", axum::routing::get(authorize))
        .route("/token", post(token))
        .layer(auth_codes_filter)
        .layer(access_tokens_filter);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn authorize(
    Query(req): Query<AuthRequest>,
    Extension(auth_codes): axum::extract::Extension<Arc<Mutex<Vec<oauth::AuthorizationCode>>>>,
) -> Redirect {
    // Validate client_id and redirect_uri
    let code = Uuid::new_v4().to_string();
    let auth_code = oauth::AuthorizationCode {
        code: code.clone(),
        client_id: req.client_id.clone(),
        redirect_uri: req.redirect_uri.clone(),
        user_id: "user_id_from_db".to_string(), // Replace with actual user ID retrieval logic
        expires_at: Utc::now() + chrono::Duration::minutes(10),
    };
    auth_codes.lock().unwrap().push(auth_code);
    Redirect::temporary(&format!("{}?code={}", req.redirect_uri, code))
}

async fn token(
    Form(req): Form<TokenRequest>,
    Extension(auth_codes): axum::extract::Extension<Arc<Mutex<Vec<oauth::AuthorizationCode>>>>,
    Extension(access_tokens): axum::extract::Extension<Arc<Mutex<Vec<oauth::AccessToken>>>>,
) -> Json<oauth::TokenResponse> {
    // Validate the authorization code and other parameters
    let auth_code = auth_codes.lock().unwrap().iter().find(|&code| code.code == req.code).cloned();
    if let Some(code) = auth_code {
        if code.client_id == req.client_id && code.redirect_uri == req.redirect_uri {
            let token = Uuid::new_v4().toString();
            let access_token = oauth::AccessToken {
                token: token.clone(),
                client_id: req.client_id.clone(),
                user_id: code.user_id.clone(),
                expires_at: Utc::now() + chrono::Duration::hours(1),
            };
            access_tokens.lock().unwrap().push(access_token);
            return Json(oauth::TokenResponse {
                access_token: token,
                token_type: "bearer".to_string(),
                expires_in: 3600,
            });
        }
    }
    // Handle invalid code or other errors
    Json(oauth::TokenResponse {
        access_token: "".to_string(),
        token_type: "".to_string(),
        expires_in: 0,
    })
}


#[derive(Deserialize)]
struct AuthRequest {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: String,
    state: Option<String>,
}

#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

#[tokio::main]
async fn main() {
    let auth_codes = Arc::new(Mutex::new(Vec::<oauth::AuthorizationCode>::new()));
    let access_tokens = Arc::new(Mutex::new(Vec::<oauth::AccessToken>::new()));

    let auth_codes_filter = axum::extract::Extension(auth_codes.clone());
    let access_tokens_filter = axum::extract::Extension(access_tokens.clone());

    let app = Router::new()
        .route("/authorize", axum::routing::get(authorize))
        .route("/token", post(token))
        .layer(auth_codes_filter)
        .layer(access_tokens_filter);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn authorize(
    Query(req): Query<AuthRequest>,
    Extension(auth_codes): axum::extract::Extension<Arc<Mutex<Vec<oauth::AuthorizationCode>>>>,
) -> Redirect {
    // Validate client_id and redirect_uri
    let code = Uuid::new_v4().to_string();
    let auth_code = oauth::AuthorizationCode {
        code: code.clone(),
        client_id: req.client_id.clone(),
        redirect_uri: req.redirect_uri.clone(),
        user_id: "user_id_from_db".to_string(), // Replace with actual user ID retrieval logic
        expires_at: Utc::now() + chrono::Duration::minutes(10),
    };
    auth_codes.lock().unwrap().push(auth_code);
    Redirect::temporary(&format!("{}?code={}", req.redirect_uri, code))
}

async fn token(
    Form(req): Form<TokenRequest>,
    Extension(auth_codes): axum::extract::Extension<Arc<Mutex<Vec<oauth::AuthorizationCode>>>>,
    Extension(access_tokens): axum::extract::Extension<Arc<Mutex<Vec<oauth::AccessToken>>>>,
) -> Json<oauth::TokenResponse> {
    // Validate the authorization code and other parameters
    let auth_code = auth_codes.lock().unwrap().iter().find(|&code| code.code == req.code).cloned();
    if let Some(code) = auth_code {
        if code.client_id == req.client_id && code.redirect_uri == req.redirect_uri {
            let token = Uuid::new_v4().toString();
            let access_token = oauth::AccessToken {
                token: token.clone(),
                client_id: req.client_id.clone(),
                user_id: code.user_id.clone(),
                expires_at: Utc::now() + chrono::Duration::hours(1),
            };
            access_tokens.lock().unwrap().push(access_token);
            return Json(oauth::TokenResponse {
                access_token: token,
                token_type: "bearer".to_string(),
                expires_in: 3600,
            });
        }
    }
    // Handle invalid code or other errors
    Json(oauth::TokenResponse {
        access_token: "".to_string(),
        token_type: "".to_string(),
        expires_in: 0,
    })
}
