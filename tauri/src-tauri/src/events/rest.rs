use reqwest::{Error, Method, header::HeaderMap, header::HeaderName};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiRequest {
    pub url: String,
    pub method: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

// #[tauri::command]
pub async fn call_api(api_request: ApiRequest) -> Result<String, Error> {
    let client = reqwest::Client::new();
    
    // Parse HTTP method
    let method = Method::from_bytes(api_request.method.as_bytes()).expect("Invalid HTTP method");

    // Build request
    let mut request_builder = client.request(method, &api_request.url);

    // Add headers if any
    if let Some(headers_map) = api_request.headers {
        let mut headers = HeaderMap::new();
        for (key, value) in headers_map.iter() {
            headers.insert(
                HeaderName::from_bytes(key.as_bytes()).expect("Invalid header name"),
                value.parse().expect("Invalid header value"),
            );
        }
        request_builder = request_builder.headers(headers);
    }

    // Add body if any
    if let Some(body) = api_request.body {
        request_builder = request_builder.body(body);
    }

    // Execute request
    let response = request_builder.send().await?;

    if response.status().is_success() {
        let text = response.text().await?;
        println!("res from rest call processor: {:?}", text);
        Ok(text)
    } else {
        //TODO: this is probabaly bad error handling
        Err(response.error_for_status().unwrap_err())
    }
}


// Example FE Call
// const apiRequest = {
//     url: 'https://jsonplaceholder.typicode.com/todos',
//     method: 'POST',
//     headers: {
//       'Content-Type': 'application/json',
//     },
//     body: JSON.stringify({ title: 'foo', body: 'bar', userId: 1 }),
//   };
//   const apiResult = await window.__TAURI__.invoke('call_api', apiRequest);
//   console.log(apiResult);