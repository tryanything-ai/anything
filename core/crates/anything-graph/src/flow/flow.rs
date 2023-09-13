use std::collections::HashMap;

use daggy::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::{AppError, AppResult};

use super::{
    action::Action,
    node::{Node, NodeBuilder, NodeList},
    sequencer::{find_node_recursive, get_nodes_in_order},
    trigger::Trigger,
};

// TODO: add node transitions
#[derive(Clone, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Flow {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub trigger: Trigger,
    pub variables: HashMap<String, String>,
    dag: Dag<Node, ()>,
    root: NodeIndex,
}

impl Flow {
    pub fn new() -> Self {
        let mut new_dag = Dag::<Node, ()>::new();
        let mut root_node = Node::default();
        root_node.name = "root_node".to_string();

        let parent = new_dag.add_node(root_node);
        Self {
            name: String::default(),
            version: None,
            description: None,
            dag: new_dag,
            root: parent,
            trigger: Trigger::default(),
            variables: HashMap::new(),
            // active_version: FlowVersion::default(),
        }
    }

    pub fn as_dotfile(&self, start_from: Option<String>) -> String {
        let nodes = if let Some(start_node) = start_from {
            self.get_nodes_in_order_from(&start_node)
        } else {
            self.get_nodes_in_order()
        };

        let mut topological_sorted_nodes = vec![];

        for node in nodes.iter() {
            for s in node.iter() {
                for dep in s.iter() {
                    topological_sorted_nodes.push(dep);
                }
            }
        }

        let title = format!("digraph {} {{", self.name);
        let node_names = topological_sorted_nodes
            .iter()
            .map(|s| format!("  \"{}\"\n", s.name))
            .collect::<String>();

        let node_connections = topological_sorted_nodes
            .iter()
            .map(|node| {
                node.depends_on
                    .iter()
                    .map(|dep| format!("  \"{}\" -> \"{}\"\n", node.name, dep))
                    .collect::<String>()
            })
            .collect::<String>();

        format!("{}\n{}{}{}", title, node_names, node_connections, "}")
    }

    pub fn add_node_obj(&mut self, node: &Node) -> AppResult<bool> {
        let depends_on = node
            .depends_on
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>();

        self.add_node(&node, &node.name, &node.node_action, &depends_on)
    }

    pub fn add_node(
        &mut self,
        node: &Node,
        name: &str,
        action: &Action,
        depends_on: &Vec<&str>,
    ) -> AppResult<bool> {
        if let Some((_, node)) = self.find_node_by_name(name) {
            return Err(AppError::FlowError(
                format!("Node {} already exists", node.name).to_string(),
            ));
        }

        if depends_on.len() > 0 {
            // Add dependencies
            if depends_on.iter().any(|s| s == &name) {
                return Err(AppError::FlowError(format!(
                    "A node cannot depend on itself"
                )));
            }

            let mut parents: Vec<NodeIndex> = vec![];
            let mut deps: Vec<String> = vec![];

            for dependency in depends_on {
                if let Some((idx, _)) = self.find_node_by_name(dependency) {
                    parents.push(idx);
                    deps.push(dependency.to_string());
                } else {
                    return Err(AppError::FlowError(format!(
                        "A node must have dependencies defined already. Could not find node: {}",
                        dependency
                    )));
                }
            }

            let node = NodeBuilder::default()
                .name(name.to_string())
                .variables(node.variables.clone())
                .node_action(action.clone())
                .depends_on(
                    depends_on
                        .iter()
                        .map(|s| String::from(*s))
                        .collect::<Vec<String>>(),
                )
                .build()?;

            let node = self.dag.add_node(node);

            for parent in parents {
                if let Err(_) = self.dag.add_edge(parent, node, ()) {
                    return Err(AppError::FlowError(format!(
                        "Unable to add edge between {} and {}",
                        self.dag[parent].name, self.dag[node].name
                    )));
                }
            }
        } else {
            let node = NodeBuilder::default()
                .name(name.to_string())
                .node_action(action.clone())
                .variables(node.variables.clone())
                .build()?;
            self.dag.add_child(self.root, (), node);
        }

        Ok(true)
    }

