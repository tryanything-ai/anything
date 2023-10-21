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
    pub async fn run(&mut self, event_payload: &serde_json::Value) -> EngineResult<()> {
        let execution_nodes = self
            .context
            .lock()
            .unwrap()
            .flow
            .get_node_execution_list(None)?;

        for node in execution_nodes.into_iter() {
            let node_execution = self.run_node(node, event_payload).await?;
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

    pub async fn run_node(
        &mut self,
        node: Node,
        event_payload: &serde_json::Value,
    ) -> EngineResult<NodeExecutionContext> {
        let mut engine = get_engine(node.clone());
        let mut node_execution_context =
            engine.render(&node, &self.context.lock().unwrap().clone(), event_payload)?;

        // Call the engine to run the node
        match engine.run(&node_execution_context).await {
            Err(e) => return Err(e),
            Ok(process) => {
                node_execution_context.process = Some(process.clone());
                node_execution_context.status = process.state.status;
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

    use crate::test_helper::test_payload;

    use super::*;

    #[tokio::test]
    async fn test_run_node_executes_and_returns_node_execution() -> Result<()> {
        let flow = Flowfile::from_file(PathBuf::from("tests/fixtures/simple.toml"))
            .unwrap()
            .flow;
        let mut executor = Executor::new(&flow);
        let run = executor.run(&test_payload()).await;
        assert!(run.is_ok());
        let context = executor.context.lock().unwrap(); // Get the final output
        assert_eq!(context.executed.len(), 2);

        assert!(!context.latest_output.stdout.is_none());

        let latest_output = context.latest_output.stdout.as_ref().unwrap().clone();
        assert_eq!(latest_output, "Heres my cheers: Jingle Bells".to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_run_with_rest_node() -> Result<()> {
        let flow = Flowfile::from_file(PathBuf::from("tests/fixtures/with_rest.toml"))
            .unwrap()
            .flow;
        let mut executor = Executor::new(&flow);
        let run = executor.run(&test_payload()).await;
        assert!(run.is_ok());
        let context = executor.context.lock().unwrap(); // Get the final output
        assert_eq!(context.executed.len(), 2);

        let latest_output = context.latest_output.stdout.as_ref().unwrap().clone();
        assert!(latest_output.contains("latitude"));
        assert!(latest_output.contains("timezone"));

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
