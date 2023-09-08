use daggy::*;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::error::{AppError, AppResult};

use super::{
    action::Action,
    node::{Node, NodeBuilder, StepList},
    sequencer::{find_node_recursive, get_nodes_in_order},
    trigger::Trigger,
};

// TODO: add node transitions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flow {
    pub id: String,
    pub name: String,
    pub trigger: Trigger,
    dag: Dag<Node, ()>,
    root: NodeIndex,
}

impl Flow {
    pub fn new() -> Self {
        let mut new_dag = Dag::<Node, ()>::new();
        let mut root_step = Node::default();
        root_step.name = "root_step".to_string();

        let parent = new_dag.add_node(root_step);
        Self {
            id: String::default(),
            name: String::default(),
            dag: new_dag,
            root: parent,
            trigger: Trigger::default(),
            // active_version: FlowVersion::default(),
        }
    }

    pub fn as_dotfile(&self, start_from: Option<String>) -> String {
        let steps = if let Some(start_step) = start_from {
            self.get_nodes_in_order_from(&start_step)
        } else {
            self.get_nodes_in_order()
        };

        let mut topological_sorted_steps = vec![];

        for step in steps.iter() {
            for s in step.iter() {
                for dep in s.iter() {
                    println!("{} -> {}", dep.name, dep.name);
                    topological_sorted_steps.push(dep);
                }
            }
        }

        let title = format!("digraph {} {{", self.name);
        let step_names = topological_sorted_steps
            .iter()
            .map(|s| format!("  \"{}\"\n", s.name))
            .collect::<String>();

        let step_connections = topological_sorted_steps
            .iter()
            .map(|step| {
                step.depends_on
                    .iter()
                    .map(|dep| format!("  \"{}\" -> \"{}\"\n", step.name, dep))
                    .collect::<String>()
            })
            .collect::<String>();

        format!("{}\n{}{}{}", title, step_names, step_connections, "}")
    }

    #[cfg(test)]
    pub fn add_step_obj(&mut self, step: &Node) -> AppResult<bool> {
        let depends_on = step
            .depends_on
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>();

        self.add_step(&step.name, &step.step_action, &depends_on)
    }

    pub fn add_step(
        &mut self,
        name: &str,
        action: &Action,
        depends_on: &Vec<&str>,
    ) -> AppResult<bool> {
        if let Some((_, step)) = self.find_node_by_name(name) {
            return Err(AppError::FlowError(
                format!("Step {} already exists", step.name).to_string(),
            ));
        }

        if depends_on.len() > 0 {
            // Add dependencies
            if depends_on.iter().any(|s| s == &name) {
                return Err(AppError::FlowError(format!(
                    "A step cannot depend on itself"
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
                        "A step must have dependencies defined already. Could not find step: {}",
                        dependency
                    )));
                }
            }

            let step = NodeBuilder::default()
                .name(name.to_string())
                .step_action(action.clone())
                .depends_on(
                    depends_on
                        .iter()
                        .map(|s| String::from(*s))
                        .collect::<Vec<String>>(),
                )
                .build()?;

            let node = self.dag.add_node(step);

            for parent in parents {
                if let Err(_) = self.dag.add_edge(parent, node, ()) {
                    return Err(AppError::FlowError(format!(
                        "Unable to add edge between {} and {}",
                        self.dag[parent].name, self.dag[node].name
                    )));
                }
            }
        } else {
            let step = NodeBuilder::default()
                .name(name.to_string())
                .step_action(action.clone())
                .build()?;
            self.dag.add_child(self.root, (), step);
        }

