use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use serde::Deserialize;
use sentencepiece::{SentencePieceProcessor, SentencePieceError};

const DATA_PATH: &str = "../data/test-AIC/dev.jsonl";
const MODEL_PATH: &str = "models/pegasus/spiece.model";
const MAX_TOKENS: u32 = 1024;

#[derive(Debug, Deserialize)]
struct RawPaper {
    source: Vec<String>,
    target: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    process_file(DATA_PATH)?;
    Ok(())
}

fn process_file(path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?; 
    let reader = BufReader::new(file);

    for line_result in reader.lines() { 
        let line = line_result?;
        let paper: RawPaper = serde_json::from_str(&line)?;
        
        // comine data into one string
        let combined_source = paper.source.iter().fold(String::new(), |acc, val| acc + " " + val).trim().to_string();
        let combined_target = paper.target.iter().fold(String::new(), |acc, val| acc + " " + val).trim().to_string();
        
        // tokenize data
        let spp = SentencePieceProcessor::open(MODEL_PATH)?;
        let tokens = spp.encode(&combined_source)?;
        let tokens = tokens.into_iter().map(|p| p.id).collect::<Vec<u32>>();

        // chunk data
        let mut chunks = Vec::new();
        if (tokens.len() as u32) > MAX_TOKENS {
            todo!("Chunking data...");
        } else {
            chunks.push(combined_source);
        }
    } 
                                                               
    Ok(())
}
