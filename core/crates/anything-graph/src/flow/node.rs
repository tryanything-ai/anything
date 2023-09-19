use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

use super::{action::Action, common::PackageData};
pub type NodeGroup = Vec<Node>;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Node {
    pub package_data: PackageData,
    pub name: String,
    pub label: String,
    pub state: NodeState,
    pub node_action: Action,
    pub depends_on: Vec<String>,
    // pub input: indexmap::IndexMap<String, String>,
    // pub output: indexmap::IndexMap<String, String>, // TODO: should we make this serializable instead of a simple string?
    pub variables: HashMap<String, String>,
    // pub environment: HashMap<String, String>,
}

impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: String::default(),
            label: String::default(),
            package_data: PackageData::default(),
            state: NodeState::default(),
            node_action: Action::default(),
            depends_on: Vec::default(),
            variables: HashMap::new(),
            // environment: HashMap::new(),
            // output: indexmap::IndexMap::new(),
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
// pub struct NodeInput(pub Vec)

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct NodeOutput {
    pub name: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum NodeState {
    Pending,
    Running,
    Success,
    Failed,
}

impl Into<String> for NodeState {
    fn into(self) -> String {
        match self {
            NodeState::Pending => "pending".to_string(),
            NodeState::Running => "running".to_string(),
            NodeState::Success => "success".to_string(),
            NodeState::Failed => "failed".to_string(),
        }
    }
}

impl Default for NodeState {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug)]
pub struct NodeList {
    pub nodes: Vec<NodeGroup>,
    edges: HashMap<String, Vec<String>>,
}

/// NodeList is the group of nodes that exist
/// on the same level. It's a helpful class to
/// group a list of same-level nodes
impl NodeList {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            edges: HashMap::new(),
        }
    }

    pub fn new_with_list(nodes: NodeGroup) -> AppResult<Self> {
        let mut new_list = Self::new();
        match new_list.add_list(nodes) {
            Ok(()) => Ok(new_list),
            Err(e) => Err(e),
        }
    }

    pub fn add_list(&mut self, nodes: NodeGroup) -> AppResult<()> {
        let new_edges: Vec<&str> = nodes.iter().map(|t| t.name.as_ref()).collect();
        for edge in new_edges {
            if self.edges.contains_key(edge) {
                return Err(AppError::FlowError(format!(
                    "Node '{}' is already added",
                    edge
                )));
            } else {
                self.edges.insert(edge.to_string(), vec![]);
            }
        }

        self.nodes.push(nodes);
        Ok(())
    }

    pub fn set_child(&mut self, parent: &str, child: &str) -> AppResult<()> {
        if self.get_node_by_name(&child).is_some() {
            if let Some(children) = self.edges.get_mut(parent) {
                children.push(child.to_string());
                Ok(())
            } else {
                Err(AppError::FlowError(format!(
                    "Parent task '{}' does not exist",
                    parent
                )))
            }
        } else {
            Err(AppError::FlowError(format!(
                "Child task '{}' does not exist",
                &child
            )))
        }
    }

    pub fn is_node_name_parent(&mut self, name: &str) -> bool {
        self.get_node_by_name(name).is_some()
    }

    pub fn get_node_by_name(&mut self, name: &str) -> Option<&mut Node> {
        for node_group in self.nodes.iter_mut() {
            for node in node_group.iter_mut() {
                if node.name == name {
                    return Some(node);
                }
            }
        }
        None
    }

    pub fn get_descendants(&self, node_name: &str) -> Vec<String> {
        let mut descendants = self.get_descendants_recusively(node_name);
        descendants.sort();
        descendants.dedup();
        descendants
    }

    fn get_descendants_recusively(&self, node_name: &str) -> Vec<String> {
        let default = &vec![];
        let deps: Vec<String> = self
            .edges
            .get(node_name)
            .unwrap_or(default)
            .iter()
            .map(|x| x.clone())
            .collect();

        let mut seen = vec![];

        for dep in deps {
            seen.push(dep.clone());
            seen.extend(self.get_descendants_recusively(&dep));
        }

        seen
    }
}

