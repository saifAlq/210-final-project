mod utils; // Import the utils module

fn main() {
    let dataset_path = "../data/facebook_combined.txt";

    match utils::load_dataset(dataset_path) {
        Ok(edges) => {
            println!("Loaded {} raw edges.", edges.len());
            let cleaned_edges = utils::clean_dataset(edges);
            println!("Cleaned dataset contains {} unique edges.", cleaned_edges.len());
        },
        Err(e) => eprintln!("Error loading dataset: {}", e),
    }
}
