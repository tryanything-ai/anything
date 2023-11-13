use anything_runtime::{RawEnvironment, RawVariables};
use derive_builder::Builder;
use petgraph::prelude::DiGraph;
use serde::{Deserialize, Serialize};

use crate::{core::keyable::Keyable, error::GraphResult, NodeType};

use super::{flow_graph::FlowGraph, flowfile::Flowfile, node::Task, trigger::Trigger};

#[allow(unused)]
#[derive(Debug, Clone, Deserialize, Serialize, Builder)]
#[builder(default)]
pub struct Flow {
    #[serde(default = "crate::core::id_alloc::alloc_uuid_string")]
    pub flow_id: String,

    pub name: String,
    pub version: String,
    pub description: String,

    #[serde(default)]
    pub variables: RawVariables,

    #[serde(default)]
    pub environment: RawEnvironment<String>,

    #[serde(default)]
    pub trigger: Trigger,

    #[serde(skip)]
    pub graph: FlowGraph,
    pub nodes: Vec<Task>,
}

impl Default for Flow {
    fn default() -> Self {
        Self {
            flow_id: crate::core::id_alloc::alloc_uuid_string(),
            name: "default".to_string(),
            version: "0.0.1".to_string(),
            description: "".to_string(),
            variables: RawVariables::default(),
            environment: RawEnvironment::default(),
            trigger: Trigger::default(),
            graph: FlowGraph::default(),
            nodes: Vec::new(),
        }
    }
}

impl Keyable for Flow {
    fn key(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

#[allow(unused)]
impl Flow {
    pub fn add_node(&mut self, node: Task) -> GraphResult<()> {
        self.graph.add_node(node);
        Ok(())
    }

    // TODO: write tests
    pub fn add_global_variable(&mut self, input: &str, value: String) -> GraphResult<()> {
        self.variables.add(input, value.into());
        Ok(())
    }

    // TODO: write tests
    pub fn add_global_environment(&mut self, input: &str, val: String) -> GraphResult<()> {
        self.environment.add(input, val);
        Ok(())
    }

    pub fn get_nodes(&self) -> Vec<Task> {
        self.graph
            .nodes
            .iter()
            .map(|(_id, n)| (*n).clone())
            .collect::<Vec<Task>>()
    }

    pub fn get_flowgraph(&self) -> FlowGraph {
        let mut graph = FlowGraph::new();

        for node in self.get_nodes() {
            graph.add_node(node);
        }

        graph
    }

    pub fn into_graph(&self) -> GraphResult<DiGraph<NodeType, usize>> {
        self.get_flowgraph().into_graph()
    }
}

impl From<Flowfile> for Flow {
    fn from(value: Flowfile) -> Self {
        let mut flow = Flow::default();
        flow.flow_id = value.flow_id;
        flow.name = value.name;
        flow.version = value.version.unwrap_or_else(|| "0.0.0".to_string());
        flow.description = value
            .description
            .unwrap_or_else(|| "no description".to_string());
        flow.variables = value.variables;
        flow.environment = value.environment;
        flow.trigger = value.trigger;

        for node in value.nodes {
            flow.add_node(node).unwrap();
        }

        flow
    }
}

impl Into<Vec<u8>> for Flow {
    fn into(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}

impl Into<Flowfile> for Flow {
    fn into(self) -> Flowfile {
        let mut flowfile = Flowfile::default();
        flowfile.name = self.name;
        flowfile.version = Some(self.version);
        flowfile.description = Some(self.description);
        flowfile.nodes = self.nodes;
        flowfile.variables = self.variables;
        flowfile.environment = self.environment;

        flowfile
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_flow, test_helper::build_task};

    use super::*;

    #[test]
    fn test_can_create_a_flow_graph() {
        let mut flow = build_flow("DemoFlow".to_string());
        let node1 = build_task("node1".to_string(), "one.js".to_string());
        let mut node2 = build_task("node2".to_string(), "two.js".to_string());

        node2.depends_on = vec!["node1".to_string()];

        let _ = flow.add_node(node1.clone());
        let _ = flow.add_node(node2.clone());

        let graph = flow.get_flowgraph();
        let mut expected = FlowGraph::default();
        expected.add_node(node1.clone());
        expected.add_node(node2.clone());

        assert_eq!(graph.get_node(node1.id).clone(), Some(&node1));
        assert_eq!(graph.get_node(node2.id).clone(), Some(&node2));
    }

    #[test]
    fn test_variables_are_passed_to_flow_from_file() {
        let mut flow = build_flow("DemoFlow".to_string());
        let node1 = build_task("node1".to_string(), "one.js".to_string());

        let _ = flow.add_node(node1.clone());

        let mut variables = RawVariables::default();
        variables.add("name", "bob".to_string().into());
        variables.add("age", 42.to_string().into());

        let mut flowfile = Flowfile::default();
        flowfile.name = "DemoFlow".to_string();
        flowfile.variables = variables.clone();

        let flow = Flow::from(flowfile);
        assert_eq!(flow.variables, variables);
    }

    #[test]
    fn test_flow_gets_a_unique_uuid_by_default() {
        let flow = Flow::default();
        assert_ne!(flow.flow_id, "".to_string());
    }

    #[test]
    fn test_can_serialize_flow_into_a_flowfile() {
        let mut flow = build_flow("DemoFlow".to_string());
        let node1 = build_task("node1".to_string(), "one.js".to_string());

        let _ = flow.add_node(node1.clone());

        let flow_file: Flowfile = flow.into();
        let flow_str: String = flow_file.into();

        assert_eq!(
            flow_str,
            r#"flow_id = ""
name = "DemoFlow"
version = "0.1"
description = ""
nodes = []

[variables]

[environment]

[trigger]
type = "empty"
"#
        );
    }
}
