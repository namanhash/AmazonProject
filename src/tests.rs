#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::file_operations::read_graph_from_file;
    use crate::graph::{Graph};
    use crate::graph_analysis::{calculate_top_nodes, find_highest_density_subgraph_with_start_node, GraphData};
    use crate::graph_operations::{bfs, find_edges};

    #[test]
    fn test_densest_subgraph_density() {
        let filename = "test_data_3.txt";
        let graph_data = read_graph_from_file(filename).expect("Failed to read graph data");

        let mut graph = Graph::new();

        if let GraphData::Edges(edges) = graph_data {
            for edge in edges {
                graph.add_edge(edge);
            }
        } else {
            panic!("Invalid graph data format");
        }

        let start_node = 1;
        let max_depth = 2;
        let subgraph = find_highest_density_subgraph_with_start_node(&graph, start_node, max_depth);

        assert_eq!(subgraph.density, 1.5, "Density of the densest subgraph did not match expected value.");
    }

    #[test]
    fn test_top_node_and_connections() {
        let filename = "test_data_3.txt";
        let graph_data = read_graph_from_file(filename).expect("Failed to read graph data");

        let mut graph = Graph::new();

        if let GraphData::Edges(edges) = graph_data {
            for edge in edges {
                graph.add_edge(edge);
            }
        } else {
            panic!("Invalid graph data format");
        }

        let start_node = 1;
        let max_depth = 2;
        let reachable_nodes = bfs(&graph, start_node, max_depth);
        let edges_set = find_edges(&graph, &reachable_nodes);

        match calculate_top_nodes(&edges_set.into_iter().collect::<Vec<_>>(), 1) {
            GraphData::TopNodes(top_nodes) => {
                assert_eq!(top_nodes.len(), 1, "Expected only one top node.");
                let top_node = &top_nodes[0];
                assert_eq!(top_node.node, 1, "Expected top node to be node 1.");
                assert_eq!(top_node.count, 5, "Expected node 1 to have 5 connections.");
            },
            _ => panic!("Invalid top nodes data"),
        }
    }

    #[test]
    fn test_nodes_in_densest_subgraph() {
        let filename = "test_data_3.txt";
        let graph_data = read_graph_from_file(filename).expect("Failed to read graph data");

        let mut graph = Graph::new();

        if let GraphData::Edges(edges) = graph_data {
            for edge in edges {
                graph.add_edge(edge);
            }
        } else {
            panic!("Invalid graph data format");
        }

        let start_node = 9;
        let max_depth = 4;
        let subgraph = find_highest_density_subgraph_with_start_node(&graph, start_node, max_depth);

        let expected_nodes: HashSet<u32> = [12, 8, 6, 11, 10, 9, 1, 7].iter().cloned().collect();
        let subgraph_nodes: HashSet<u32> = subgraph.nodes;

        assert_eq!(subgraph_nodes, expected_nodes, "The nodes in the densest subgraph do not match the expected set.");
    }
}
