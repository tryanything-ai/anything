use std::{
    collections::BTreeMap,
    future::Future,
    ops::ControlFlow,
    sync::{Arc, Mutex},
};

use anything_graph::Task;
use anything_runtime::{EngineKind, PluginEngine, Runner, Scope};
use petgraph::{
    algo::has_path_connecting,
    graph::{DiGraph, IndexType, NodeIndex},
    visit::{VisitMap, Visitable},
    Directed, Direction, Graph,
};
use tokio::{sync::mpsc, task::JoinSet, time::Instant};

use crate::error::{CoordinatorError, CoordinatorResult};

pub async fn execute<N, E, Ix, F, Fut, B>(
    graph: Arc<DiGraph<N, E, Ix>>,
    task: F,
) -> BTreeMap<Instant, B>
where
    N: Clone + Send + Sync + 'static,
    E: Send + Sync + 'static,
    Ix: IndexType + Send + Sync,
    F: Send + 'static + Fn(N) -> Fut,
    Fut: Future<Output = ControlFlow<B>> + Send + 'static,
    B: std::fmt::Debug + Send + Sync + 'static,
{
    // If there are no nodes, there is nothing to do
    if graph.node_count() == 0 {
        return BTreeMap::new();
    }

    let (visit_tx, visit_rx) = mpsc::channel(16);
    let (ready_tx, ready_rx) = mpsc::channel(16);

    let scheduler = tokio::spawn(scheduler(graph.clone(), visit_rx, ready_tx));

    let scheduler_handle = Arc::new(scheduler.abort_handle());

    let executor = tokio::spawn(executor(graph, task, ready_rx, visit_tx, scheduler_handle));

    scheduler.await.expect("should be able to join scheduler");
    executor.await.expect("should be able to join executor")
}

/// Consumes visited nodes and produces readied nodes based on the graph
async fn scheduler<N, E, Ix>(
    graph: Arc<Graph<N, E, Directed, Ix>>,
    mut visit_rx: mpsc::Receiver<(NodeIndex<Ix>, bool)>,
    ready_tx: mpsc::Sender<NodeIndex<Ix>>,
) where
    Ix: IndexType,
{
    let mut visit_map = graph.visit_map();

    for entrypoint in graph.externals(Direction::Incoming) {
        ready_tx
            .send(entrypoint)
            .await
            .expect("channel should still be open");
    }

    while let Some((visited, ok)) = visit_rx.recv().await {
        visit_map.visit(visited);

        let all_nodes_visited = graph.node_indices().all(|node| visit_map.is_visited(&node));

        if all_nodes_visited || !ok {
            // We're done!
            return;
        }

        let ready_nodes = graph
            .node_indices()
            .filter(|node| {
                // We only care about nodes connected to this
                // visited node
                has_path_connecting(graph.as_ref(), visited, *node, None)
            })
            .filter(|node| {
                // We only care about this node's dependency
                graph
                    .neighbors_directed(*node, Direction::Incoming)
                    .all(|node| visit_map.is_visited(&node))
            })
            .filter(|node| {
                // We don't want to revisit nodes
                !visit_map.is_visited(node)
            });

        for node in ready_nodes {
            ready_tx
                .send(node)
                .await
                .expect("channel should still be open");
        }
    }
}

/// Consumes and executes readied nodes and produces visited nodes
async fn executor<N, E, Ix, F, Fut, B>(
    graph: Arc<Graph<N, E, Directed, Ix>>,
    task: F,
    mut ready_rx: mpsc::Receiver<NodeIndex<Ix>>,
    visit_tx: mpsc::Sender<(NodeIndex<Ix>, bool)>,
    scheduler_handle: Arc<tokio::task::AbortHandle>,
) -> BTreeMap<Instant, B>
where
    N: Clone,
    Ix: IndexType + Send,
    F: Fn(N) -> Fut,
    Fut: Future<Output = ControlFlow<B>> + Send + 'static,
    B: std::fmt::Debug + Send + Sync + 'static,
{
    let mut join_set = JoinSet::new();

    while let Some(node) = ready_rx.recv().await {
        let task = task(graph[node].clone());

        let visit_tx = visit_tx.clone();
        let scheduler_handle = scheduler_handle.clone();
        join_set.spawn(async move {
            let control_flow = task.await;

            let result = visit_tx.send((node, control_flow.is_continue())).await;

            if !scheduler_handle.is_finished() {
                result.expect("channel should still be open");
            }

            (control_flow, Instant::now())
        });
    }

    let mut results = BTreeMap::new();

    // Join all tasks
    while let Some(result) = join_set.join_next().await {
        if let (ControlFlow::Break(x), exit_instant) = result.expect("should be able to join task")
        {
            results.insert(exit_instant, x);
        }
    }

    results
}

