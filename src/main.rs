mod utils; // Import the utilities module for dataset loading and cleaning
mod graph; // Import the graph module for graph-related operations

fn main() {
    // Path to the dataset file
    let dataset_path: &str = "../data/facebook_combined.txt";

    // Attempt to load the dataset using the utility function
    match utils::load_dataset(dataset_path) {
        // If the dataset is successfully loaded
        Ok(edges) => {
            println!("Loaded {} raw edges.", edges.len()); // Print the number of raw edges

            // Clean the dataset to remove duplicate edges
            let cleaned_edges: Vec<(String, String)> = utils::clean_dataset(edges);
            println!("Cleaned dataset contains {} unique edges.", cleaned_edges.len()); // Print the cleaned dataset size

            // Create and initialize an empty graph
            let mut graph: graph::Graph = graph::Graph::new();

            // Populate the graph by adding edges from the cleaned dataset
            for (source, target) in cleaned_edges {
                graph.add_edge(source, target); // Add each edge to the graph
            }

            // Print basic statistics about the constructed graph
            println!(
                "Graph constructed with {} nodes and {} edges.",
                graph.node_count(),   // Total number of nodes in the graph
                graph.edge_count()    // Total number of edges in the graph
            );

            // Perform BFS starting from a specific node (e.g., the first node in the dataset)
let start_node = "0".to_string(); 
println!("Performing BFS starting from node {}", start_node);

let distances = graph.bfs(&start_node);
println!("Distances from {} to the first 10 nodes:", start_node);

// Display distances to the first 10 nodes
for (node, distance) in distances.iter().take(10) {
    println!("Node: {}, Distance: {}", node, distance);
}
// Perform clustering coefficient calculations
println!("Calculating clustering coefficients...");

// Calculate clustering coefficient for a specific node
let specific_node = "0".to_string(); // Replace "0" with any valid node
let coefficient = graph.clustering_coefficient(&specific_node);
println!(
    "Clustering coefficient for node {}: {:.4}",
    specific_node, coefficient
);

// Calculate the average clustering coefficient for the entire graph
let average_coefficient = graph.average_clustering_coefficient();
println!("Average clustering coefficient for the graph: {:.4}", average_coefficient);
        }

        // If there is an error while loading the dataset
        Err(e) => eprintln!("Error loading dataset: {}", e),
    }
}

