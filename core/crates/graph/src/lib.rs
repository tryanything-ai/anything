pub(crate) mod error;
pub(crate) mod graph;
// pub(crate) mod task;
pub(crate) mod core;
pub(crate) mod raw;

pub use graph::flow::Flow;
pub use graph::flow_graph::*;
pub use graph::flowfile::Flowfile;
pub use graph::node::{NodeId, TaskBuilder};

#[allow(unused)]
#[cfg(debug_assertions)]
pub use graph::{
    flow::FlowBuilder,
    flow_graph::*,
    node::{NodeType, Task},
};
