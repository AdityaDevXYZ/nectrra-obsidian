pub mod split_brain;
pub mod mcts;
pub mod rlaif;
pub mod corpus;
pub mod trainer_daemon;
pub mod api;

use std::error::Error;
use split_brain::{ComplexityEvaluator, HeuristicEvaluator, RouteDecision};
use mcts::SwarmTree;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting Obsidian Node...");
    println!("Starting local P2P swarm node...");
    
    // Spawn Background Autonomous Data Generator
    task::spawn(async {
        rlaif::run_idle_cycles().await;
    });

    let evaluator = HeuristicEvaluator;
    
    // Test the Split-Brain Router
    let prompt1 = "Hello, what is your status?";
    match evaluator.evaluate_complexity(prompt1) {
        RouteDecision::LocalReflex => println!("Query 1 routed to Local Ternary Reflex Engine."),
        RouteDecision::GlobalSwarm => println!("Query 1 routed to Decentralized Swarm."),
    }

    let prompt2 = "Calculate the prime factorization of 1204812.";
    match evaluator.evaluate_complexity(prompt2) {
        RouteDecision::LocalReflex => println!("Query 2 routed to Local Ternary Reflex Engine."),
        RouteDecision::GlobalSwarm => println!("Query 2 routed to Decentralized Swarm."),
    }
    
    // Block main thread to allow the background RLAIF loops to run
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Execute Full Training Phase
    trainer_daemon::run_training_loop().await;
    
    // Spawn Web API Server in a dedicated system thread (blocking)
    std::thread::spawn(move || {
        api::start_server();
    });
    
    println!("\nObsidian Daemon is now running indefinitely. Awaiting federated training cycles...");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
    }
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
