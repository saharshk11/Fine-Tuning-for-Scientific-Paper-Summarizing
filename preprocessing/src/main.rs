use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::error::Error;
use serde::{Deserialize, Serialize};
use sentencepiece::SentencePieceProcessor;

const READ_PATH: &str = "../data/test-AIC/dev.jsonl";
const WRITE_PATH: &str = "../data/test-AIC/processed/dev.jsonl";
const MODEL_PATH: &str = "models/pegasus/spiece.model";
const MAX_TOKENS: u32 = 1024;
const OVERLAP: u32 = 512;

#[derive(Debug, Deserialize)]
struct RawPaper {
    source: Vec<String>,
    target: Vec<String>
}

#[derive(Debug, Serialize)]
struct ProcessedPaper {
    source: Vec<String>,
    target: String
}

fn main() -> Result<(), Box<dyn Error>> {
    process_file()?;
    Ok(())
}

fn process_file() -> Result<(), Box<dyn Error>> {
    let read_file = File::open(READ_PATH)?; 
    let reader = BufReader::new(read_file);
    let write_file = OpenOptions::new().create(true).append(true).open(WRITE_PATH)?;
    let mut writer = BufWriter::new(write_file);

    for line_result in reader.lines() { 
        let line = line_result?;
        let paper: RawPaper = serde_json::from_str(&line)?;
        
        // combine data into one string
        let combined_source = paper.source.iter().fold(String::new(), |acc, val| acc + " " + val).trim().to_string();
        let combined_target = paper.target.iter().fold(String::new(), |acc, val| acc + " " + val).trim().to_string();
        
        // tokenize data
        let spp = SentencePieceProcessor::open(MODEL_PATH)?;
        let tokens = spp.encode(&combined_source)?;
        let tokens = tokens.into_iter().map(|p| p.id).collect::<Vec<u32>>();

        // chunk data
        let mut processed = ProcessedPaper{source:Vec::new(), target: combined_target};
        let size = tokens.len() as u32;
        if size > MAX_TOKENS {
            let mut pointer = 1024;
            while pointer <= size {
                // push chunk text
                processed.source.push(spp.decode_piece_ids(&tokens[((pointer-MAX_TOKENS) as usize)..(pointer as usize)])?);
                
                // overlap of MAX_TOKENS/2
                pointer += OVERLAP;
                if pointer > size && pointer-size != OVERLAP {
                    pointer = size;
                }
            }
        } else {
            processed.source.push(combined_source);
        }

        // write to file
        let json_line = serde_json::to_string(&processed)?;
        writeln!(writer, "{}", json_line)?;
    } 
                                                               
    Ok(())
}