    /// get_node_execution_list returns the execution list with
    /// children for ordered execution
    ///
    /// `start_from` is an optional node name to get the execution list
    /// from vs. the root
    pub fn get_node_execution_list<'a>(
        &'a self,
        start_from: Option<String>,
    ) -> AppResult<NodeList> {
        let mut node_list = NodeList::new();

        let nodes = if let Some(start_node) = start_from {
            info!("Reduced run... starting from {}", &start_node);
            self.get_nodes_in_order_from(&start_node)
        } else {
            self.get_nodes_in_order()
        };

        if let Err(e) = nodes {
            return Err(e);
        }
        let nodes = nodes.unwrap();

        for lvl in nodes.iter() {
            let node_group = lvl
                .iter()
                .map(|node| Node {
                    name: node.name.clone(),
                    label: node.label.clone(),
                    state: node.state.clone(),
                    package_data: node.package_data.clone(),
                    // trigger: node.trigger.clone(),
                    node_action: node.node_action.clone(),
                    depends_on: node.depends_on.clone(),
                    variables: node.variables.clone(),
                })
                .collect::<Vec<Node>>();

            match node_list.add_list(node_group) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for lvl in nodes.iter() {
            for node in lvl.iter() {
                for dep in node.depends_on.iter() {
                    if node_list.is_node_name_parent(&dep)
                        && node_list.is_node_name_parent(&node.name)
                    {
                        match node_list.set_child(&dep, &node.name) {
                            Err(_e) => {
                                return Err(AppError::FlowNodeError(format!(
                                    "Could not add '{}' to child '{}'",
                                    dep, node.name
                                )))
                            }
                            Ok(_) => (),
                        }
                    }
                }
            }
        }

        Ok(node_list)
    }

    /// get_nodes_in_order
    /// returns the next nodes starting from the `start_index`
    ///
    /// Example:
    ///     flow.get_nodes_in_order_from(parent);
    pub fn get_nodes_in_order_from<'a>(
        &'a self,
        start_from: &str,
    ) -> AppResult<Vec<Vec<&'a Node>>> {
        if let Some((idx, node)) = self.find_node_by_name(start_from) {
            match self.get_nodes_in_order_from_node_index(idx) {
                Err(e) => Err(e),
                Ok(mut nodes) => {
                    nodes.insert(0, vec![node]);
                    Ok(nodes)
                }
            }
        } else {
            Err(AppError::FlowError(
                format!("Cannot start from {} - nodes do not exist", start_from).to_string(),
            ))
        }
    }

    pub fn get_nodes_in_order<'a>(&'a self) -> AppResult<Vec<Vec<&'a Node>>> {
        self.get_nodes_in_order_from_node_index(self.root)
    }

    fn get_nodes_in_order_from_node_index<'a>(
        &'a self,
        start_node_index: NodeIndex,
    ) -> AppResult<Vec<Vec<&'a Node>>> {
        let mut tree: Vec<Vec<&Node>> = vec![];
        get_nodes_in_order(
            &self.dag,
            &self
                .dag
                .children(start_node_index)
                .iter(&self.dag)
                .map(|(_, node_idx)| node_idx)
                .collect(),
            &mut tree,
        );
        Ok(tree)
    }

    fn find_node_by_name(&self, name: &str) -> Option<(NodeIndex, &Node)> {
        find_node_recursive(&self.dag, name, self.root)
    }
}

impl Default for Flow {
    fn default() -> Self {
        Self::new()
    }
}

// #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
// pub struct FlowVersion {
//     id: String,
//     display_name: String,
//     valid: bool,
//     state: FlowVersionState,
//     created_at: DateTime<Utc>,
//     updated_at: DateTime<Utc>,
// }

// impl Default for FlowVersion {
//     fn default() -> Self {
//         Self {
//             id: String::default(),
//             display_name: String::default(),
//             valid: true,
//             state: FlowVersionState::Draft,
//             created_at: DateTime::default(),
//             updated_at: DateTime::default(),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub enum FlowVersionState {
//     Locked,
//     Draft,
// }

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;

    use crate::{
        flow::action::{ActionType, ShellAction},
        test_helpers::test_helpers::*,
    };

    use super::*;

    #[test]
    fn test_flow_can_add_a_single_simple_node_for_two_node_sequence() {
        let mut flow = Flow::new();
        let action = make_action(
            "find_one_action",
            ActionType::Shell(ShellAction {
                command: "echo".to_string(),
                executor: Some("sh -c".to_string()),
                args: None,
                cwd: None,
            }),
        );
        let mut node = make_node("first_node", &vec![]);
        node.node_action = action;
        let res = flow.add_node_obj(&node);
        assert!(res.is_ok());

        let nodes = flow.get_nodes_in_order().ok();
        assert!(nodes.is_some());
        let nodes = nodes.unwrap();
        assert_eq!(1, nodes.len());
    }

    #[test]
    fn test_flow_can_add_a_series_of_dependent_command_nodes_for_two_node_sequence() {
        let (flow, _orig_nodes) = make_two_node_sequence_flow();

        let nodes = flow.get_nodes_in_order().ok();
        assert!(nodes.is_some());
        let nodes = nodes.unwrap();
        assert_eq!(2, nodes.len());

        let first_nodes = nodes.get(0).unwrap();
        let first_node = first_nodes.get(0).unwrap();
        assert_eq!(first_node.name, "get_weather_node");

        let second_node = nodes.get(1).unwrap().get(0).unwrap();
        assert_eq!(second_node.name, "print_forecast");
    }

    #[test]
    fn test_get_execution_list_in_order_for_two_node_sequence() {
        let (flow, _orig_nodes) = make_two_node_sequence_flow();
        let execution_list = flow.get_node_execution_list(None);
        assert!(execution_list.is_ok());
        let sl = execution_list.unwrap();
        assert_eq!(sl.nodes.len(), 2);

        // let mut sl2 = sl.clone();
        let level_one = sl.nodes.get(0).unwrap();
        let level_two = sl.nodes.get(1).unwrap();
        assert_eq!(1, level_one.len());
        assert_eq!(1, level_two.len());
    }

    #[test]
    fn test_flow_as_graphviz_dot_file() {
        let (mut flow, _orig_nodes) = make_two_node_sequence_flow();
        let action = make_action(
            "print-to-screen",
            ActionType::Shell(ShellAction {
                command: "echo".to_string(),
                executor: Some("sh -c".to_string()),
                args: None,
                cwd: None,
            }),
        );
        let mut node = make_node("output-forecast", &vec!["print_forecast"]);
        node.node_action = action;
        add_node_obj(&mut flow, &node).ok();

        let graphviz = flow.as_dotfile(None);
        assert!(graphviz.len() > 0);

        let mut file = File::create("/tmp/test.dot").expect("unable to open file");
        file.write_all(graphviz.as_bytes()).expect("error writing");
    }
}
