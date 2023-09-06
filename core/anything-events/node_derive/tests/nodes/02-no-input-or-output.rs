mod common;
use common::*;
use node_derive::Node;

#[derive(Node)]
pub struct SimpleNode {}

impl SimpleNode {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, _x: u32) -> Result<Vec<u32>, NodeError> {
        Ok(vec![1])
    }
}

fn main() {}
