use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
use crate::graph::Edge;
use crate::graph_analysis::GraphData;

pub fn read_graph_from_file(filename: &str) -> io::Result<GraphData> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges_set: HashSet<(u32, u32)> = HashSet::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }

        let from: u32 = parts[0].parse().unwrap();
        let to: u32 = parts[1].parse().unwrap();

        if !edges_set.contains(&(to, from)) {
            edges_set.insert((from, to));
            edges_set.insert((to, from)); 
        }
    }

    let edges: Vec<Edge> = edges_set.into_iter()
        .map(|(from, to)| Edge { from, to })
        .collect();

    Ok(GraphData::Edges(edges))
}
