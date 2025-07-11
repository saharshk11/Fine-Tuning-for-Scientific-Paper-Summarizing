use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use serde::Deserialize;
use sentencepiece::SentencePieceProcessor;

#[derive(Debug, Deserialize)]
struct RawPaper {
    source: Vec<String>,
    target: Vec<String>
}

fn main() -> Result<(), Box<dyn Error>>{
    let data_path = "../data/test-AIC/dev.jsonl";
    let model_path = "models/pegasus/spiece.model";
    process_file(data_path, model_path)?;

    Ok(())
}

fn process_file(path: &str, model_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?; 
    let reader = BufReader::new(file);

    for line_result in reader.lines() { 
        let line = line_result?;
        let paper: RawPaper = serde_json::from_str(&line)?;

        let combined_source = paper.source.iter().fold(String::new(), |acc, val| acc + val).trim().to_string();
        let combined_target = paper.target.iter().fold(String::new(), |acc, val| acc + val).trim().to_string();

        let num_tokens = count_tokens(&combined_source, model_path)?;
        println!("Length of text: {num_tokens}");
    } 
                                                               
    Ok(())
}

fn count_tokens(text: &String, model_path: &str) -> Result<u32, Box<dyn Error>>{
    let spp = SentencePieceProcessor::open(model_path)?;
    let pieces = spp.encode(text)?;
    let pieces = pieces.into_iter().map(|p| p.piece).collect::<Vec<_>>();


    Ok(pieces.len() as u32)
}
