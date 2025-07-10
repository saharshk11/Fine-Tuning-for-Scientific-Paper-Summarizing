use std::fs::File;
use std::io::{self, BufRead, BufReader};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Paper {
    source: Vec<String>,
    target: Vec<String>
}


fn main() -> io::Result<()> {
    let data_path = "../data/raw/SciTLDR-AIC/dev.jsonl";
    let file = File::open(data_path)?;
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result?;
        let paper: Paper = serde_json::from_str(&line)?;
        println!("{:#?}", paper);
        break;
    }

    Ok(())
}
