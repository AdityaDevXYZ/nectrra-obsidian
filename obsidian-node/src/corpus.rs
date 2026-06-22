use std::fs::OpenOptions;
use std::io::Write;

pub struct GoldenData {
    pub prompt: String,
    pub verified_answer: String,
    pub confidence_score: f32,
}

pub fn append_to_corpus(data: &GoldenData) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("golden_corpus.jsonl")?;

    // Manual JSON serialization for the PoC 
    let json = format!(
        "{{\"prompt\": \"{}\", \"verified_answer\": \"{}\", \"confidence_score\": {}}}",
        data.prompt.escape_default(),
        data.verified_answer.escape_default(),
        data.confidence_score
    );
    writeln!(file, "{}", json)?;
    Ok(())
}
