use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

use super::{action::Action, common::PackageData, trigger::Trigger};
pub type StepGroup = Vec<Node>;

#[derive(Clone, Debug, Deserialize, Serialize, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct Node {
    pub name: String,
    pub trigger: Option<Trigger>,
    pub state: StepState,
    pub step_action: Action,
    pub depends_on: Vec<String>,
    pub run_started: Option<DateTime<Utc>>,
    pub input: indexmap::IndexMap<String, String>,
    pub package_data: PackageData,
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
            package_data: PackageData::default(),
            trigger: None,
            state: StepState::default(),
            step_action: Action::default(),
            depends_on: Vec::default(),
            run_started: None,
            input: indexmap::IndexMap::new(),
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
// pub struct StepInput(pub Vec)

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct StepOutput {
    pub name: String,
    pub value: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum StepState {
    Pending,
    Running,
    Success,
    SuccessNoop,
    Failed(String),
    Skipped(String),
}

impl Default for StepState {
    fn default() -> Self {
        Self::Pending
    }
}

pub enum StepType {
    Shell(ShellStep),
    Missing,
}

// TODO:
pub struct ShellStep {}

#[derive(Clone, Debug)]
pub struct StepList {
    pub steps: Vec<StepGroup>,
    edges: HashMap<String, Vec<String>>,
}

/// StepList is the group of steps that exist
/// on the same level. It's a helpful class to
/// group a list of same-level steps
impl StepList {
    pub fn new() -> Self {
        Self {
            steps: vec![],
            edges: HashMap::new(),
        }
    }

    pub fn new_with_list(steps: StepGroup) -> AppResult<Self> {
        let mut new_list = Self::new();
        match new_list.add_list(steps) {
            Ok(()) => Ok(new_list),
            Err(e) => Err(e),
        }
    }

    pub fn add_list(&mut self, steps: StepGroup) -> AppResult<()> {
        let new_edges: Vec<&str> = steps.iter().map(|t| t.name.as_ref()).collect();
        for edge in new_edges {
            if self.edges.contains_key(edge) {
                return Err(AppError::FlowError(format!(
                    "Step '{}' is already added",
                    edge
                )));
            } else {
                self.edges.insert(edge.to_string(), vec![]);
            }
        }

        self.steps.push(steps);
        Ok(())
    }

    pub fn set_child(&mut self, parent: &str, child: &str) -> AppResult<()> {
        if self.get_step_by_name(&child).is_some() {
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

    pub fn is_step_name_parent(&mut self, name: &str) -> bool {
        self.get_step_by_name(name).is_some()
    }

    pub fn get_step_by_name(&mut self, name: &str) -> Option<&mut Node> {
        for step_group in self.steps.iter_mut() {
            for step in step_group.iter_mut() {
                if step.name == name {
                    return Some(step);
                }
            }
        }
        None
    }

    pub fn get_descendants(&self, step_name: &str) -> Vec<String> {
        let mut descendants = self.get_descendants_recusively(step_name);
        descendants.sort();
        descendants.dedup();
        descendants
    }

    fn get_descendants_recusively(&self, step_name: &str) -> Vec<String> {
        let default = &vec![];
        let deps: Vec<String> = self
            .edges
            .get(step_name)
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

#[cfg(test)]
mod tests {

    use crate::test_helpers::test_helpers::*;

    use super::*;

    #[test]
    fn test_create_step_default() {
        let _ = Node::new();
    }

    #[test]
    fn test_can_create_a_step_list() {
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);
        let step3 = make_node("step3", &vec![]);
        let step4 = make_node("step4", &vec![]);

        let mut step_list = StepList::new();

        step_list.add_list(vec![step1, step2]).ok().unwrap();
        step_list.add_list(vec![step3, step4]).ok().unwrap();

        assert!(step_list.get_step_by_name("step1").is_some());
        assert!(step_list.get_step_by_name("step2").is_some());
        assert!(step_list.get_step_by_name("step3").is_some());
        assert!(step_list.get_step_by_name("step4").is_some());
    }

    #[test]
    fn test_returns_none_if_step_does_not_exist() {
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);

        let mut step_list = StepList::new();
        step_list.add_list(vec![step1, step2]).ok().unwrap();

        assert!(step_list.get_step_by_name("step5").is_none());
    }

    #[test]
    fn test_set_child_without_a_parent_err() {
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);

        let mut step_list = StepList::new_with_list(vec![step1, step2]).unwrap();
        let r = step_list.set_child("parent", "step1");
        assert!(r.is_err());
    }

    #[test]
    fn test_set_child_without_child_err() {
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);

        let mut step_list = StepList::new_with_list(vec![step1, step2]).unwrap();
        let r = step_list.set_child("parent", "step2");
        assert!(r.is_err());
    }

    #[test]
    fn test_set_valid_child() {
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);

        let mut step_list = StepList::new_with_list(vec![step1, step2]).unwrap();
        let r = step_list.set_child("step1", "step2");
        assert!(r.is_ok());
        assert!(r.ok().is_some());
    }

    #[test]
    fn get_child_steps() {
        let mut step_list = StepList::new();
        let step1 = make_node("step1", &vec![]);
        let step2 = make_node("step2", &vec![]);
        let step3 = make_node("step3", &vec![]);
        let step4 = make_node("step4", &vec![]);

        let step_group = vec![step1, step2, step3, step4];
        step_list.add_list(step_group).ok();
        step_list.set_child("step1", "step2").ok();
        step_list.set_child("step2", "step3").ok();
        step_list.set_child("step3", "step4").ok();

        assert_eq!(
            vec!["step2", "step3", "step4"],
            step_list.get_descendants("step1")
        );
        assert_eq!(vec!["step3", "step4"], step_list.get_descendants("step2"));
        assert_eq!(Vec::<String>::new(), step_list.get_descendants("step4"));
        assert_eq!(Vec::<String>::new(), step_list.get_descendants(""));
    }

    #[test]
    fn get_children_without_duplicates() {
        let mut step_list = StepList::new();
        let step1 = make_node("parent", &vec![]);
        let step2 = make_node("child", &vec![]);
        let step3 = make_node("grandchild", &vec![]);
        let step4 = make_node("grandchild2", &vec![]);

        let step_group = vec![step1, step2, step3, step4];
        step_list.add_list(step_group).ok();
        step_list.set_child("parent", "child").ok();
        step_list.set_child("child", "grandchild").ok();
        step_list.set_child("child", "grandchild2").ok();
        step_list.set_child("parent", "grandchild2").ok();

        assert_eq!(
            vec!["grandchild", "grandchild2"],
            step_list.get_descendants("child")
        );
        assert_eq!(
            vec!["child", "grandchild", "grandchild2"],
            step_list.get_descendants("parent")
        );
        assert_eq!(Vec::<String>::new(), step_list.get_descendants(""));
    }
}
