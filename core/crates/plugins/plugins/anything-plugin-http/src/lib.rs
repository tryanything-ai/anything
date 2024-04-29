extern crate anything_runtime;
use anyhow::{anyhow, Error};
use anything_common::tracing;
use anything_runtime::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use ureq;

#[derive(Debug, Default)]
pub struct HttpClientPlugin {
    config: RuntimeConfig,
}

impl Extension for HttpClientPlugin {
    fn name(&self) -> &'static str {
        "http"
    }

    fn on_load(&mut self, config: RuntimeConfig) {
        self.config = config;
    }

    fn on_unload(&self) {
        // Nothing to do here
    }

    fn register_action(&self) -> &'static str {
        static JSON_DATA: &str = r#"{
            "trigger": false,
            "node_name": "http_action",
            "node_label": "HTTP Action",
            "icon": "<svg width=\"16\" height=\"16\" viewBox=\"0 0 16 16\" xmlns=\"http://www.w3.org/2000/svg\" fill=\"currentColor\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M2.998 5.58a5.55 5.55 0 0 1 1.62-3.88l-.71-.7a6.45 6.45 0 0 0 0 9.16l.71-.7a5.55 5.55 0 0 1-1.62-3.88zm1.06 0a4.42 4.42 0 0 0 1.32 3.17l.71-.71a3.27 3.27 0 0 1-.76-1.12 3.45 3.45 0 0 1 0-2.67 3.22 3.22 0 0 1 .76-1.13l-.71-.71a4.46 4.46 0 0 0-1.32 3.17zm7.65 3.21l-.71-.71c.33-.32.59-.704.76-1.13a3.449 3.449 0 0 0 0-2.67 3.22 3.22 0 0 0-.76-1.13l.71-.7a4.468 4.468 0 0 1 0 6.34zM13.068 1l-.71.71a5.43 5.43 0 0 1 0 7.74l.71.71a6.45 6.45 0 0 0 0-9.16zM9.993 5.43a1.5 1.5 0 0 1-.245.98 2 2 0 0 1-.27.23l3.44 7.73-.92.4-.77-1.73h-5.54l-.77 1.73-.92-.4 3.44-7.73a1.52 1.52 0 0 1-.33-1.63 1.55 1.55 0 0 1 .56-.68 1.5 1.5 0 0 1 2.325 1.1zm-1.595-.34a.52.52 0 0 0-.25.14.52.52 0 0 0-.11.22.48.48 0 0 0 0 .29c.04.09.102.17.18.23a.54.54 0 0 0 .28.08.51.51 0 0 0 .5-.5.54.54 0 0 0-.08-.28.58.58 0 0 0-.23-.18.48.48 0 0 0-.29 0zm.23 2.05h-.27l-.87 1.94h2l-.86-1.94zm2.2 4.94l-.89-2h-2.88l-.89 2h4.66z\"/></svg>",
            "description": "Make an HTTP request",
            "handles": [
                {
                    "id": "a",
                    "position": "top",
                    "type": "target"
                },
                {
                    "id": "b",
                    "position": "bottom",
                    "type": "source"
                }
            ],
            "variables": [],
            "config": {
                "method": "GET",
                "url": "",
                "headers": {},
                "body": ""
            },
            "extension_id": "http"
        }"#;

        JSON_DATA
    }
}

impl ExecutionPlugin for HttpClientPlugin {
    fn execute(
        &self,
        _scope: &Scope,
        config: &ExecuteConfig,
    ) -> Result<ExecutionResult, Box<PluginError>> {
        let method = config
            .context
            .get("method")
            .unwrap_or(&json!("GET"))
            .as_str()
            .unwrap()
            .to_uppercase();
        let url = config
            .context
            .get("url")
            .unwrap_or(&json!(""))
            .as_str()
            .unwrap()
            .to_string();
        let headers: HashMap<String, String> = config
            .context
            .get("headers")
            .unwrap_or(&json!({}))
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
            .collect();
        let body = config
            .context
            .get("body")
            .unwrap_or(&json!(""))
            .as_str()
            .unwrap()
            .to_string();

        let request = ureq::request(&method, &url);

        for (key, value) in &headers {
            request.clone().set(key, value);
        }

        let response = if body.is_empty() {
            request.call()
        } else {
            request.send_string(&body)
        };

        match response {
            Ok(success) => {
                let status = success.status();
                let headers: HashMap<String, String> = success
                    .headers_names()
                    .iter()
                    .map(|k| (k.clone(), success.header(k).unwrap_or_default().to_string()))
                    .collect();
                let body = success.into_string().unwrap_or_default();

                Ok(ExecutionResult {
                    stdout: body.clone(),
                    stderr: "".to_string(),
                    status: status as i32,
                    result: json!({
                        "status": status,
                        "headers": headers,
                        "body": body
                    }),
                })
            }
            Err(e) => Err(Box::new(PluginError::AnythingError(Error::new(e)))),
        }
    }
    // fn execute(
    //     &self,
    //     _scope: &Scope,
    //     config: &ExecuteConfig,
    // ) -> Result<ExecutionResult, Box<PluginError>> {
    //     let method = config
    //         .context
    //         .get("method")
    //         .unwrap_or(&json!("GET"))
    //         .as_str()
    //         .unwrap()
    //         .to_uppercase();
    //     let url = config
    //         .context
    //         .get("url")
    //         .unwrap_or(&json!(""))
    //         .as_str()
    //         .unwrap()
    //         .to_string();
    //     let headers: HashMap<String, String> = config
    //         .context
    //         .get("headers")
    //         .unwrap_or(&json!({}))
    //         .as_object()
    //         .unwrap()
    //         .iter()
    //         .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
    //         .collect();
    //     let body = config
    //         .context
    //         .get("body")
    //         .unwrap_or(&json!(""))
    //         .as_str()
    //         .unwrap()
    //         .to_string();

    //     let client = Client::new();

    //     let future = async {
    //         let mut request = match method.as_str() {
    //             "GET" => client.get(&url),
    //             "POST" => client.post(&url),
    //             "PUT" => client.put(&url),
    //             "DELETE" => client.delete(&url),
    //             _ => {
    //                 return Err(PluginError::Custom(
    //                     "Invalid HTTP method".to_string(),
    //                 ))
    //             }
    //         };

    //         for (key, value) in headers {
    //             request = request.header(key, value);
    //         }

    //         let response = if body.is_empty() {
    //             request.send().await.map_err(|err| PluginError::AnythingError(Error::new(err)))?
    //         } else {
    //             request
    //                 .body(body)
    //                 .send()
    //                 .await
    //                 .map_err(|err| PluginError::AnythingError(Error::new(err)))?
    //         };

    //         let status = response.status().as_u16();
    //         let headers: HashMap<String, String> = response
    //             .headers()
    //             .iter()
    //             .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
    //             .collect();
    //         let body = response
    //             .text()
    //             .await
    //             .map_err(|err| PluginError::AnythingError(Error::new(err)))?;

    //         Ok(ExecutionResult {
    //             stdout: body.clone(),
    //             stderr: "".to_string(),
    //             status: status as i32,
    //             result: json!({
    //                 "status": status,
    //                 "headers": headers,
    //                 "body": body
    //             }),
    //         })

    //     };

    //     match futures::executor::block_on(future) {
    //         Ok(result) => Ok(result),
    //         Err(err) => Err(Box::new(err)),
    //     }
    // }
}

declare_plugin!(HttpClientPlugin, HttpClientPlugin::default);
