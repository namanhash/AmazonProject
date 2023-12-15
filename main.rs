mod graph;
mod graph_operations;
mod graph_analysis;
mod file_operations;
mod tests;

use std::io;
use crate::file_operations::read_graph_from_file;
use crate::graph::{Graph, GraphOperationResult};
use crate::graph_analysis::{calculate_top_nodes, find_highest_density_subgraph_with_start_node, calculate_graph_density, GraphData};
use crate::graph_operations::{bfs, find_edges};
use rand::Rng;

fn main() -> io::Result<()> {
    let filename = "converted_data.txt";
    let graph_data = read_graph_from_file(filename)?;

    let mut graph = Graph::new();

    match &graph_data {
        GraphData::Edges(edges) => {
            for edge in edges {
                graph.add_edge(edge.clone());
            }
        },
        _ => {
            eprintln!("Invalid graph data");
            return Ok(());
        }
    }

    let start_node = 11;
    let max_depth = 6;
    let result = graph.find_node(start_node);

    match result {
        GraphOperationResult::Success => {
            let reachable_nodes = bfs(&graph, start_node, max_depth);
            let edges_set = find_edges(&graph, &reachable_nodes);
            let mut edges = edges_set.clone().into_iter().collect::<Vec<_>>();
            edges.sort();
            println!("The top 5 nodes in this subgraph are:");
            match calculate_top_nodes(&edges, 5) {
                GraphData::TopNodes(top_nodes) => {
                    for node_connection in top_nodes {
                        println!("Node {}: {} connections", node_connection.node, node_connection.count);
                    }
                },
                _ => {
                    eprintln!("Invalid top nodes data \n");
                }
            }
            println!();
            let density = calculate_graph_density(&reachable_nodes, &edges_set); 
            println!("Density of the BFS Subgraph: {}", density);
            println!("Total Nodes in the BFS Subgraph: {}", reachable_nodes.len());
            println!();

            let subgraph = find_highest_density_subgraph_with_start_node(&graph, start_node, max_depth);
            println!("Highest Density Subgraph with the starting node {} and depth of bfs {}: {:?} \n", start_node, max_depth, subgraph);

        },
        GraphOperationResult::NodeNotFound(node_id) => {
            eprintln!("Node {} not found in the graph", node_id);
        }
    }

    println!("Now printing some results for some random nodes and depths:");
    println!();
    let max_node_id = 300_000;
    let iterations = 3;
    let mut rng = rand::thread_rng();

    for _ in 0..iterations {
        let start_node = rng.gen_range(1..=max_node_id);
        let max_depth = rng.gen_range(3..=5);

        let result = graph.find_node(start_node);

        match result {
            GraphOperationResult::Success => {
                let subgraph = find_highest_density_subgraph_with_start_node(&graph, start_node, max_depth);
                println!("Highest Density Subgraph with the starting node {} and depth of bfs {}: {:?} \n", start_node, max_depth, subgraph);
            },
            GraphOperationResult::NodeNotFound(node_id) => {
                eprintln!("Node {} not found in the graph", node_id);
                println!();
            }
        }
    }

    Ok(())
}
