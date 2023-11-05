use std::{
    num::NonZeroUsize,
    ops::ControlFlow,
    sync::{Arc, Mutex},
};

use anything_graph::{Flow, NodeType, Task};
use anything_runtime::Scope;
use indexmap::IndexMap;
use tokio::sync::{mpsc::Sender, Semaphore};

use crate::{
    error::{CoordinatorError, CoordinatorResult},
    processing::{executor::run_task, sequence::Sequence},
};

use super::executor::execute;

#[derive(Debug, Clone)]
pub struct Processor {
    pub runtime_runner: anything_runtime::Runner,
    pub execution_scopes: IndexMap<String, Scope>,
    #[allow(unused)]
    flow: Flow,
}

#[allow(unused)]
impl Processor {
    pub fn new(runtime_runner: anything_runtime::Runner, flow: Flow) -> Self {
        Self {
            runtime_runner,
            execution_scopes: IndexMap::new(),
            flow,
        }
    }

    pub async fn run_graph(
        &mut self,
        results_tx: Sender<(String, Arc<Mutex<Scope>>)>,
        max_parallelism: Option<NonZeroUsize>,
    ) -> CoordinatorResult<()> {
        // Attach flow details

        let semaphore = max_parallelism.map(|max| Arc::new(Semaphore::new(max.get())));

        let graph = self
            .flow
            .into_graph()
            .expect("should be able to turn flow into a graph");

        let arc_graph = Arc::new(graph);
        let mut runner = self.runtime_runner.clone();

        self.flow.variables.vars.iter().for_each(|(key, value)| {
            runner.add_global_variable(key, Into::<String>::into(value.clone()).as_str());
        });
        self.flow.environment.vars.iter().for_each(|(key, value)| {
            runner.add_global_environment(key, Some(value.clone()));
        });

        let runtime_runner = Arc::new(Mutex::new(runner.clone()));

        let errors = execute(arc_graph, move |node: NodeType| {
            let semaphore = semaphore.clone();
            let mut runner = runtime_runner.clone();
            let results_tx = results_tx.clone();

            async move {
                let permit = if let Some(semaphore) = semaphore {
                    Some(
                        semaphore
                            .acquire_owned()
                            .await
                            .expect("semaphore must be open"),
                    )
                } else {
                    None
                };

                // TODO: handle async groups
                if let NodeType::Task(task) = node {
                    let task = Arc::new(task);
                    let res = run_task(runner.clone(), task.clone()).await;

                    match res {
                        Ok(task_result) => {
                            results_tx
                                .send((task.name.clone(), task_result.clone()))
                                .await
                                .expect("must send result");
                        }
                        Err(e) => {
                            return ControlFlow::Break::<(Arc<Task>, CoordinatorError)>((task, e));
                        }
                    }
                }

                drop(permit);

                ControlFlow::Continue(())
            }
        })
        .await;

        let errors_len: usize = errors.len();
        let empty_errors = errors.is_empty();

        for (is_last, (task, error)) in errors
            .into_values()
            .enumerate()
            .map(|(ix, bar)| (ix + 1 == errors_len, bar))
        {
            let seq = if is_last {
                Sequence::End
            } else {
                Sequence::Middle
            };
        }
        if empty_errors {
            Ok(())
        } else {
            Err(CoordinatorError::GraphRunTaskError)
        }
    }
}

#[cfg(test)]
mod tests {
    use anything_graph::{
        test_helper::{default_deno_run_options, default_system_run_options},
        TaskBuilder,
    };
    use anything_runtime::{Runner, RuntimeConfig};
    use tokio::sync::mpsc;

    use super::*;

    #[tokio::test]
    async fn test_processor_can_run_a_simple_graph() {
        let runtime_config: RuntimeConfig = RuntimeConfig::default();
        let tasks: Vec<Task> = vec![
            TaskBuilder::default()
                .name("node1".to_string())
                .run_options(default_deno_run_options(
                    "export default function() { return 'hello {{ name }}' }".to_string(),
                )),
            TaskBuilder::default()
                .name("node2".to_string())
                .depends_on(vec!["node1".to_string()])
                .run_options(default_system_run_options(
                    "echo 'hello back {{ node1.stdout }}'".to_string(),
                )),
        ]
        .into_iter()
        .map(|b| b.build().unwrap())
        .collect::<Vec<Task>>();

        let mut flow: Flow = setup_flow(tasks, runtime_config);
        flow.add_global_variable("name", "bob".to_string())
            .expect("must add global variable");
        let runner = setup_runner();

        let (results_tx, mut results_rx) = mpsc::channel::<(String, Arc<Mutex<Scope>>)>(16);

        let mut processor = Processor::new(runner, flow);
        let errs = processor.run_graph(results_tx.clone(), None).await;
        assert!(errs.is_ok());

        let r1 = results_rx.recv().await.expect("must receive result");
        let results = r1.1.lock().unwrap().results.clone();
        let node1_results = results.get(r1.0.as_str()).unwrap();
        assert_eq!(node1_results.stdout, "\"hello bob\"");
        let r2 = results_rx.recv().await.expect("must receive result");
        let results = r2.1.lock().unwrap().results.clone();
        let node2_results = results.get(r2.0.as_str()).unwrap();
        assert_eq!(node2_results.stdout, "hello back \"hello bob\"");
    }

    fn setup_flow(tasks: Vec<Task>, _runtime_config: RuntimeConfig) -> Flow {
        let mut flow: Flow = anything_graph::build_flow("some-flow".to_string());

        tasks.iter().for_each(|n| {
            flow.add_node(n.clone()).expect("must add task");
        });

        flow
    }

    fn setup_runner() -> Runner {
        let mut runtime_executor = anything_runtime::Runner::new(RuntimeConfig::default());
        runtime_executor
            .load_plugins()
            .expect("unable to load plugins");
        runtime_executor
    }
}
