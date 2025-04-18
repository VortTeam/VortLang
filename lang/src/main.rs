// main.rs - Entry point for the Vortlang compiler
//
// This file serves as the main entry point for the Vortlang compiler.
// It orchestrates the compilation process by:
// 1. Parsing command-line arguments to get the source file
// 2. Reading the source code from the file
// 3. Coordinating the different phases of compilation (lexing, parsing, code generation)
// 4. Handling errors at each stage and providing useful feedback
// 5. Invoking the C compiler to produce the final executable
//
// The Vortlang compiler is structured as a multi-pass compiler:
// - Lexical analysis (tokenization) converts the source code into tokens
// - Parsing transforms the token stream into an Abstract Syntax Tree (AST)
// - Static analysis checks the AST for semantic errors
// - Code generation translates the AST into C code
// - The external C compiler (GCC) generates the final executable

// Import required modules
mod ast;        // Abstract Syntax Tree definitions
mod codegen;    // C code generation
mod errors;     // Error formatting and reporting
mod lexer;      // Lexical analysis
mod parser;     // Syntactic analysis

// Standard library imports
use std::env;                       // For accessing command-line arguments
use std::fs;                        // For file operations
use std::path::Path;                // For path manipulation
use std::process::{Command, exit};  // For executing external commands and program termination
use std::time::Instant;             // For tracking compilation duration

/// The main entry point for the Vortlang compiler.
///
/// This function handles command-line arguments, reads the source file,
/// and coordinates the compilation process. It provides error handling
/// and user feedback throughout the process.
fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a source file was provided
    if args.len() < 2 {
        println!("Usage: vortlang <source_file>");
        exit(1);
    }

    // Get the source file path from arguments
    let source_path = &args[1];
    
    // Read the source code from the file
    let source_code = match fs::read_to_string(source_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", source_path, e);
            exit(1);
        }
    };

    // Determine the output path based on the source file name
    let output_path = Path::new(source_path)
        .file_stem()                // Get the filename without extension
        .unwrap_or_default()        // Use default if the stem can't be extracted
        .to_str()                   // Convert to string
        .unwrap_or("output");       // Use "output" as fallback

    // Determine the source path stem for reporting purposes
    let source_path_stem = Path::new(source_path)
        .file_stem()                // Get the filename without extension
        .unwrap_or_default()        // Use default if the stem can't be extracted
        .to_str()                   // Convert to string
        .unwrap_or("source");       // Use "source" as fallback

    // Start tracking compilation time
    let start_time = Instant::now();

    // Compile the source code
    match compile(&source_code, source_path_stem, output_path) {
        Ok(_) => {
            let duration = start_time.elapsed();
            let formatted_duration = format_duration(duration);
            println!("Successfully compiled {}.vl to {}.exe in {}", 
                     source_path_stem, output_path, formatted_duration);
        },
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

/// Compiles the source code into an executable.
///
/// This function orchestrates the different phases of compilation:
/// 1. Lexical analysis (tokenization)
/// 2. Parsing into an AST
/// 3. Static analysis for warnings and optimizations
/// 4. Code generation to C
/// 5. Compilation of C code to an executable
///
/// # Arguments
///
/// * `source` - The source code to compile
/// * `source_path` - The path to the source file (for error reporting)
/// * `output_path` - The path where the output executable should be placed
///
/// # Returns
///
/// A Result indicating success or an error message
fn compile(source: &str, source_path: &str, output_path: &str) -> Result<(), String> {
    // Step 1: Lexical analysis (tokenization)
    // Convert the source code into a stream of tokens
    let tokens = match lexer::tokenize(source, source_path) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e),
    };

    // Step 2: Parsing
    // Convert the token stream into an Abstract Syntax Tree (AST)
    let ast = match parser::parse(tokens) {
        Ok(ast) => ast,
        Err(e) => return Err(e),
    };

    // Step 3: Static analysis
    // Check for semantic errors, dead code, and optimization opportunities
    let (ast, warnings) = ast::analyze(ast);
    
    // Display any warnings that were found
    for warning in warnings {
        eprintln!("Warning: {}", warning);
    }

    // Step 4: Code generation
    // Convert the AST into C code as an intermediate representation
    let c_code = codegen::generate_c_code(&ast)?;

    // Step 5: Write the generated C code to a temporary file
    let temp_c_file = format!("{}.c", output_path);
    fs::write(&temp_c_file, c_code)
        .map_err(|e| format!("Failed to write temporary C file: {}", e))?;

    // Step 6: Compile the C code to an executable using GCC
    let output = Command::new("gcc")
        .arg(&temp_c_file)
        .arg("-o")
        .arg(format!("{}.exe", output_path))
        .output()
        .map_err(|e| format!("Failed to execute gcc: {}", e))?;

    // Step 7: Clean up the temporary C file
    fs::remove_file(&temp_c_file)
        .map_err(|e| format!("Failed to remove temporary C file: {}", e))?;

    // Check if GCC compilation was successful
    if !output.status.success() {
        return Err(format!(
            "GCC compilation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Formats a duration into human-readable string with units of seconds (s), minutes (m), or hours (h).
/// The formatting follows these rules:
/// - 1-59 seconds: displays as "Xs" (e.g., "5s")
/// - 1-59 minutes: displays as "Xm" (e.g., "10m")
/// - 1 or more hours: displays as "Xh" (e.g., "2h")
///
/// # Arguments
/// * `duration` - The duration to format
///
/// # Returns
/// A formatted string representing the duration in the largest appropriate unit
fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        // Less than a minute: display in seconds
        format!("{}s", total_seconds)
    } else if total_seconds < 3600 {
        // Less than an hour: display in minutes (integer division)
        let minutes = total_seconds / 60;
        format!("{}m", minutes)
    } else {
        // 1 hour or more: display in hours
        let hours = total_seconds / 3600;
        format!("{}h", hours)
    }
}