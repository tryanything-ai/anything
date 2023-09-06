mod common;
use common::*;
use node_derive::Node;

#[derive(Node)]
#[aggregate]
pub struct SimpleNode {
    pub input: NodeReceiver<u32>,
    pub output: NodeSender<Vec<u32>>,
}

impl SimpleNode {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            output: Vec::default(),
        }
    }

    pub fn run(&mut self, _x: u32) -> Result<Vec<u32>, NodeError> {
        Ok(vec![1])
    }
}

fn main() {}
