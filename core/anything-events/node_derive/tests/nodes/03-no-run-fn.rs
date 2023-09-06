mod common;
use common::*;
use node_derive::Node;

#[derive(Node)]
pub struct SimpleNode {
    pub input: NodeReceiver<u32>,
    pub output: NodeSender<Vec<u32>>,
}

fn main() {}
