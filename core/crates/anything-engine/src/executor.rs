use std::sync::{Arc, Mutex};

use anything_graph::flow::{flow::Flow, node::Node};
// use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    engines::get_engine,
    error::EngineResult,
    types::ProcessState,
};

#[derive(Debug, Clone, Default)]
pub struct Executor {
    pub context: Arc<Mutex<ExecutionContext>>,
}

impl Executor {
    pub fn new(flow: &Flow) -> Self {
        let uuid = Uuid::new_v4();
        let executed: Vec<NodeExecutionContext> = Vec::default();
        let context = ExecutionContext {
            uuid,
            flow: flow.clone(),
            executed: executed.clone(),
            latest_output: ProcessState::default(),
        };
        Self {
            context: Arc::new(Mutex::new(context)),
        }
    }

    // TODO: parallelize this by breaking each flow "level" and concurrently execute when possible
    // For now this is a simple linear execution
    pub fn run(&mut self) -> EngineResult<()> {
        let execution_nodes = self
            .context
            .lock()
            .unwrap()
            .flow
            .get_node_execution_list(None)?;

        for node in execution_nodes.into_iter() {
            let node_execution = self.run_node(node)?;
            self.context
                .lock()
                .unwrap()
                .executed
                .push(node_execution.clone());
            let process = node_execution.process.unwrap();
            self.context.lock().unwrap().latest_output = process.state.clone();
        }

        Ok(())
    }

    pub fn run_node(&mut self, node: Node) -> EngineResult<NodeExecutionContext> {
        let mut engine = get_engine(node.clone());
        let mut node_execution_context =
            engine.render(&node, &self.context.lock().unwrap().clone())?;

        // Call the engine to run the node
        match engine.run(&node_execution_context) {
            Err(e) => return Err(e),
            Ok(_executed) => {
                if let Some(process) = engine.process() {
                    node_execution_context.process = Some(process.clone());
                    node_execution_context.status = process.state.status;
                }
            }
        }
        Ok(node_execution_context)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use anyhow::Result;
    use anything_graph::flow::flowfile::Flowfile;

    use super::*;

    #[tokio::test]
    async fn test_run_node_executes_and_returns_node_execution() -> Result<()> {
        let flow = Flowfile::from_file(PathBuf::from("tests/fixtures/simple.toml"))
            .unwrap()
            .flow;
        let mut executor = Executor::new(&flow);
        let run = executor.run();
        assert!(run.is_ok());
        let context = executor.context.lock().unwrap(); // Get the final output
        assert_eq!(context.executed.len(), 2);

        println!("context last output: {:?}", context.latest_output.stdout);
        Ok(())
    }
}

/*
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
*/
