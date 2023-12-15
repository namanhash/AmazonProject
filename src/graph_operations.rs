use std::collections::{HashSet, VecDeque};
use crate::graph::{Graph, Edge};

pub fn bfs(graph: &Graph, start_node: u32, max_depth: usize) -> HashSet<u32> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<(u32, usize)> = VecDeque::new();

    queue.push_back((start_node, 0));
    visited.insert(start_node);

    while let Some((current_node, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        for &neighbor in graph.nodes.get(&current_node).unwrap_or(&vec![]) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back((neighbor, depth + 1));
            }
        }
    }
    visited
}

pub fn find_edges(graph: &Graph, nodes: &HashSet<u32>) -> HashSet<Edge> {
    let mut edges: HashSet<Edge> = HashSet::new();

    for &node in nodes {
        if let Some(neighbors) = graph.nodes.get(&node) {
            for &neighbor in neighbors {
                if nodes.contains(&neighbor) {
                    edges.insert(Edge { from: node, to: neighbor });
                }
            }
        }
    }
    edges
}
