use std::sync::{Arc, Mutex};

use anything_graph::flow::{flow::Flow, node::Node};
// use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    context::{ExecutionContext, NodeExecutionContext},
    engines::get_engine,
    error::EngineResult,
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
            outputs: Vec::default(),
        };
        Self {
            context: Arc::new(Mutex::new(context)),
        }
    }

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
        }

        Ok(())
    }

    pub fn run_node(&mut self, node: Node) -> EngineResult<NodeExecutionContext> {
        let mut node_execution = NodeExecutionContext {
            node: node.clone(),
            status: None,
            process: None,
        };
        let mut engine = get_engine(node);
        match engine.run(&self.context.lock().unwrap()) {
            Err(e) => return Err(e),
            Ok(_executed) => {
                if let Some(process) = engine.process() {
                    node_execution.process = Some(process.clone());
                    node_execution.status = process.state.status;
                }
            }
        }
        Ok(node_execution)
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
        let _run = executor.run();
        // let context = executor.context.lock().unwrap();
        // println!("context: {:?}", context);
        // TODO: finish along the executor chain
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
