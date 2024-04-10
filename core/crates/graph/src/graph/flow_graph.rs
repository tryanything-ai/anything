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
        // for (node_id, node) in self.nodes.iter() {
        //     for dep in node.depends_on.iter() {
        //         let dep_id = self.node_name_to_node_id.get(dep).unwrap();
        //         node_deps.entry(node_id).or_insert(Vec::new()).push(dep_id);
        //     }
        // }

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
