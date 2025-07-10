use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let data_path = "../data/raw/SciTLDR-AIC/dev.jsonl";
    if let Ok(lines) = read_lines(data_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            break;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
