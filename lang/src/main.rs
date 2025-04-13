mod parser;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }

    // Read the content of the VortLang(.vl) file
    let filename = &args[1];
    let code = fs::read_to_string(filename).expect("Error reading file");

    
    match parser::parse(&code) {
        Ok(_) => println!(" "),
        Err(e) => eprintln!("Error: {}", e),
    }
}
