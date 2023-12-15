use std::collections::{HashSet,HashMap};
use crate::graph::{Graph, Edge, Subgraph};
use crate::graph_operations::{bfs, find_edges};
pub struct NodeConnection {
    pub node: u32,
    pub count: usize,
}

pub enum GraphData {
    Edges(Vec<Edge>),
    TopNodes(Vec<NodeConnection>),
}

pub fn calculate_top_nodes(edges: &[Edge], top_count: usize) -> GraphData {
    let mut node_connections: HashMap<u32, usize> = HashMap::new();

    for edge in edges {
        *node_connections.entry(edge.from).or_insert(0) += 1;
    }

    let mut sorted_nodes: Vec<NodeConnection> = node_connections.into_iter()
        .map(|(node, count)| NodeConnection { node, count })
        .collect();

    sorted_nodes.sort_by(|a, b| b.count.cmp(&a.count));

    GraphData::TopNodes(sorted_nodes.into_iter().take(top_count).collect())
}

pub fn calculate_graph_density(nodes: &HashSet<u32>, edges: &HashSet<Edge>) -> f64 {
    let node_count = nodes.len() as f64;
    let edge_count = edges.len() as f64;
    edge_count / (2.0*node_count).max(1.0)
}

pub fn find_highest_density_subgraph_with_start_node(
    graph: &Graph,
    start_node: u32,
    max_depth: usize,
) -> Subgraph {
    let bfs_nodes = bfs(graph, start_node, max_depth);
    let mut subgraph_edges = find_edges(graph, &bfs_nodes);
    let mut highest_density_subgraph = Subgraph {
        nodes: bfs_nodes.clone(),
        density: calculate_graph_density(&bfs_nodes, &subgraph_edges),
    };

    let mut current_nodes = bfs_nodes;

    while current_nodes.len() > 1 {
        let mut connections = HashMap::new();
        for &node in &current_nodes {
            connections.insert(node, 0);
        }
        for edge in &subgraph_edges {
            *connections.get_mut(&edge.from).unwrap() += 1;
            *connections.get_mut(&edge.to).unwrap() += 1;
        }

        let node_to_remove = current_nodes.iter()
            .filter(|&&node| node != start_node)
            .min_by_key(|&node| connections.get(node).unwrap_or(&0))
            .cloned();

        if let Some(node) = node_to_remove {
            current_nodes.remove(&node);
            subgraph_edges.retain(|edge| edge.from != node && edge.to != node);

            let new_density = calculate_graph_density(&current_nodes, &subgraph_edges);

            if new_density > highest_density_subgraph.density && current_nodes.contains(&start_node) {
                highest_density_subgraph.nodes = current_nodes.clone();
                highest_density_subgraph.density = new_density;
            }
        } else {
            break;
        }
    }

    highest_density_subgraph
}
