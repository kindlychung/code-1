use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let filename = "README.md";
    let filename_lower = filename.to_lowercase();
    let f = File::open(filename).or(
        File::open(filename_lower)
    ).expect(&format!("Failed to open file: {}", filename));
    let reader = BufReader::new(f);

    for line_ in reader.lines() { // <1>
        let line = line_.unwrap(); // <2>
        println!("{} ({} bytes long)", line, line.len());
    }
}
