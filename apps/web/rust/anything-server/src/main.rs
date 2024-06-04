use axum::{Router, routing::get, Json, extract::State,  http::{HeaderValue, Method, StatusCode}, response::IntoResponse};
use hyper::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE}; 
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use postgrest::Postgrest;
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_api_key = env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set");

    let client = Arc::new(Postgrest::new(supabase_url).insert_header("apikey", supabase_api_key));

    let cors = CorsLayer::new()
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/items", get(get_items))
        .layer(cors)
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        //     // or see this issue https://github.com/tokio-rs/axum/issues/849
        //     CorsLayer::new()
        //     // .allow_origin(Any)
        //     // .allow_methods(Any)
        //     // .allow_headers(Any)
        //         .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        //         .allow_methods([Method::GET]),
        // )
        // .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .with_state(client);

          // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World!"
}

// #[derive(Serialize, Deserialize)]
// struct Item {
//     id: i32,
//     name: String,
// }

// async fn get_items(State(client): State<Arc<Postgrest>>, headers: HeaderMap) -> Json<Vec<Item>> {
//     let jwt = headers.get("Authorization").and_then(|h| h.to_str().ok()).unwrap_or("");

//     let response = client
//         .from("flows")
//         .auth(jwt)
//         .select("*")
//         .execute()
//         .await?; 

//     let body = response.text().await?; 
//     let items: Vec<Item> = serde_json::from_str(&body).unwrap();
//     Json(items)
// }

async fn get_items(State(client): State<Arc<Postgrest>>, headers: HeaderMap) -> impl IntoResponse {
    let jwt = match headers.get("Authorization").and_then(|h| h.to_str().ok()) {
        Some(jwt) => jwt,
        None => return (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response(),
    };

    let response = match client
        .from("flows")
        .auth(jwt)
        .select("*")
        .execute()
        .await
    {
        Ok(response) => response,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to execute request").into_response(),
    };

    println!("response: {:?}", response);

    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body").into_response(),
    };

    let items: Value = match serde_json::from_str(&body) {
        Ok(items) => items,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
    };

    Json(items).into_response()
}