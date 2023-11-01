#![allow(unused)]
use std::collections::HashMap;

use indexmap::IndexMap;
use petgraph::{
    algo::tarjan_scc, data::Build, graph::NodeIndex, prelude::DiGraph, stable_graph::IndexType,
};

use crate::error::{Cycle, GraphError, GraphResult};

use super::node::{GroupId, NodeGroup, NodeId, NodeType, Task};

#[derive(Debug, Clone, Default)]
pub struct FlowGraph {
    pub nodes: HashMap<NodeId, Task>,
    pub inorder_nodes: Vec<NodeId>,
    node_name_to_node_id: HashMap<String, NodeId>,
    lookup_table_nodes_to_graph: HashMap<NodeId, NodeIndex>,
    lookup_graph_index_to_node: HashMap<NodeIndex, NodeId>,
    graph: DiGraph<NodeType, NodeId>,
}

impl FlowGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            inorder_nodes: Vec::default(),
            nodes: HashMap::new(),
            node_name_to_node_id: HashMap::new(),
            lookup_table_nodes_to_graph: HashMap::new(),
            lookup_graph_index_to_node: HashMap::new(),
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> Option<&Task> {
        self.nodes.get(&node_id)
    }

    pub fn add_node(&mut self, node: Task) {
        let node_id = node.id.clone();
        self.inorder_nodes.push(node_id);
        self.nodes.insert(node_id, node.clone());
        let idx = self.graph.add_node(NodeType::Task(node.clone()));
        // Eventually simplify this mofo
        // self.lookup_table_nodes_to_graph.insert(node_id, idx);
        // self.lookup_graph_index_to_node.insert(idx, node_id);
        self.node_name_to_node_id.insert(node.name, node.id);
    }

    pub fn into_graph(&mut self) -> GraphResult<DiGraph<NodeType, usize>> {
        let mut graph = DiGraph::new();

        let mut node_to_index = HashMap::new();
        let mut node_to_group = HashMap::new();
        let mut node_to_node_group = HashMap::new();
        // let mut group_to_graph_index = HashMap::new();

        // TODO: make sure we don't have any cyclic entries in `calculate_groups()`
        let groups = self.calculate_groups().unwrap_or(vec![]);

        let group_node_prefix =
            |group_id: NodeId, node_id: NodeId| format!("{}-{}", group_id, node_id);

        // Add all the possible nodes to the hashmap before we start moving to the graph
        for group in groups.iter().clone() {
            let node = group.node.clone();
            let node_idx = graph.add_node(NodeType::Task(node.clone()));
            node_to_index.insert(node.id.clone(), node_idx);
            node_to_group.insert((group.id.clone(), node.id.clone()), group.id.clone());
            node_to_node_group.insert(node.id.clone(), group.id.clone());
        }

        for group in groups.iter().cloned() {
            // add all nodes to the storage
            let node = group.node.clone();
            let node_idx = node_to_index.get(&node.id).unwrap();

            for dep in group.depends.iter() {
                // Add the dep as an edge
                match node_to_index.get(&dep.id.clone()) {
                    None => {
                        // We should never get here
                        return Err(GraphError::NodeNotFound {
                            group: group.name.clone(),
                            node: dep.name.clone(),
                        });
                    }
                    Some(dep_idx) => {
                        graph.add_edge(*dep_idx, *node_idx, 1);
                    }
                }

                // graph.add_edge(group_start_index, node_idx, 1);
                // graph.add_edge(node_idx, group_end_index, 1);
            }
        }

        Self::ensure_acyclic(&graph)?;
        self.graph = graph.clone();

        Ok(graph)
    }

    pub fn calculate_groups(&mut self) -> GraphResult<Vec<NodeGroup>> {
        let mut node_groups = Vec::new();

        let mut node_deps: IndexMap<&NodeId, Vec<&NodeId>> = IndexMap::new();
        // This is terrible performance
        for (node_id, node) in self.nodes.iter() {
            for dep in node.depends_on.iter() {
                let dep_id = self.node_name_to_node_id.get(dep).unwrap();
                node_deps.entry(node_id).or_insert(Vec::new()).push(dep_id);
            }
        }

        let mut node_deps = node_deps.clone();
        for node_id in self.inorder_nodes.iter() {
            let nodes: Vec<Task> = node_deps
                .get(&node_id)
                .unwrap_or(&vec![])
                .into_iter()
                .map(|node_id| self.nodes.get(node_id).unwrap())
                .map(|node| node.clone())
                .collect();

            let node = self.nodes.get(node_id).unwrap().clone();
            let name = format!("{}-{}", node_id.clone(), node_id);

            let nodes = nodes
                .clone()
                .iter()
                .map(|mut n| {
                    let mut n = n.clone();
                    n.group = Some(name.clone());
                    n
                })
                .collect::<Vec<Task>>();

            let node_group = NodeGroup {
                id: node_id.clone(),
                name,
                node,
                depends: nodes,
            };
            node_groups.push(node_group);
        }

        Ok(node_groups)
    }

    pub fn ensure_acyclic<E, Ix>(graph: &DiGraph<NodeType, E, Ix>) -> GraphResult<()>
    where
        Ix: IndexType,
    {
        let sccs = tarjan_scc(graph)
            .into_iter()
            .filter(|scc| {
                scc.len() > 1
                    || scc.iter().copied().fold(false, |acc, node| {
                        graph.find_edge_undirected(node, node).is_some() || acc
                    })
            })
            .map(|scc| {
                scc.into_iter()
                    .map(|node| graph[node].clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if sccs.is_empty() {
            Ok(())
        } else {
            Err(GraphError::CycleError(Cycle { sccs }))
        }
    }
}

#[cfg(test)]
mod tests {
    use anything_runtime::{
        EngineKind, EngineOption, PluginEngine, RawEnvironment, RawNode, RawVariables,
    };
    use indexmap::indexmap;
    use petgraph::{
        dot::{Config, Dot},
        Graph,
    };

    use crate::{
        graph::{
            flow::FlowBuilder, flow_graph::test_helper::build_task, flowfile::Flowfile,
            node::TaskBuilder,
        },
        Flow,
    };

    use super::*;

    #[test]
    fn test_compiled_tiny_graph_adds_nodes_correctly() {
        let mut flow = build_flow("DemoFlow".to_string());
        let mut node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());

        // Setup dependencies
        node2.depends_on = vec!["node1".to_string()];

        let mut flow_graph = FlowGraph::default();
        flow_graph.add_node(node1.clone());
        flow_graph.add_node(node2.clone());

        let graph = flow_graph.into_graph();
        assert!(graph.is_ok());
        let graph = graph.unwrap();
        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_compiled_graph_adds_nodes_correctly() {
        let mut flow = build_flow("DemoFlow".to_string());
        let mut node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());
        let mut node3 = build_task("node3".to_string(), "three.js".to_string());
        let mut node4 = build_task("node4".to_string(), "four.js".to_string());
        let mut node5 = build_task("node5".to_string(), "five.js".to_string());

        // Setup dependencies
        node2.depends_on = vec!["node1".to_string()];
        node3.depends_on = vec!["node2".to_string()];
        node4.depends_on = vec!["node2".to_string()];
        node5.depends_on = vec!["node4".to_string(), "node3".to_string()];

        let mut flow_graph = FlowGraph::default();
        flow_graph.add_node(node1.clone());
        flow_graph.add_node(node2.clone());
        flow_graph.add_node(node3.clone());
        flow_graph.add_node(node4.clone());
        flow_graph.add_node(node5.clone());

        let graph = flow_graph.into_graph();
        assert!(graph.is_ok());
        let graph = graph.unwrap();
        assert_eq!(graph.node_count(), 5);
    }

    #[test]
    fn test_throws_cyclic_error() {
        let mut flow = build_flow("DemoFlow".to_string());
        let mut node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());

        // Setup dependencies
        node1.depends_on = vec!["node1".to_string()];
        node2.depends_on = vec!["node1".to_string(), "node2".to_string()];

        let mut flow_graph = FlowGraph::default();
        flow_graph.add_node(node1.clone());
        flow_graph.add_node(node2.clone());

        let graph = flow_graph.into_graph();
        assert!(graph.is_err());
        let err = graph.unwrap_err();
    }

    #[test]
    fn test_calculate_groups() {
        let mut flow = build_flow("DemoFlow".to_string());
        let mut root = build_task("root_node".to_string(), "".to_string());
        let mut node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());
        let mut node3 = build_task("node3".to_string(), "three.js".to_string());
        let mut node4 = build_task("node4".to_string(), "four.js".to_string());
        let mut node5 = build_task("node5".to_string(), "five.js".to_string());

        // Setup dependencies
        node1.depends_on = vec!["root_node".to_string()];
        node2.depends_on = vec!["node1".to_string()];
        node3.depends_on = vec!["node2".to_string()];
        node4.depends_on = vec!["node2".to_string()];
        node5.depends_on = vec!["node4".to_string(), "node3".to_string()];

        // Groups should look like this:
        /*
        +-------+     +-------+
        | node4 | <-- | node5 |
        +-------+     +-------+
        |             |
        |             |
        |             v
        |           +-------+
        |           | node3 |
        |           +-------+
        |             |
        |             |
        |             v
        |           +-------+
        +---------> | node2 |
                    +-------+
                        |
                        |
                        v
                    +-------+
                    | node1 |
                    +-------+
         */

        let mut flow_graph = FlowGraph::default();
        flow_graph.add_node(root.clone());
        flow_graph.add_node(node1.clone());
        flow_graph.add_node(node2.clone());
        flow_graph.add_node(node3.clone());
        flow_graph.add_node(node4.clone());
        flow_graph.add_node(node5.clone());

        let result = flow_graph.calculate_groups();
        assert!(result.is_ok());
        let groups = result.unwrap();
        assert_eq!(groups.len(), 6);

        let expected = vec![
            vec![],
            vec![root],
            vec![node1],
            vec![node2.clone()],
            vec![node2],
            vec![node3, node4],
        ];

        for (idx, node_group) in groups.iter().enumerate() {
            let expected_group = expected.get(idx).unwrap();
            assert_eq!(node_group.depends.len(), expected_group.len());
            let group_node_ids: Vec<NodeId> =
                node_group.depends.iter().map(|node| node.id).collect();
            let expected_node_ids: Vec<NodeId> =
                expected_group.iter().map(|node| node.id).collect();

            for node_id in expected_node_ids.iter() {
                assert!(group_node_ids.contains(node_id));
            }
        }
    }

    #[test]
    fn test_calculates_little_graph_adds_nodes_correctly() {
        let mut flow = build_flow("DemoFlow".to_string());
        let mut node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());

        // Setup dependencies
        node1.depends_on = vec!["node2".to_string()];

        let mut flow_graph = FlowGraph::default();
        flow_graph.add_node(node1.clone());
        flow_graph.add_node(node2.clone());

        let groups = flow_graph.calculate_groups();
        assert!(groups.is_ok());
        let groups = groups.unwrap();
        let graph = flow_graph.into_graph().unwrap();
        println!(
            "{:?}",
            Dot::with_config(&graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
        );
    }
}

