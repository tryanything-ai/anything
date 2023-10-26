use std::collections::HashMap;

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    error::EngineResult,
    types::{Process, ProcessBuilder, ProcessStateBuilder},
};

use super::Engine;
use anything_graph::flow::{
    action::{ActionType, RestAction},
    node::{Node, NodeState},
};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct RestEngine {
    pub config: RestAction,
    pub process: Option<Process>,
}

impl RestEngine {
    pub fn new(config: RestAction) -> Self {
        Self {
            config,
            process: Some(Process::default()),
        }
    }

    /// Make a get request to the url
    async fn make_get_request(&self, _context: &NodeExecutionContext) -> EngineResult<Response> {
        let mut url = Url::parse(&self.config.url).unwrap();
        if let Some(qs) = &self.config.query_params {
            for (key, val) in qs.into_iter() {
                url.query_pairs_mut().append_pair(key, val);
            }
        }
        let req = reqwest::get(url).await?;
        Ok(req)
    }
}

#[async_trait::async_trait]
impl Engine for RestEngine {
    async fn run(&mut self, context: &NodeExecutionContext) -> EngineResult<Process> {
        let mut process = self.process.clone().unwrap();
        process.state.status = Some(NodeState::Running);

        let response = if let Some(method) = self.config.method.as_ref() {
            match method.as_str() {
                _ => self.make_get_request(context).await?,
            }
        } else {
            self.make_get_request(context).await?
        };
        let stdout = response.text().await?;

        Ok(ProcessBuilder::default()
            .state(
                ProcessStateBuilder::default()
                    .stdout(stdout)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap())
    }
    fn process(&self) -> Option<Process> {
        self.process.clone()
    }
    fn render(
        &mut self,
        node: &Node,
        global_context: &ExecutionContext,
        event_payload: &serde_json::Value,
    ) -> EngineResult<NodeExecutionContext> {
        let mut exec_context = NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        };

        let mut rest_action = RestAction {
            ..self.config.clone()
        };

        let url = self.config.url.clone();
        let evaluated_url = global_context.render_string(&exec_context, event_payload, url);

        self.config.url = evaluated_url.clone();
        rest_action.url = evaluated_url.clone();

        if let Some(body) = &self.config.body {
            let evaluated_body =
                global_context.render_string(&exec_context, event_payload, body.clone());
            self.config.body = Some(evaluated_body.clone());
            rest_action.body = Some(evaluated_body.clone());
        }

        let mut evaluated_qs: HashMap<String, String> = HashMap::new();
        if let Some(qs) = &self.config.query_params {
            for (key, val) in qs.into_iter() {
                let evaluated_val =
                    global_context.render_string(&exec_context, event_payload, val.clone());
                evaluated_qs.insert(key.clone(), evaluated_val);
            }
            rest_action.query_params = Some(evaluated_qs.clone());
        }

        let mut evaluated_headers: HashMap<String, String> = HashMap::new();
        if let Some(args) = &self.config.headers {
            for (key, val) in args.into_iter() {
                let evaluated_val =
                    global_context.render_string(&exec_context, event_payload, val.clone());
                evaluated_headers.insert(key.clone(), evaluated_val);
            }
            rest_action.headers = Some(evaluated_headers.clone());
        }

        exec_context.node.node_action.action_type = ActionType::Rest(rest_action);

        Ok(exec_context)
    }
}
