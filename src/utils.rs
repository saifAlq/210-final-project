use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

/// Loads the dataset from a file and returns edges as tuples.
pub fn load_dataset(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            edges.push((nodes[0].to_string(), nodes[1].to_string()));
        }
    }

    Ok(edges)
}

/// Cleans the dataset by removing duplicate edges.
pub fn clean_dataset(edges: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut unique_edges = edges.clone();
    unique_edges.sort();
    unique_edges.dedup(); // Remove duplicates
    unique_edges
}