#[cfg(debug_assertions)]
pub fn build_flow(node_name: String) -> crate::Flow {
    use crate::Flow;

    use super::flow::FlowBuilder;

    FlowBuilder::default()
        .name(node_name)
        .version("0.1".to_string())
        .build()
        .unwrap()
}

#[cfg(debug_assertions)]
pub mod test_helper {
    use crate::{Task, TaskBuilder};

    pub fn build_task(name: String, code: String) -> Task {
        use anything_runtime::{EngineKind, PluginEngine, RawEnvironment, RawNode, RawVariables};
        use indexmap::indexmap;

        let deno_runtime_raw_node = default_deno_run_options(code);

        TaskBuilder::default()
            .name(name)
            .depends_on(vec![])
            .run_options(deno_runtime_raw_node)
            .build()
            .unwrap()
    }

    pub fn build_task_builder(name: String, code: String) -> TaskBuilder {
        use anything_runtime::{EngineKind, PluginEngine, RawEnvironment, RawNode, RawVariables};
        use indexmap::indexmap;

        let deno_runtime_raw_node = default_deno_run_options(code);

        TaskBuilder::default()
            .name(name)
            .depends_on(vec![])
            .run_options(deno_runtime_raw_node)
            .clone()
    }

    pub fn default_deno_run_options(code: String) -> anything_runtime::RawNode {
        anything_runtime::RawNode {
            variables: anything_runtime::RawVariables::default(),
            environment: anything_runtime::RawEnvironment::default(),
            engine: Some(anything_runtime::EngineKind::PluginEngine(
                anything_runtime::PluginEngine {
                    engine: "deno".to_string(),
                    args: None,
                    options: indexmap::indexmap! {"code".to_string() => anything_runtime::EngineOption::String(code)},
                },
            )),
        }
    }

    pub fn default_system_run_options(code: String) -> anything_runtime::RawNode {
        anything_runtime::RawNode {
            variables: anything_runtime::RawVariables::default(),
            environment: anything_runtime::RawEnvironment::default(),
            engine: Some(anything_runtime::EngineKind::PluginEngine(
                anything_runtime::PluginEngine {
                    engine: "system-shell".to_string(),
                    args: Some(vec!["-c".to_string(), code]),
                    options: indexmap::indexmap! {},
                },
            )),
        }
    }
}
