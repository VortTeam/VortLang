// errors.rs
use std::cmp::{max, min};

pub struct ErrorPosition {
    pub line: usize,
    pub column: usize,
}

pub fn format_error(
    source_path: &str,
    source: &str,
    pos: ErrorPosition,
    message: String,
    hint: String,
) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let line_idx = pos.line - 1;

    let mut error = format!("Error in {}:{}:{}\n", source_path, pos.line, pos.column);
    error.push_str(&format!("  {}\n", message));

    // Add source code context
    if line_idx < lines.len() {
        let line = lines[line_idx];
        error.push_str(&format!("\n{:>4} | {}\n", pos.line, line));

        // Add pointer to the error location
        let mut pointer = String::new();
        for _ in 0..pos.column - 1 {
            pointer.push(' ');
        }
        pointer.push('^');
        error.push_str(&format!("     | {}\n", pointer));
    }

    // Add hint
    error.push_str(&format!("\nHint: {}\n", hint));

    error
}

pub fn _highlight_code_region(
    source: &str,
    start_line: usize,
    start_col: usize,
    end_line: usize,
    end_col: usize,
) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = String::new();

    for line_num in max(1, start_line - 2)..=min(lines.len(), end_line + 2) {
        let line_idx = line_num - 1;
        result.push_str(&format!("{:>4} | {}\n", line_num, lines[line_idx]));

        if line_num >= start_line && line_num <= end_line {
            let mut underline = String::new();
            for _ in 0..lines[line_idx].len() {
                underline.push(' ');
            }

            let start = if line_num == start_line {
                start_col - 1
            } else {
                0
            };
            let end = if line_num == end_line {
                end_col - 1
            } else {
                lines[line_idx].len() - 1
            };

            for i in start..=end {
                if i < underline.len() {
                    underline.replace_range(i..i + 1, "~");
                }
            }

            result.push_str(&format!("     | {}\n", underline));
        }
    }

    result
}