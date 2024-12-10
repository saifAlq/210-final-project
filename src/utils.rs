use std::fs::File; // For file handling
use std::io::{BufRead, BufReader}; // For reading files line by line
use std::error::Error; // For handling errors

/// Load the dataset from a file and return edges as tuples of (source, target)
pub fn load_dataset(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    // Open the file at the given path
    let file = File::open(file_path)?;
    let reader = BufReader::new(file); // Wrap the file in a buffered reader for efficient reading

    let mut edges = Vec::new(); // Create a vector to store the edges

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the line or return an error

        // Split the line into whitespace-separated parts
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            // If there are exactly two parts, treat them as an edge
            edges.push((nodes[0].to_string(), nodes[1].to_string()));
        }
    }

    Ok(edges) // Return the vector of edges
}

/// Clean the dataset by removing duplicate edges
pub fn clean_dataset(edges: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut unique_edges = edges.clone(); // Create a copy of the edge list
    unique_edges.sort(); // Sort the edges (to bring duplicates together)
    unique_edges.dedup(); // Remove duplicate edges
    unique_edges // Return the cleaned edge list
}
