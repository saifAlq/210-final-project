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

impl Graph {
    /// Perform BFS from a given start node
    pub fn bfs(&self, start: &String) -> HashMap<String, usize> {
        use std::collections::{HashMap, VecDeque};
        
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        distances.insert(start.clone(), 0);
        queue.push_back(start.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !distances.contains_key(neighbor) {
                        distances.insert(neighbor.clone(), distances[&current] + 1);
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        distances
    }
}

impl Graph {
    /// Calculate the clustering coefficient for a specific node
    pub fn clustering_coefficient(&self, node: &String) -> f64 {
        // Get neighbors of the node
        if let Some(neighbors) = self.adjacency_list.get(node) {
            let neighbors: Vec<&String> = neighbors.iter().collect();
            let neighbor_count = neighbors.len();

            // If the node has less than 2 neighbors, clustering coefficient is 0
            if neighbor_count < 2 {
                return 0.0;
            }

            // Count the edges between neighbors
            let mut edges_between_neighbors = 0;
            for i in 0..neighbor_count {
                for j in (i + 1)..neighbor_count {
                    if let Some(neighbor_edges) = self.adjacency_list.get(neighbors[i]) {
                        if neighbor_edges.contains(neighbors[j]) {
                            edges_between_neighbors += 1;
                        }
                    }
                }
            }

            // Total possible edges between neighbors
            let possible_edges = neighbor_count * (neighbor_count - 1) / 2;

            // Clustering coefficient
            return edges_between_neighbors as f64 / possible_edges as f64;
        }

        // Return 0 if the node does not exist
        0.0
    }

    /// Calculate the average clustering coefficient for the entire graph
    pub fn average_clustering_coefficient(&self) -> f64 {
        let mut total_coefficient = 0.0;
        let mut node_count = 0;

        for node in self.adjacency_list.keys() {
            total_coefficient += self.clustering_coefficient(node);
            node_count += 1;
        }

        if node_count == 0 {
            return 0.0;
        }

        total_coefficient / node_count as f64
    }
}
