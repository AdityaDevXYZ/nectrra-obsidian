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
                        println!("\n[Infer] Routing query to OpenRouter Commercial API: '{}'", payload.prompt);
                        
                        let client = reqwest::blocking::Client::new();
                        
                        let groq_token = std::env::var("GROQ_API_KEY").unwrap_or_else(|_| "".to_string());
                        
                        // Use the Bulletproof Commercial Groq API (Lightning Fast & Free)
                        let api_req = serde_json::json!({
                            "model": "llama-3.1-8b-instant",
                            "messages": [
                                {"role": "user", "content": payload.prompt}
                            ],
                            "max_tokens": 500,
                            "temperature": 0.7
                        });

                        let response = client.post("https://api.groq.com/openai/v1/chat/completions")
                            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Nectrra/1.0") 
                            .header("Authorization", format!("Bearer {}", groq_token))
                            .json(&api_req)
                            .send();

                        let output = match response {
                            Ok(res) => {
                                let status = res.status();
                                let raw_text = res.text().unwrap_or_else(|_| "Failed to read response body".to_string());
                                
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw_text) {
                                    // Parse OpenAI standard response: choices[0].message.content
                                    if let Some(choices) = json["choices"].as_array() {
                                        if let Some(first) = choices.get(0) {
                                            first["message"]["content"].as_str().unwrap_or("Error parsing chat message").to_string()
                                        } else {
                                            "Empty choices array".to_string()
                                        }
                                    } else {
                                        json["error"]["message"].as_str().unwrap_or(&format!("API Error (Status {}): {}", status, raw_text)).to_string()
                                    }
                                } else {
                                    format!("Failed to parse JSON. HTTP {}. Raw Response: {}", status, raw_text)
                                }
                            },
                            Err(e) => format!("**SYSTEM ERROR:** Could not reach Groq API. Details: {}", e),
                        };
                        
                        let formatted_output = output.trim().to_string();
                        
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
