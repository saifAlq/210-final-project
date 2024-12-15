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

impl Graph {
    /// Calculate the degree of each node and return as a HashMap
    pub fn degree_distribution(&self) -> HashMap<String, usize> {
        let mut degree_map = HashMap::new();

        for (node, neighbors) in &self.adjacency_list {
            degree_map.insert(node.clone(), neighbors.len());
        }

        degree_map
    }

    /// Calculate the min, max, and average degree
    pub fn degree_statistics(&self) -> (usize, usize, f64) {
        let degree_map = self.degree_distribution();
        let mut min_degree = usize::MAX;
        let mut max_degree = usize::MIN;
        let mut total_degree = 0;

        for &degree in degree_map.values() {
            if degree < min_degree {
                min_degree = degree;
            }
            if degree > max_degree {
                max_degree = degree;
            }
            total_degree += degree;
        }

        let average_degree = total_degree as f64 / degree_map.len() as f64;

        (min_degree, max_degree, average_degree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());

        assert_eq!(graph.node_count(), 3); // 3 unique nodes
        assert_eq!(graph.edge_count(), 2); // 2 unique edges
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());
        graph.add_edge("C".to_string(), "D".to_string());

        let distances = graph.bfs(&"A".to_string());
        assert_eq!(distances["A"], 0); // Start node distance
        assert_eq!(distances["B"], 1);
        assert_eq!(distances["C"], 2);
        assert_eq!(distances["D"], 3);
    }

    #[test]
    fn test_clustering_coefficient() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());
        graph.add_edge("C".to_string(), "A".to_string()); // Triangle

        let coefficient = graph.clustering_coefficient(&"A".to_string());
        assert!((coefficient - 1.0).abs() < f64::EPSILON); // Fully connected neighbors

        graph.add_edge("A".to_string(), "D".to_string());
        let coefficient = graph.clustering_coefficient(&"A".to_string());
        assert!((coefficient - 0.3333).abs() < 0.001); // Adjusted coefficient
    }

    #[test]
    fn test_degree_statistics() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());
        graph.add_edge("C".to_string(), "D".to_string());

        let (min_degree, max_degree, avg_degree) = graph.degree_statistics();

        assert_eq!(min_degree, 1); // Nodes A and D have degree 1
        assert_eq!(max_degree, 2); // Nodes B and C have degree 2
        assert!((avg_degree - 1.5).abs() < f64::EPSILON); // Average degree
    }
}