/// Try to run a task
pub async fn run_task(
    runner: Arc<Mutex<Runner>>,
    task: Arc<Task>,
) -> CoordinatorResult<Arc<Mutex<Scope>>> {
    let task_name = task.name.clone();

    let mut cfg: anything_runtime::ExecuteConfig = task
        .run_options
        .engine
        .as_ref()
        .unwrap()
        .to_owned()
        .try_into()
        .expect("unable to derive execution config");

    let engine = task.run_options.engine.as_ref().unwrap();
    println!("Engine in run_task: {:?}", engine);
    let plugin_name = if let EngineKind::PluginEngine(PluginEngine { engine, .. }) = engine {
        engine.clone()
    } else {
        "system-shell".to_string()
    };
    cfg.plugin_name = plugin_name;

    let mut runner = runner
        .try_lock()
        .map_err(|_e| CoordinatorError::RuntimeError)?;

    runner
        .execute(task_name, cfg)
        .map_err(|e| CoordinatorError::RunnerError(e))
}

#[cfg(test)]
mod tests {
    use anything_graph::{test_helper::default_system_run_options, Flow, NodeType, TaskBuilder};
    use anything_runtime::RuntimeConfig;

    use crate::processing::sequence::Sequence;

    use super::*;

    #[tokio::test]
    async fn test_execute_execs_a_single_shell_task() {
        let task = TaskBuilder::default()
            .name("node1".to_string())
            .run_options(default_system_run_options("echo 'Hello world'".to_string()))
            .build()
            .unwrap();

        let mut runtime_executor = anything_runtime::Runner::new(RuntimeConfig::default());
        runtime_executor
            .load_plugins()
            .expect("unable to load plugins");

        let res = run_task(Arc::new(Mutex::new(runtime_executor)), Arc::new(task)).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let res = res.try_lock().unwrap();
        assert_eq!(res.get_result("node1").unwrap().stdout, "Hello world");
    }

    #[tokio::test]
    async fn test_execute_execs_a_single_shell_task_with_a_variable() {
        let task = TaskBuilder::default()
            .name("node1".to_string())
            .run_options(default_system_run_options(
                "echo 'Hello {{name}}'".to_string(),
            ))
            .build()
            .unwrap();

        let mut runtime_executor = anything_runtime::Runner::new(RuntimeConfig::default());
        runtime_executor
            .load_plugins()
            .expect("unable to load plugins");

        runtime_executor.add_global_variable("name", "bobby");

        let res = run_task(Arc::new(Mutex::new(runtime_executor)), Arc::new(task)).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let res = res.try_lock().unwrap();
        assert_eq!(res.get_result("node1").unwrap().stdout, "Hello bobby");
    }

    #[tokio::test]
    async fn test_execute_execs_a_single_shell_task_with_an_environment_variable() {
        let task = TaskBuilder::default()
            .name("node1".to_string())
            .run_options(default_system_run_options(
                "echo 'my path is {{PATH}}'".to_string(),
            ))
            .build()
            .unwrap();

        let mut runtime_executor = anything_runtime::Runner::new(RuntimeConfig::default());
        runtime_executor
            .load_plugins()
            .expect("unable to load plugins");

        runtime_executor.add_global_environment("PATH", Some("/usr/bin".to_string()));

        let res = run_task(Arc::new(Mutex::new(runtime_executor)), Arc::new(task)).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        let res = res.try_lock().unwrap();
        assert_eq!(
            res.get_result("node1").unwrap().stdout,
            "my path is /usr/bin"
        );
    }

    #[tokio::test]
    async fn test_execute_a_simple_graph_successfully() {
        let runtime_config = RuntimeConfig::default();
        let tasks: Vec<Task> = vec![
            TaskBuilder::default()
                .name("node1".to_string())
                .run_options(default_system_run_options(
                    "echo 'Hello {{name}}'".to_string(),
                )),
            TaskBuilder::default()
                .name("node2".to_string())
                .depends_on(vec!["node1".to_string()])
                .run_options(default_system_run_options(
                    "echo 'got: {{node1.stdout}}'".to_string(),
                )),
        ]
        .into_iter()
        .map(|b| b.build().unwrap())
        .collect::<Vec<Task>>();

        let mut flow: Flow = setup_flow(tasks, runtime_config);
        flow.add_global_variable("name", "bob".to_string())
            .expect("must add global variable");

        let mut runner = setup_runner();
        runner.add_global_variable("name", "bob");

        let runner = Arc::new(Mutex::new(runner));
        let (results_tx, mut results_rx) = mpsc::channel::<(String, Arc<Mutex<Scope>>)>(16);

        let graph = Arc::new(flow.into_graph().unwrap());
        let graph: Arc<Graph<NodeType, usize>> = graph;

        // Helper runner
        let errors = run_graph_helper(graph, runner, results_tx).await;

        let len_errors = errors.len();
        assert_eq!(len_errors, 0);

        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = res.unwrap();
        let node2_res = res.1.try_lock().unwrap();

        let result = node2_res.get_result(res.0.as_str()).unwrap();
        assert_eq!(result.stdout, "got: Hello bob")
    }