        Ok(true)
    }

    /// get_step_execution_list returns the execution list with
    /// children for ordered execution
    ///
    /// `start_from` is an optional step name to get the execution list
    /// from vs. the root
    pub fn get_step_execution_list<'a>(
        &'a self,
        start_from: Option<String>,
    ) -> AppResult<StepList> {
        let mut step_list = StepList::new();

        let steps = if let Some(start_step) = start_from {
            info!("Reduced run... starting from {}", &start_step);
            self.get_nodes_in_order_from(&start_step)
        } else {
            self.get_nodes_in_order()
        };

        if let Err(e) = steps {
            return Err(e);
        }
        let steps = steps.unwrap();

        for lvl in steps.iter() {
            let step_group = lvl
                .iter()
                .map(|step| Node {
                    name: step.name.clone(),
                    state: step.state.clone(),
                    package_data: step.package_data.clone(),
                    trigger: step.trigger.clone(),
                    step_action: step.step_action.clone(),
                    depends_on: step.depends_on.clone(),
                    run_started: step.run_started.clone(),
                    input: step.input.clone(),
                })
                .collect::<Vec<Node>>();
            match step_list.add_list(step_group) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        for lvl in steps.iter() {
            for step in lvl.iter() {
                for dep in step.depends_on.iter() {
                    if step_list.is_step_name_parent(&dep)
                        && step_list.is_step_name_parent(&step.name)
                    {
                        match step_list.set_child(&dep, &step.name) {
                            Err(_e) => {
                                return Err(AppError::FlowStepError(format!(
                                    "Could not add '{}' to child '{}'",
                                    dep, step.name
                                )))
                            }
                            Ok(_) => (),
                        }
                    }
                }
            }
        }

        Ok(step_list)
    }

    /// get_nodes_in_order
    /// returns the next steps starting from the `start_index`
    ///
    /// Example:
    ///     flow.get_nodes_in_order_from(parent);
    pub fn get_nodes_in_order_from<'a>(
        &'a self,
        start_from: &str,
    ) -> AppResult<Vec<Vec<&'a Node>>> {
        if let Some((idx, step)) = self.find_node_by_name(start_from) {
            match self.get_nodes_in_order_from_node_index(idx) {
                Err(e) => Err(e),
                Ok(mut steps) => {
                    steps.insert(0, vec![step]);
                    Ok(steps)
                }
            }
        } else {
            Err(AppError::FlowError(
                format!("Cannot start from {} - steps do not exist", start_from).to_string(),
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
    fn test_flow_can_add_a_single_simple_step_for_two_step_sequence() {
        let mut flow = Flow::new();
        let action = make_action(
            "find_one_action",
            ActionType::Shell(ShellAction {
                command: "echo".to_string(),
                executor: Some("sh -c".to_string()),
                args: Vec::default(),
            }),
        );
        let mut step = make_node("first_step", &vec![]);
        step.step_action = action;
        let res = flow.add_step_obj(&step);
        assert!(res.is_ok());

        let steps = flow.get_nodes_in_order().ok();
        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(1, steps.len());
    }

    #[test]
    fn test_flow_can_add_a_series_of_dependent_command_steps_for_two_step_sequence() {
        let (flow, _orig_steps) = make_two_step_sequence_flow();

        let steps = flow.get_nodes_in_order().ok();
        assert!(steps.is_some());
        let steps = steps.unwrap();
        assert_eq!(2, steps.len());

        let first_steps = steps.get(0).unwrap();
        let first_step = first_steps.get(0).unwrap();
        assert_eq!(first_step.name, "get_weather_step");

        let second_step = steps.get(1).unwrap().get(0).unwrap();
        assert_eq!(second_step.name, "print_forecast");
    }

    #[test]
    fn test_get_execution_list_in_order_for_two_step_sequence() {
        let (flow, _orig_steps) = make_two_step_sequence_flow();
        let execution_list = flow.get_step_execution_list(None);
        assert!(execution_list.is_ok());
        let sl = execution_list.unwrap();
        assert_eq!(sl.steps.len(), 2);

        // let mut sl2 = sl.clone();
        let level_one = sl.steps.get(0).unwrap();
        let level_two = sl.steps.get(1).unwrap();
        assert_eq!(1, level_one.len());
        assert_eq!(1, level_two.len());
    }

    #[test]
    fn test_flow_as_graphviz_dot_file() {
        let (mut flow, _orig_steps) = make_two_step_sequence_flow();
        let action = make_action(
            "print-to-screen",
            ActionType::Shell(ShellAction {
                command: "echo".to_string(),
                executor: Some("sh -c".to_string()),
                args: Vec::default(),
            }),
        );
        let mut step = make_node("output-forecast", &vec!["print_forecast"]);
        step.step_action = action;
        add_step_obj(&mut flow, &step).ok();

        let graphviz = flow.as_dotfile(None);
        assert!(graphviz.len() > 0);

        let mut file = File::create("/tmp/test.dot").expect("unable to open file");
        file.write_all(graphviz.as_bytes()).expect("error writing");
    }
}
