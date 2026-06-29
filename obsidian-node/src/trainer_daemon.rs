use obsidian_ml::training::Trainer;
use obsidian_ml::model::ObsidianLLM;
use candle_core::{Tensor, Device};
use std::time::Instant;
use std::path::Path;

pub async fn run_training_loop() {
    println!("\n[Trainer Daemon] Initializing Federated Training Loop on AGI Architecture...");
    
    let device = Device::Cpu;
    let mut trainer = Trainer::new(0.01);
    
    // Initialize our massive LLM Architecture
    let mut llm = ObsidianLLM::new(100_000, 256, 4, &device).expect("Failed to init LLM");
    
    // Initialize the Phase 1 Data Pipeline
    // Path for Kaggle: /kaggle/working/dataset.txt and tokenizer.json
    let mut dataloader = crate::dataloader::ObsidianDataLoader::new(
        "/kaggle/working/dataset.txt", 
        "/kaggle/working/tokenizer.json", 
        16 // Sequence length
    ).expect("Failed to initialize DataLoader");

    let checkpoint_path = "/kaggle/working/checkpoint.safetensors";
    
    // Auto-Resume logic
    if Path::new(checkpoint_path).exists() {
        println!("[Trainer Daemon] Found existing checkpoint! Resuming training...");
        llm.load_checkpoint(checkpoint_path).unwrap();
    } else {
        println!("[Trainer Daemon] No checkpoint found. Starting pre-training from scratch.");
    }
    
    println!("[Trainer Daemon] Beginning training loop over real benchmark datasets...");
    
    // Track execution time to prevent Kaggle 12-hour forced termination
    let start_time = Instant::now();
    let kaggle_time_limit_secs = 42300; 

    // Run training over 14,500 epochs
    for epoch in 1..=14500 {
        // Fetch real tokens from the hard drive
        let (input_tokens, target_tokens) = match dataloader.next_batch() {
            Some(batch) => batch,
            None => {
                println!("[Trainer Daemon] Dataset epoch complete. Restarting stream...");
                // Loop back to the beginning of the text
                let _ = dataloader.next_batch(); 
                continue;
            }
        };
        
        // Convert [u32] slices to Tensors [batch_size=1, seq_len]
        let input_batch = Tensor::new(input_tokens.as_slice(), &device).unwrap().unsqueeze(0).unwrap();
        let target_batch = Tensor::new(target_tokens.as_slice(), &device).unwrap().unsqueeze(0).unwrap();
        
        // Execute the backward pass
        let loss = trainer.train_epoch(&mut llm, &input_batch, &target_batch).unwrap();
        
        if epoch % 100 == 0 || epoch == 1 {
            println!(" -> Epoch {}/14500 | STE Gradient Descent | Model Loss: {:.4}", epoch, loss);
        }
        
        // Auto-Stop Check
        let elapsed = start_time.elapsed().as_secs();
        if elapsed >= kaggle_time_limit_secs {
            println!("\n⚠️ [Kaggle Auto-Stop Triggered] ⚠️");
            println!("Time elapsed: {} seconds. Reaching the 12-hour Kaggle limit.", elapsed);
            println!("Gracefully halting training to save weights before forced termination!");
            break;
        }
        
        // Simulating compute time
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    // Serialize and save to disk
    llm.save_checkpoint(checkpoint_path).unwrap();
    println!("[Trainer Daemon] Training Session Complete. Safetensors written to disk.");
}
