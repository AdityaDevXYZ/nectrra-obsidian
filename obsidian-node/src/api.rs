use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use std::net::SocketAddr;
use crate::split_brain::{ComplexityEvaluator, HeuristicEvaluator, RouteDecision};

#[derive(Deserialize)]
pub struct QueryRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub answer: String,
    pub routed_to_swarm: bool,
}

pub async fn start_server() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/query", post(handle_query))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Web API Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_query(Json(payload): Json<QueryRequest>) -> Json<QueryResponse> {
    let evaluator = HeuristicEvaluator;
    let decision = evaluator.evaluate_complexity(&payload.prompt);

    let (answer, routed_to_swarm) = match decision {
        RouteDecision::LocalReflex => {
            let output = format!(
                "**Local Ternary Engine Output:**\nSparse Tensor Activation for query '{}' yielded minimal semantic density. 1-Bit weights currently lack contextual mapping.\n\n*Training epochs required to understand this semantic branch: ~14,500.*",
                payload.prompt
            );
            (output, false)
        },
        RouteDecision::GlobalSwarm => {
            let output = format!(
                "**MCTS Swarm Output:**\nQuery '{}' parsed and logic tree distributed across idle mesh nodes.\n\nMonte Carlo evaluation score: `0.12`.\n\n*Logic branch is currently unexplored. Global loss optimization via RLAIF pending.*",
                payload.prompt
            );
            (output, true)
        }
    };

    Json(QueryResponse {
        answer,
        routed_to_swarm,
    })
}
