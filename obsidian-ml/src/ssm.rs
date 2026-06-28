use candle_core::{Tensor, Device, Error};
use crate::ternary::TernaryWeight;

/// A single step in a State Space Model recurrence
pub struct SsmStep {
    a_proj: TernaryWeight, // Hidden state transition matrix
    b_proj: TernaryWeight, // Input projection matrix
    c_proj: TernaryWeight, // Output projection matrix
}

impl SsmStep {
    pub fn new(input_dim: usize, hidden_dim: usize, device: &Device) -> Result<Self, Error> {
        Ok(Self {
            // A matrix: hidden_dim -> hidden_dim
            a_proj: TernaryWeight::new((hidden_dim, hidden_dim), device)?,
            // B matrix: input_dim -> hidden_dim
            b_proj: TernaryWeight::new((input_dim, hidden_dim), device)?,
            // C matrix: hidden_dim -> input_dim (or output dim)
            c_proj: TernaryWeight::new((hidden_dim, input_dim), device)?,
        })
    }

    /// h_t = SiLU(A * h_{t-1} + B * x_t)
    /// y_t = C * h_t
    pub fn forward_step(&self, x_t: &Tensor, h_prev: &Tensor) -> Result<(Tensor, Tensor), Error> {
        // 1. Input influence: B * x_t
        let input_influence = self.b_proj.forward_simulate_add(x_t)?;
        
        // 2. Hidden state transition: A * h_{t-1}
        let state_transition = self.a_proj.forward_simulate_add(h_prev)?;
        
        // 3. New hidden state: h_t = SiLU(A * h_{t-1} + B * x_t)
        let h_raw = state_transition.broadcast_add(&input_influence)?;
        let h_t = candle_nn::ops::silu(&h_raw)?;
        
        // 4. Output projection: y_t = C * h_t
        let y_t = self.c_proj.forward_simulate_add(&h_t)?;
        
        Ok((y_t, h_t))
    }
}
