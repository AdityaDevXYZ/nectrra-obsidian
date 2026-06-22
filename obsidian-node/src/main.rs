pub mod split_brain;
pub mod mcts;
pub mod rlaif;
pub mod corpus;

use std::error::Error;
use split_brain::{ComplexityEvaluator, HeuristicEvaluator, RouteDecision};
use mcts::SwarmTree;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Obsidian Node...");
    
    // Spawn the background RLAIF data synthesis engine
    task::spawn(async {
        rlaif::run_idle_cycles().await;
    });
    
    // Initialize the Split-Brain Router
    let evaluator = HeuristicEvaluator;
    
    let complex_query = "fn calculate_derivative(matrix_x: Vec<f32>) { return matrix_x + 1.0; }";
    println!("\n[Split-Brain] Evaluating Query: '{}'", complex_query);
    route_query(&evaluator, complex_query);
    
    // Simulate Network Gradient Sync
    println!("\n[Network Sync] Listening for Swarm Gradient Updates...");
    let device = candle_core::Device::Cpu;
    let mut simulated_peers = Vec::new();
    
    // Generate simulated gradient vectors from the mesh
    simulated_peers.push(candle_core::Tensor::randn(0f32, 1f32, (4, 4), &device)?); // Good Peer
    simulated_peers.push(candle_core::Tensor::randn(0f32, 1f32, (4, 4), &device)?); // Good Peer
    simulated_peers.push(candle_core::Tensor::randn(100f32, 50f32, (4, 4), &device)?); // Malicious Peer (Massive Anomaly)

    let filter = obsidian_ml::aggregation::ByzantineFilter::new(1.5);
    if let Ok(_safe_update) = filter.filter_and_aggregate(simulated_peers) {
        println!("[Network Sync] Sovereign local model state successfully updated via Federated Learning.");
    }
    
    // Block main thread to allow the background RLAIF loops to run
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("\nObsidian Node Shutting Down (PoC completed successfully).");
    
    Ok(())
}

fn route_query(evaluator: &impl ComplexityEvaluator, query: &str) {
    match evaluator.evaluate_complexity(query) {
        RouteDecision::LocalReflex => {
            println!(" -> Decision: LOCAL.");
            if let Err(e) = obsidian_ml::dummy_inference() {
                eprintln!("Failed to run local inference: {}", e);
            }
        }
        RouteDecision::GlobalSwarm => {
            println!(" -> Decision: GLOBAL SWARM.");
            let swarm_tree = SwarmTree::new(query.to_string());
            for i in 1..=2 {
                println!("\n--- MCTS Search Cycle {} ---", i);
                swarm_tree.execute_search_cycle();
            }
        }
    }
}
