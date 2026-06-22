pub enum RouteDecision {
    /// Query is simple enough to be executed instantly by the local Ternary SSM
    LocalReflex,
    /// Query is too complex; package and route to the P2P swarm
    GlobalSwarm,
}

pub trait ComplexityEvaluator {
    /// Decides whether a prompt should be handled locally or globally
    fn evaluate_complexity(&self, query: &str) -> RouteDecision;
}

/// A simple heuristic evaluator for the PoC.
/// Designed to be easily swapped with the Ternary SSM confidence tensor score later.
pub struct HeuristicEvaluator;

impl ComplexityEvaluator for HeuristicEvaluator {
    fn evaluate_complexity(&self, query: &str) -> RouteDecision {
        // Condition 1: Exceeds ~150 characters (approx 40 tokens)
        if query.len() > 150 {
            return RouteDecision::GlobalSwarm;
        }

        // Condition 2: Contains explicit math/code triggers
        let triggers = ["=", "+", "fn ", "import ", "def ", "forall", "matrix"];
        for trigger in triggers.iter() {
            if query.contains(trigger) {
                return RouteDecision::GlobalSwarm;
            }
        }

        RouteDecision::LocalReflex
    }
}
