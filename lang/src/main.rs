mod parser;
mod error;
mod variables;
mod tokenizer;
mod expressions;

use std::fs;
use std::env;
use std::process;
use crate::error::VortError;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    if !filename.ends_with(".vl") {
        eprintln!("Error: Wrong file extension. Use '.vl'.");
        process::exit(1);
    }

    let code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    match parser::parse(&code) {
        Ok(()) => {},
        Err(e) => {
            print_error(&e, &code);
            process::exit(1);
        }
    }
}

fn print_error(error: &VortError, source: &str) {
    eprintln!("\n{}", error);
    
    if let Some(span) = &error.span {
        let lines: Vec<&str> = source.lines().collect();
        if span.line <= lines.len() {
            let line = lines[span.line - 1];
            eprintln!("\n{:>4} | {}", span.line, line);
            
            // Print error pointer
            let padding = " ".repeat(span.column);
            let pointer = "^".repeat(span.length.max(1));
            eprintln!("     | {}{} {}", padding, pointer, error.message);
        }
    }
    
    match &error.kind {
        error::VortErrorKind::UndefinedVariable(var) => {
            eprintln!("\nHint: Variable '{}' is not defined. Make sure you declare it before use.", var);
        },
        error::VortErrorKind::MismatchedParentheses => {
            eprintln!("\nHint: Check your parentheses to ensure they are balanced.");
        },
        error::VortErrorKind::DivisionByZero => {
            eprintln!("\nHint: Division by zero is not allowed. Check your expression.");
        },
        error::VortErrorKind::TypeMismatch(msg) => {
            eprintln!("\nHint: Type mismatch - {}. Check variable types.", msg);
        },
        _ => {}
    }
}
