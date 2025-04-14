mod parser;
mod error;
mod variables;
mod tokenizer;
mod expressions;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Error reading file");

    match parser::parse(&code) {
        Ok(()) => {},
        Err(e) => eprintln!("Error: {}", e),
}
}
