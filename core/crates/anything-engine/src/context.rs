use std::collections::HashMap;

use anything_graph::flow::{
    flow::Flow,
    node::{Node, NodeState},
};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use tera::Tera;
use uuid::Uuid;

use crate::types::Process;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ExecutionContext {
    pub uuid: Uuid,
    pub flow: Flow,
    pub executed: Vec<NodeExecutionContext>,
    pub outputs: Vec<String>,
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
            outputs: Vec::default(),
        };
        context
    }

    // pub fn render_string_for_node(
    //     &self,
    //     node: NodeExecutionContext,
    //     engine: Box<dyn Engine>,
    // ) -> String {
    //     let context = self.build_render_context(node.clone());
    //     let mut tera = Tera::default();

    //     // let command = node.node_action.command.as_str();

    //     let mut output = String::new();
    //     output
    // }

    pub fn render_string(&self, node: &NodeExecutionContext, string: String) -> String {
        let mut tera = Tera::default();
        let context = self.build_render_context(node.clone());
        tera.add_raw_template("string", &string).unwrap();
        tera.render("string", &context).unwrap()
    }

    fn build_render_context(&self, node: NodeExecutionContext) -> tera::Context {
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

        // Next handle the node specific variables
        for (key, value) in node.node.variables.iter() {
            context.insert(key, value);
        }

        context
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use anything_graph::flow::{flow::FlowBuilder, node::NodeBuilder};

    use crate::types::{ProcessBuilder, ProcessStateBuilder};

    use super::*;

    #[test]
    fn test_render_variables_are_available_from_global_flow_scope() {
        let flow = FlowBuilder::default()
            .variables(HashMap::from([("name".to_string(), "ducks".to_string())]))
            .build()
            .unwrap();

        let exec_context = ExecutionContext::new(&flow);

        let node_exec = NodeExecutionContext::default();
        let result = exec_context.render_string(&node_exec, "Hello {{name}}".to_string());
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
        let result = execution_context.render_string(&node, "Hello {{name}}".to_string());
        assert_eq!(result, "Hello mantalope");
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

        let result = execution_context
            .render_string(&node_exec2, r#"Hello {{simple['stdout']}}"#.to_string());
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

        let result =
            execution_context.render_string(&node_exec2, r#"Hello {{simple.stderr}}"#.to_string());
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

        let result =
            execution_context.render_string(&node_exec2, r#"Hello {{simple.status}}"#.to_string());
        assert_eq!(result, "Hello success");
    }
}
