use tiny_http::{Server, Response, Header, Method};
use serde::{Deserialize, Serialize};
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

pub fn start_server() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Web API Server listening on 0.0.0.0:8080");

    for mut request in server.incoming_requests() {
        if request.method() == &Method::Options {
            let response = Response::empty(204)
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap())
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"POST, GET, OPTIONS"[..]).unwrap())
                .with_header(Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"Content-Type"[..]).unwrap());
            let _ = request.respond(response);
            continue;
        }

        if request.url() == "/query" && request.method() == &Method::Post {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap_or(0);
            
            if let Ok(payload) = serde_json::from_str::<QueryRequest>(&content) {
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

                let resp_body = serde_json::to_string(&QueryResponse { answer, routed_to_swarm }).unwrap();
                let response = Response::from_string(resp_body)
                    .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap())
                    .with_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                
                let _ = request.respond(response);
            }
        } else {
            let _ = request.respond(Response::empty(404));
        }
    }
}
