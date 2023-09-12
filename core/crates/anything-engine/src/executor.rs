use std::sync::Arc;

use anything_graph::flow::{flow::Flow, node::Node};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    engines::get_engine,
    error::EngineResult,
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Executor {
    pub context: Arc<ExecutionContext>,
}

impl Executor {
    pub fn new(flow: &Flow) -> Self {
        let uuid = Uuid::new_v4();
        let executed: Vec<NodeExecutionContext> = Vec::default();
        let context = ExecutionContext {
            uuid,
            flow: flow.clone(),
            executed,
        };
        Self {
            context: Arc::new(context),
        }
    }

    pub fn run_node(&mut self, node: Node) -> EngineResult<NodeExecutionContext> {
        let mut node_execution = NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        };
        let mut engine = get_engine(node);
        match engine.run(&self.context) {
            Err(e) => return Err(e),
            Ok(_executed) => {
                let process = engine.get_process()?;
                node_execution.process = Some(process.clone());
                node_execution.status = process.state.status;
            }
        }
        Ok(node_execution)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use anything_graph::flow::{
        action::{ActionBuilder, ActionType, ShellActionBuilder},
        flowfile::Flowfile,
        node::NodeBuilder,
    };

    use super::*;

    #[tokio::test]
    async fn test_run_node_executes_and_returns_node_execution() -> Result<()> {
        let flow = Flowfile::from_file(PathBuf::from("tests/fixtures/simple.toml"))
            .unwrap()
            .flow;
        let executor = Executor::new(&flow);
        // TODO: finish along the executor chain
        Ok(())
    }

    fn demo_flow() -> Flow {
        let mut flow = Flow::new();

        flow.add_node_obj(
            &NodeBuilder::default()
                .id("holiday-cheer")
                .name("holiday cheer")
                .node_action(
                    ActionBuilder::default()
                        .action_type(ActionType::Shell(
                            ShellActionBuilder::default()
                                .command("echo 'ducks")
                                .build()
                                .unwrap(),
                        ))
                        .display_name("holiday cheer")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .unwrap();

        flow.add_node_obj(
            &NodeBuilder::default()
                .id("echo-cheer")
                .name("echo holiday cheer")
                .node_action(
                    ActionBuilder::default()
                        .action_type(ActionType::Shell(
                            ShellActionBuilder::default()
                                .command("echo '{{ cheer }}")
                                .build()
                                .unwrap(),
                        ))
                        .display_name("holiday cheer")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        )
        .unwrap();

        flow
    }
}
