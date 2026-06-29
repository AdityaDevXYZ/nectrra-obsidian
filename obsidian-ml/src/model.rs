use candle_core::{Tensor, Device, Error};
use candle_nn::{Embedding, Linear, VarBuilder, VarMap, Module};
use crate::ssm::SsmStep;

pub struct ObsidianLLM {
    pub varmap: VarMap,
    pub embedding: Embedding,
    pub layers: Vec<SsmStep>,
    pub lm_head: Linear,
    pub hidden_dim: usize,
    pub vocab_size: usize,
}

impl ObsidianLLM {
    pub fn new(vocab_size: usize, hidden_dim: usize, num_layers: usize, device: &Device) -> Result<Self, Error> {
        let varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, candle_core::DType::F32, device);
        
        let embedding = candle_nn::embedding(vocab_size, hidden_dim, vb.pp("embedding"))?;
        let lm_head = candle_nn::linear_no_bias(hidden_dim, vocab_size, vb.pp("lm_head"))?;
        
        let mut layers = Vec::new();
        for i in 0..num_layers {
            layers.push(SsmStep::new(hidden_dim, hidden_dim, vb.pp(&format!("ssm_layer_{}", i)))?);
        }
        
        Ok(Self {
            varmap,
            embedding,
            layers,
            lm_head,
            hidden_dim,
            vocab_size,
        })
    }

    /// Forward pass through the LLM. 
    /// Takes a tensor of token IDs [batch_size, seq_len]
    /// Returns logits [batch_size, seq_len, vocab_size]
    pub fn forward(&self, token_ids: &Tensor) -> Result<Tensor, Error> {
        let (_batch_size, seq_len) = token_ids.dims2()?;
        let device = token_ids.device();
        
        // 1. Embedding Layer: [batch_size, seq_len] -> [batch_size, seq_len, hidden_dim]
        let mut hidden_states = self.embedding.forward(token_ids)?;
        
        // 2. Pass through Ternary SSM layers
        for layer in self.layers.iter() {
            // Since this is a sequence model, we ideally process token by token or use parallel scan.
            // For now, we simulate a recurrent step over the sequence length.
            let mut step_outputs = Vec::new();
            let mut h_prev = Tensor::zeros((_batch_size, self.hidden_dim), candle_core::DType::F32, device)?;
            
            for t in 0..seq_len {
                // Get the hidden state for this token: [batch_size, hidden_dim]
                let x_t = hidden_states.narrow(1, t, 1)?.squeeze(1)?;
                let (y_t, h_t) = layer.forward_step(&x_t, &h_prev)?;
                h_prev = h_t;
                step_outputs.push(y_t.unsqueeze(1)?); // [batch_size, 1, hidden_dim]
            }
            
            // Concatenate along the sequence dimension: [batch_size, seq_len, hidden_dim]
            hidden_states = Tensor::cat(&step_outputs, 1)?;
        }
        
        // 3. LM Head: project back to vocabulary logits
        let logits = self.lm_head.forward(&hidden_states)?;
        
        Ok(logits)
    }

    /// Serializes the model weights to a .safetensors file for Kaggle checkpointing
    pub fn save_checkpoint(&self, path: &str) -> Result<(), Error> {
        println!("[ObsidianLLM] Serializing true tracked weights to {}...", path);
        self.varmap.save(path)?;
        println!("[ObsidianLLM] Checkpoint saved successfully.");
        Ok(())
    }

    /// Loads the model weights from a .safetensors file to resume Kaggle training
    pub fn load_checkpoint(&mut self, path: &str) -> Result<(), Error> {
        println!("[ObsidianLLM] Restoring true tracked weights from {}...", path);
        self.varmap.load(path)?;
        println!("[ObsidianLLM] Checkpoint restored successfully.");
        Ok(())
    }
}
