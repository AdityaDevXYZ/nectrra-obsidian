use std::sync::{Arc, Mutex};

/// Represents a single logical step in the reasoning tree
pub struct ThoughtNode {
    pub logic_state: String,
    pub visits: u32,
    pub total_reward: f32,
    pub children: Vec<Arc<Mutex<ThoughtNode>>>,
}

impl ThoughtNode {
    pub fn new(logic_state: String) -> Self {
        Self {
            logic_state,
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
        }
    }

    /// Calculate Upper Confidence Bound applied to Trees (UCT)
    pub fn uct_score(&self, parent_visits: u32) -> f32 {
        if self.visits == 0 {
            return f32::MAX; // Unexplored nodes have infinite priority to ensure exploration
        }
        let exploitation = self.total_reward / (self.visits as f32);
        // Exploration parameter C = sqrt(2) approx 1.414
        let exploration = 1.414 * ((parent_visits as f32).ln() / (self.visits as f32)).sqrt();
        exploitation + exploration
    }
}

/// The orchestrator for the System 2 Swarm Reasoning
pub struct SwarmTree {
    pub root: Arc<Mutex<ThoughtNode>>,
}

impl SwarmTree {
    pub fn new(initial_prompt: String) -> Self {
        Self {
            root: Arc::new(Mutex::new(ThoughtNode::new(initial_prompt))),
        }
    }

    /// Simulates farming out a thought node to the global swarm and getting a float reward [-1.0, 1.0]
    pub fn decentralized_simulate(&self, node: &mut ThoughtNode) -> f32 {
        println!("[MCTS-Swarm] Broadcasting logic branch to peers for evaluation: '{}'", node.logic_state);
        // In a real P2P scenario, this blocks and awaits `obsidian_p2p::request_peer_evaluation()`
        // For the PoC architecture, we mock a high-confidence float response from the swarm.
        let mock_swarm_score = 0.85; 
        println!("[MCTS-Swarm] Received Value Score from Peer: {}", mock_swarm_score);
        mock_swarm_score
    }
    
    pub fn execute_search_cycle(&self) {
        let mut root = self.root.lock().unwrap();
        // 1. Selection & Expansion (Mocking 2 possible logic branches for the math problem)
        if root.children.is_empty() {
            root.children.push(Arc::new(Mutex::new(ThoughtNode::new("Hypothesis A: Apply Chain Rule".to_string()))));
            root.children.push(Arc::new(Mutex::new(ThoughtNode::new("Hypothesis B: Apply Product Rule".to_string()))));
        }
        
        // 2. Swarm Simulation on a child
        let mut child = root.children[0].lock().unwrap();
        let reward = self.decentralized_simulate(&mut child);
        
        // 3. Backpropagation
        child.visits += 1;
        child.total_reward += reward;
        root.visits += 1;
        root.total_reward += reward;
        
        println!("[MCTS-Backprop] Updated Tree. Root visits: {}, Child UCT Score: {:.3}", root.visits, child.uct_score(root.visits));
    }
}
