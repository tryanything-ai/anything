#![allow(unused)]

#[cfg(test)]
pub mod test_helpers {
    use crate::{
        error::*,
        flow::{
            action::{Action, ActionBuilder, ActionType, ShellAction},
            common::{PackageData, PackageDataBuilder},
            flow::Flow,
            node::{Node, NodeBuilder},
        },
    };

    pub fn make_node(name: &str, dependencies: &Vec<&str>) -> Node {
        let dependencies = dependencies
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        let package_data = PackageDataBuilder::default()
            .build()
            .expect("unable to create package data");
        NodeBuilder::default()
            .package_data(package_data)
            .name(name.to_string())
            .depends_on(dependencies)
            .build()
            .expect("unable to create node :/")
    }

    pub fn make_action(name: &str, action_type: ActionType) -> Action {
        ActionBuilder::default()
            .id(name)
            .action_type(action_type)
            .build()
            .expect("unable to create action")
    }

    pub fn get_tree_names<'a>(actual: Vec<Vec<&'a Node>>) -> Vec<Vec<&'a str>> {
        actual
            .iter()
            .into_iter()
            .map(|nodes| {
                nodes
                    .into_iter()
                    .map(|s| s.name.as_str())
                    .collect::<Vec<&'a str>>()
            })
            .collect::<Vec<Vec<&'a str>>>()
    }

    /// Flow Helpers
    /// Helpers
    /// Make a simple 2-node sequence action flow
    pub fn make_two_node_sequence_flow() -> (Flow, Vec<Node>) {
        let mut nodes = Vec::new();

        let mut flow = Flow::new();
        let action = make_action(
            "get_weather_forecast",
            ActionType::Shell(ShellAction {
                command: "curl".to_string(),
                executor: Some("sh -c".to_string()),
                args: Vec::default(),
            }),
        );
        let mut node = make_node("get_weather_node", &vec![]);
        node.node_action = action;
        add_node_obj(&mut flow, &node).ok();

        nodes.push(node);

        let action = make_action(
            "print_forecast",
            ActionType::Shell(ShellAction {
                command: "echo".to_string(),
                executor: Some("sh -c".to_string()),
                args: Vec::default(),
            }),
        );

        let mut node = make_node("print_forecast", &vec!["get_weather_node"]);
        node.node_action = action;
        add_node_obj(&mut flow, &node).ok();
        nodes.push(node);

        (flow, nodes)
    }

    pub fn add_node_obj(flow: &mut Flow, node: &Node) -> AppResult<bool> {
        let depends_on = node
            .depends_on
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>();

        flow.add_node(&node.name, &node.node_action, &depends_on)
    }
}
