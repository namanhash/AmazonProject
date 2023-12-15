use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone, Hash)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.nodes.entry(edge.from).or_default().push(edge.to);
        self.nodes.entry(edge.to).or_default();
    }

    pub fn find_node(&self, node_id: u32) -> GraphOperationResult {
        if self.nodes.contains_key(&node_id) {
            GraphOperationResult::Success
        } else {
            GraphOperationResult::NodeNotFound(node_id)
        }
    }
}

pub enum GraphOperationResult {
    Success,
    NodeNotFound(u32),
}

#[derive(Debug)]
pub struct Subgraph {
    pub nodes: HashSet<u32>,
    pub density: f64,
}
