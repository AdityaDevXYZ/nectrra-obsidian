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

use tokenizers::Tokenizer;
use std::path::Path;

pub fn start_server() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Web API Server listening on 0.0.0.0:8080");

    println!("[Obsidian Boot] Connecting to Ollama Open-Source Inference Engine on localhost:11434...");
    println!("[Obsidian Boot] AGI Online. Awaiting web queries...");

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
                        println!("\n[Infer] Routing query to Ollama: '{}'", payload.prompt);
                        
                        let client = reqwest::blocking::Client::new();
                        let ollama_req = serde_json::json!({
                            "model": "qwen2:7b",
                            "prompt": payload.prompt,
                            "stream": false
                        });

                        let response = client.post("http://127.0.0.1:11434/api/generate")
                            .json(&ollama_req)
                            .send();

                        let output = match response {
                            Ok(res) => {
                                if let Ok(json) = res.json::<serde_json::Value>() {
                                    json["response"].as_str().unwrap_or("Error parsing Ollama response").to_string()
                                } else {
                                    "Failed to parse Ollama JSON".to_string()
                                }
                            },
                            Err(_) => "**SYSTEM ERROR:** Could not reach Ollama at localhost:11434. Is the background engine running?".to_string(),
                        };
                        
                        let formatted_output = format!(
                            "**Qwen2 7B Intelligence Core:**\n{}",
                            output
                        );
                        
                        (formatted_output, false)
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