impl IntoIterator for NodeList {
    type Item = Node;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut nodes = vec![];
        for node_group in self.nodes {
            for node in node_group {
                nodes.push(node);
            }
        }
        nodes.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use crate::test_helpers::test_helpers::*;

    use super::*;

    #[test]
    fn test_create_node_default() {
        let _ = Node::new();
    }

    #[test]
    fn test_can_create_a_node_list() {
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);
        let node3 = make_node("node3", &vec![]);
        let node4 = make_node("node4", &vec![]);

        let mut node_list = NodeList::new();

        node_list.add_list(vec![node1, node2]).ok().unwrap();
        node_list.add_list(vec![node3, node4]).ok().unwrap();

        assert!(node_list.get_node_by_name("node1").is_some());
        assert!(node_list.get_node_by_name("node2").is_some());
        assert!(node_list.get_node_by_name("node3").is_some());
        assert!(node_list.get_node_by_name("node4").is_some());
    }

    #[test]
    fn test_returns_none_if_node_does_not_exist() {
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);

        let mut node_list = NodeList::new();
        node_list.add_list(vec![node1, node2]).ok().unwrap();

        assert!(node_list.get_node_by_name("node5").is_none());
    }

    #[test]
    fn test_set_child_without_a_parent_err() {
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);

        let mut node_list = NodeList::new_with_list(vec![node1, node2]).unwrap();
        let r = node_list.set_child("parent", "node1");
        assert!(r.is_err());
    }

    #[test]
    fn test_set_child_without_child_err() {
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);

        let mut node_list = NodeList::new_with_list(vec![node1, node2]).unwrap();
        let r = node_list.set_child("parent", "node2");
        assert!(r.is_err());
    }

    #[test]
    fn test_set_valid_child() {
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);

        let mut node_list = NodeList::new_with_list(vec![node1, node2]).unwrap();
        let r = node_list.set_child("node1", "node2");
        assert!(r.is_ok());
        assert!(r.ok().is_some());
    }

    #[test]
    fn get_child_nodes() {
        let mut node_list = NodeList::new();
        let node1 = make_node("node1", &vec![]);
        let node2 = make_node("node2", &vec![]);
        let node3 = make_node("node3", &vec![]);
        let node4 = make_node("node4", &vec![]);

        let node_group = vec![node1, node2, node3, node4];
        node_list.add_list(node_group).ok();
        node_list.set_child("node1", "node2").ok();
        node_list.set_child("node2", "node3").ok();
        node_list.set_child("node3", "node4").ok();

        assert_eq!(
            vec!["node2", "node3", "node4"],
            node_list.get_descendants("node1")
        );
        assert_eq!(vec!["node3", "node4"], node_list.get_descendants("node2"));
        assert_eq!(Vec::<String>::new(), node_list.get_descendants("node4"));
        assert_eq!(Vec::<String>::new(), node_list.get_descendants(""));
    }

    #[test]
    fn get_children_without_duplicates() {
        let mut node_list = NodeList::new();
        let node1 = make_node("parent", &vec![]);
        let node2 = make_node("child", &vec![]);
        let node3 = make_node("grandchild", &vec![]);
        let node4 = make_node("grandchild2", &vec![]);

        let node_group = vec![node1, node2, node3, node4];
        node_list.add_list(node_group).ok();
        node_list.set_child("parent", "child").ok();
        node_list.set_child("child", "grandchild").ok();
        node_list.set_child("child", "grandchild2").ok();
        node_list.set_child("parent", "grandchild2").ok();

        assert_eq!(
            vec!["grandchild", "grandchild2"],
            node_list.get_descendants("child")
        );
        assert_eq!(
            vec!["child", "grandchild", "grandchild2"],
            node_list.get_descendants("parent")
        );
        assert_eq!(Vec::<String>::new(), node_list.get_descendants(""));
    }
}
