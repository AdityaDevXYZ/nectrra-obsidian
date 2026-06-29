use tokenizers::Tokenizer;
use std::fs;
use std::path::Path;

pub struct ObsidianDataLoader {
    pub tokenizer: Tokenizer,
    pub token_stream: Vec<u32>,
    pub current_idx: usize,
    pub seq_len: usize,
}

impl ObsidianDataLoader {
    pub fn new(dataset_path: &str, tokenizer_path: &str, seq_len: usize) -> Result<Self, String> {
        // 1. Initialize the massive HuggingFace Tokenizer
        let tokenizer = if Path::new(tokenizer_path).exists() {
            Tokenizer::from_file(tokenizer_path).map_err(|e| e.to_string())?
        } else {
            return Err("Kaggle Tokenizer JSON not found!".to_string());
        };
        
        // 2. Load the unified dataset (FineWeb + TinyStories)
        let text = if Path::new(dataset_path).exists() {
            fs::read_to_string(dataset_path).map_err(|e| e.to_string())?
        } else {
            println!("[DataLoader] Warning: No dataset found. Falling back to synthetic logic testing.");
            "Obsidian is a highly advanced Artificial General Intelligence capable of deep reasoning. It processes neural branches simultaneously.".to_string()
        };
        
        // 3. Tokenize the entire dataset into an integer stream
        let encoding = tokenizer.encode(text, false).unwrap_or_default();
        let token_stream = encoding.get_ids().to_vec();
        
        println!("[DataLoader] Successfully tokenized dataset into {} integer tokens.", token_stream.len());

        Ok(Self {
            tokenizer,
            token_stream,
            current_idx: 0,
            seq_len,
        })
    }

    /// Fetches the next (Input, Target) batch for the Neural Network.
    /// The Target is simply the Input shifted by 1 token into the future.
    pub fn next_batch(&mut self) -> Option<(Vec<u32>, Vec<u32>)> {
        // We need seq_len + 1 tokens to create an input and a target
        if self.current_idx + self.seq_len + 1 > self.token_stream.len() {
            // End of dataset (epoch complete)
            self.current_idx = 0; 
            return None; 
        }

        let chunk = &self.token_stream[self.current_idx..(self.current_idx + self.seq_len + 1)];
        let input = chunk[0..self.seq_len].to_vec();
        let target = chunk[1..=self.seq_len].to_vec();

        // Stride forward by seq_len for non-overlapping batches
        self.current_idx += self.seq_len;

        Some((input, target))
    }
}
