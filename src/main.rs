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
        }

        // If there is an error while loading the dataset
        Err(e) => eprintln!("Error loading dataset: {}", e),
    }
}
