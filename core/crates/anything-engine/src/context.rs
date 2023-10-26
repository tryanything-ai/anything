use std::collections::HashMap;

use anything_graph::flow::{
    flow::Flow,
    node::{Node, NodeState},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tera::Tera;
use uuid::Uuid;

use crate::{
    engines::Engine,
    types::{Process, ProcessState},
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ExecutionContext {
    pub uuid: Uuid,
    pub flow: Flow,
    pub executed: Vec<NodeExecutionContext>,
    pub latest_output: ProcessState,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct NodeExecutionContext {
    pub node: Node,
    pub status: Option<NodeState>,
    pub process: Option<Process>,
}

impl ExecutionContext {
    pub fn new(flow: &Flow) -> Self {
        let uuid = Uuid::new_v4();
        let executed: Vec<NodeExecutionContext> = Vec::default();
        let context = ExecutionContext {
            uuid,
            flow: flow.clone(),
            executed: executed.clone(),
            latest_output: ProcessState::default(),
        };
        context
    }

    pub fn render_string_for_node(
        &mut self,
        node: Node,
        event_payload: &serde_json::Value,
        mut engine: Box<dyn Engine + Send>,
    ) -> NodeExecutionContext {
        engine
            .render(&node, &self, event_payload)
            .expect("unable to render node")
    }

    pub fn render_string(
        &self,
        node: &NodeExecutionContext,
        event_payload: &serde_json::Value,
        string: String,
    ) -> String {
        let mut context = self.build_global_render_context();
        let mut context = self.build_node_render_context(&mut context, &node);
        let mut context = self.build_payload_render_context(&mut context, event_payload);

        self.render_with_tera(&mut context, string)
    }

    fn render_with_tera(&self, context: &mut tera::Context, string: String) -> String {
        let mut tera = Tera::default();
        tera.add_raw_template("string", &string).unwrap();
        match tera.render("string", &context) {
            Ok(rendered) => rendered,
            Err(e) => {
                tracing::error!("Error rendering string: {:?}", e);
                string
            }
        }
    }

    fn build_payload_render_context(
        &self,
        context: &mut tera::Context,
        payload: &serde_json::Value,
    ) -> tera::Context {
        if !payload.is_object() {
            return context.clone();
        }
        let mut executed_context: HashMap<String, &str> = HashMap::new();
        for (key, value) in payload.as_object().unwrap().iter() {
            executed_context.insert(key.clone(), value.as_str().unwrap());
        }
        context.insert("payload", &executed_context);
        context.clone()
    }

    fn build_node_render_context(
        &self,
        context: &mut tera::Context,
        node: &NodeExecutionContext,
    ) -> tera::Context {
        for (key, value) in node.node.variables.iter() {
            let rendered_val = self.render_with_tera(context, value.clone());
            context.insert(key, &rendered_val);
        }
        context.clone()
    }

    fn build_global_render_context(&self) -> tera::Context {
        let mut context = tera::Context::new();

        // Process global variables and environment
        for (key, value) in self.flow.variables.iter() {
            context.insert(key, value);
        }

        // Process previous outputs
        self.executed.iter().for_each(|node| {
            if let Some(process) = node.process.clone() {
                // Add stderr, stdout, status
                let mut executed_context: HashMap<String, String> = HashMap::new();
                executed_context.insert(
                    "stdout".to_string(),
                    process.state.stdout.unwrap_or_default().clone(),
                );
                executed_context.insert(
                    "stderr".to_string(),
                    process.state.stderr.unwrap_or_default().clone(),
                );
                executed_context.insert(
                    "status".to_string(),
                    process.state.status.unwrap_or_default().into(),
                );
                context.insert(&node.node.name, &executed_context);
            }
        });

        context
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anything_graph::flow::{flow::FlowBuilder, node::NodeBuilder};

    use crate::{
        test_helper::test_payload,
        types::{ProcessBuilder, ProcessStateBuilder},
    };

    use super::*;

    #[test]
    fn test_rendering_node_command_string() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let exec_context = ExecutionContext::new(&flow);

        let node_exec = NodeExecutionContext::default();
        let result =
            exec_context.render_string(&node_exec, &test_payload(), "Hello {{name}}".to_string());
        assert_eq!(result, "Hello ducks");
    }

    #[test]
    fn test_render_variables_are_available_from_global_flow_scope() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let exec_context = ExecutionContext::new(&flow);

        let node_exec = NodeExecutionContext::default();
        let result =
            exec_context.render_string(&node_exec, &test_payload(), "Hello {{name}}".to_string());
        assert_eq!(result, "Hello ducks");
    }

    #[test]
    fn test_render_variables_are_available_from_node_scope() {
        let flow = FlowBuilder::default().build().unwrap();

        let execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .variables(HashMap::from([(
                "name".to_string(),
                "mantalope".to_string(),
            )]))
            .build()
            .unwrap();

        let node = NodeExecutionContextBuilder::default()
            .node(node)
            .build()
            .unwrap();
        let result =
            execution_context.render_string(&node, &test_payload(), "Hello {{name}}".to_string());
        assert_eq!(result, "Hello mantalope");
    }

    #[test]
    fn test_render_variables_are_available_from_payload() {
        let flow = FlowBuilder::default().build().unwrap();

        let execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .variables(HashMap::from([(
                "name".to_string(),
                "mantalope".to_string(),
            )]))
            .build()
            .unwrap();

        let node = NodeExecutionContextBuilder::default()
            .node(node)
            .build()
            .unwrap();

        let mut payload = test_payload();
        payload.as_object_mut().unwrap().insert(
            "name".to_string(),
            serde_json::Value::String("ari".to_string()),
        );

        let result = execution_context.render_string(
            &node,
            &payload,
            "Hello {{payload.name}} {{name}}".to_string(),
        );
        assert_eq!(result, "Hello ari mantalope");
    }

    #[test]
    fn test_render_variables_are_rendered_and_available() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([(
                "boys".to_string(),
                "will be boys".to_string(),
            )]))
            .build()
            .unwrap();

        let execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .variables(HashMap::from([(
                "name".to_string(),
                "{{ boys }}".to_string(),
            )]))
            .build()
            .unwrap();

        let node = NodeExecutionContextBuilder::default()
            .node(node)
            .build()
            .unwrap();
        let result =
            execution_context.render_string(&node, &test_payload(), "Hello {{name}}".to_string());
        assert_eq!(result, "Hello will be boys");
    }

    #[test]
    fn test_render_variables_are_available_from_executed_nodes_stdout() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let mut execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .name("simple".to_string())
            .variables(HashMap::from([(
                "name".to_string(),
                "mantalope".to_string(),
            )]))
            .build()
            .unwrap();

        let node_exec = NodeExecutionContextBuilder::default()
            .node(node)
            .process(
                ProcessBuilder::default()
                    .state(
                        ProcessStateBuilder::default()
                            .stdout("output_from_dummy_data")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        execution_context.executed.push(node_exec);

        let node_exec2 = NodeExecutionContextBuilder::default()
            .node(
                NodeBuilder::default()
                    .name("simpletwo".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = execution_context.render_string(
            &node_exec2,
            &test_payload(),
            r#"Hello {{simple['stdout']}}"#.to_string(),
        );
        assert_eq!(result, "Hello output_from_dummy_data");
    }

    #[test]
    fn test_render_variables_are_available_from_executed_nodes_stderr() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let mut execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .name("simple".to_string())
            .build()
            .unwrap();

        let node_exec = NodeExecutionContextBuilder::default()
            .node(node)
            .process(
                ProcessBuilder::default()
                    .state(
                        ProcessStateBuilder::default()
                            .stderr("error_from_dummy_data")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        execution_context.executed.push(node_exec);

        let node_exec2 = NodeExecutionContextBuilder::default()
            .node(
                NodeBuilder::default()
                    .name("simpletwo".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = execution_context.render_string(
            &node_exec2,
            &test_payload(),
            r#"Hello {{simple.stderr}}"#.to_string(),
        );
        assert_eq!(result, "Hello error_from_dummy_data");
    }

    #[test]
    fn test_render_variables_are_available_from_executed_nodes_status() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let mut execution_context = ExecutionContext::new(&flow);

        let node = NodeBuilder::default()
            .name("simple".to_string())
            .build()
            .unwrap();

        let node_exec = NodeExecutionContextBuilder::default()
            .node(node)
            .process(
                ProcessBuilder::default()
                    .state(
                        ProcessStateBuilder::default()
                            .status(NodeState::Success)
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        execution_context.executed.push(node_exec);

        let node_exec2 = NodeExecutionContextBuilder::default()
            .node(
                NodeBuilder::default()
                    .name("simpletwo".to_string())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        let result = execution_context.render_string(
            &node_exec2,
            &test_payload(),
            r#"Hello {{simple.status}}"#.to_string(),
        );
        assert_eq!(result, "Hello success");
    }
}
