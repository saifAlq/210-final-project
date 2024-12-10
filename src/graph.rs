use std::collections::{HashMap, HashSet};

// Define the Graph structure
pub struct Graph {
    // Adjacency list to store nodes and their neighbors
    pub adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    /// Create a new, empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    /// Add an edge to the graph
    /// Since this is an undirected graph, add both directions
    pub fn add_edge(&mut self, source: String, target: String) {
        // Insert the target node into the source node's neighbor list
        self.adjacency_list
            .entry(source.clone())
            .or_insert_with(HashSet::new)
            .insert(target.clone());

        // Insert the source node into the target node's neighbor list
        self.adjacency_list
            .entry(target)
            .or_insert_with(HashSet::new)
            .insert(source);
    }

    /// Get the total number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.adjacency_list.len() // Count the number of keys in the adjacency list
    }

    /// Get the total number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.adjacency_list
            .values() // Access all neighbor lists
            .map(|neighbors| neighbors.len()) // Count the number of neighbors for each node
            .sum::<usize>() // Sum up all neighbor counts
            / 2 // Divide by 2 because each edge is counted twice in an undirected graph
    }
}