    #[tokio::test]
    async fn test_execute_a_complex_graph_successfully() {
        let runtime_config = RuntimeConfig::default();
        let tasks: Vec<Task> = vec![
            TaskBuilder::default()
                .name("node1".to_string())
                .run_options(default_system_run_options(
                    "echo 'Hello {{name}}'".to_string(),
                )),
            TaskBuilder::default()
                .name("node2".to_string())
                .depends_on(vec!["node1".to_string()])
                .run_options(default_system_run_options(
                    "echo 'got: {{node1.stdout}}'".to_string(),
                )),
            TaskBuilder::default()
                .name("node3".to_string())
                .depends_on(vec!["node2".to_string()])
                .run_options(default_system_run_options(
                    "echo 'got: {{node2.stdout}}'".to_string(),
                )),
            TaskBuilder::default()
                .name("node4".to_string())
                .depends_on(vec!["node2".to_string(), "node3".to_string()])
                .run_options(default_system_run_options(
                    "echo 'finally got: {{node3.stdout}} and {{node2.stdout}}'".to_string(),
                )),
        ]
        .into_iter()
        .map(|b| b.build().unwrap())
        .collect::<Vec<Task>>();

        let mut flow: Flow = setup_flow(tasks, runtime_config);
        flow.add_global_variable("name", "bob".to_string())
            .expect("must add global variable");

        let mut runner = setup_runner();
        runner.add_global_variable("name", "bob");

        let runner = Arc::new(Mutex::new(runner));
        let (results_tx, mut results_rx) = mpsc::channel::<(String, Arc<Mutex<Scope>>)>(16);

        let graph = Arc::new(flow.into_graph().unwrap());
        let graph: Arc<Graph<NodeType, usize>> = graph;

        // Helper runner
        let errors = run_graph_helper(graph, runner, results_tx).await;

        let len_errors = errors.len();
        assert_eq!(len_errors, 0);

        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = results_rx.recv().await;
        assert!(res.is_some());
        let res = res.unwrap();
        let node2_res = res.1.try_lock().unwrap();

        let result = node2_res.get_result(res.0.as_str()).unwrap();
        assert_eq!(
            result.stdout,
            "finally got: got: got: Hello bob and got: Hello bob"
        )
    }

    #[tokio::test]
    async fn test_execute_a_graph_with_an_error() {
        let runtime_config: RuntimeConfig = RuntimeConfig::default();
        let tasks: Vec<Task> = vec![
            TaskBuilder::default()
                .name("node1".to_string())
                .run_options(default_system_run_options(
                    "echo 'Hello {{name}}: {{greetings}}'".to_string(),
                )),
            TaskBuilder::default()
                .name("node2".to_string())
                .depends_on(vec!["node1".to_string()])
                .run_options(default_system_run_options(
                    "echo 'got: {{node1.stdout}}'".to_string(),
                )),
        ]
        .into_iter()
        .map(|b| b.build().unwrap())
        .collect::<Vec<Task>>();

        let mut flow: Flow = setup_flow(tasks, runtime_config);
        flow.add_global_variable("name", "bob".to_string())
            .expect("must add global variable");
        let mut runner = setup_runner();
        runner.add_global_variable("name", "bob");

        let runner = Arc::new(Mutex::new(runner));

        let graph = Arc::new(flow.into_graph().unwrap());
        let errors = execute(graph, move |node| {
            let runner = Arc::clone(&runner);

            async move {
                if let NodeType::Task(task) = node {
                    let task = Arc::new(task);
                    let res = run_task(runner.clone(), task.clone()).await;

                    match res {
                        Ok(_task_result) => ControlFlow::Continue(()),
                        Err(e) => {
                            // Handle the error
                            ControlFlow::Break::<(Arc<Task>, CoordinatorError)>((task, e))
                        }
                    }
                } else {
                    ControlFlow::Continue(())
                }
            }
        })
        .await;

        let len_errors = errors.len();

        for (is_last, (_task, _error)) in errors
            .into_values()
            .enumerate()
            .map(|(i, arc_task)| (i + 1 == len_errors, arc_task))
        {
            if is_last {
                Sequence::End
            } else {
                Sequence::Middle
            };
        }
        assert_eq!(len_errors, 1);
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

    async fn run_graph_helper(
        graph: Arc<Graph<NodeType, usize>>,
        runner: Arc<Mutex<Runner>>,
        results_tx: mpsc::Sender<(String, Arc<Mutex<Scope>>)>,
    ) -> BTreeMap<tokio::time::Instant, (std::sync::Arc<anything_graph::Task>, CoordinatorError)>
    {
        let errors = execute(graph, move |node| {
            let runner = Arc::clone(&runner);
            let results_tx = results_tx.clone();

            async move {
                let runner = runner.clone();

                match node {
                    NodeType::Task(task) => {
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
                                return ControlFlow::Break::<(Arc<Task>, CoordinatorError)>((
                                    task, e,
                                ));
                            }
                        }
                    }
                    _ => {}
                }

                ControlFlow::Continue(())
            }
        })
        .await;
        errors
    }
}
