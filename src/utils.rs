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
    let mut seen = std::collections::HashSet::new();
    let mut unique_edges = Vec::new();

    for (a, b) in edges {
        // Ensure edges are stored in a consistent order (smallest first)
        let edge = if a < b { (a.clone(), b.clone()) } else { (b.clone(), a.clone()) };
        if seen.insert(edge.clone()) {
            unique_edges.push(edge);
        }
    }

    unique_edges
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dataset() {
        let edges = vec![
            "A B",
            "B C",
            "C D",
            "D E",
        ];

        let file_path = "test_dataset.txt";
        std::fs::write(file_path, edges.join("\n")).unwrap();

        let result = load_dataset(file_path).unwrap();
        std::fs::remove_file(file_path).unwrap(); // Cleanup test file

        assert_eq!(result.len(), 4); // Verify all edges are loaded
        assert_eq!(result[0], ("A".to_string(), "B".to_string())); // Check first edge
        assert_eq!(result[3], ("D".to_string(), "E".to_string())); // Check last edge
    }

    #[test]
    fn test_clean_dataset() {
        // Test edges with duplicates and unordered pairs
        let edges = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "A".to_string()), // Duplicate (unordered)
            ("A".to_string(), "B".to_string()), // Duplicate (ordered)
            ("C".to_string(), "D".to_string()),
        ];

        // Clean the dataset
        let cleaned = clean_dataset(edges);

        // Check that only unique, normalized edges remain
        assert_eq!(cleaned.len(), 2); // Expect 2 unique edges
        assert!(cleaned.contains(&("A".to_string(), "B".to_string()))); // Ensure (A, B) is present
        assert!(cleaned.contains(&("C".to_string(), "D".to_string()))); // Ensure (C, D) is present
    }
}

