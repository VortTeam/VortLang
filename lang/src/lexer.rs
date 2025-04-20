// lexer.rs - Lexical Analyzer for the Vortlang compiler
//
// This module implements the lexical analysis phase of the compiler, converting
// the raw source code into tokens that can be processed by the parser.
//
// The lexer reads the input source character by character, recognizing tokens
// such as keywords, identifiers, literals, and operators according to the
// language grammar. It also handles comments, whitespace, and reporting
// detailed lexical errors.

use crate::errors::{ErrorPosition, format_error};

/// Represents the different types of tokens in the Vortlang language.
///
/// Each variant corresponds to a specific lexical element of the language,
/// such as keywords, operators, literals, etc.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    /// The 'print' keyword for output statements
    Print,
    
    /// An identifier (variable name, function name, etc.)
    Identifier(String),
    
    /// A string literal enclosed in double quotes
    StringLiteral(String),
    
    /// A numerical literal (integer or floating-point)
    NumberLiteral(f64),
    
    /// Left parenthesis '('
    OpenParen,
    
    /// Right parenthesis ')'
    CloseParen,
    
    /// The 'let' keyword for string variable declaration
    Let,
    
    /// The 'num' keyword for numerical variable declaration
    Num,
    
    /// Assignment operator '='
    Equals,
    
    /// Addition operator '+'
    Plus,
    
    /// Subtraction operator '-'
    Minus,
    
    /// Multiplication operator '*'
    Star,
    
    /// Division operator '/'
    Slash,
    
    /// Format string prefix marker 'o' (used in print(o"..."))
    FormatStringPrefix,
    
    /// Newline character (important for line counting and statement separation)
    Newline,
    
    /// End of file marker
    EOF,
    
    /// The 'newfn' keyword for function definitions
    NewFn,
    
    /// The 'callfn' keyword for function calls
    CallFn,
    
    /// Opening brace '{' for function bodies
    OpenBrace,
    
    /// Closing brace '}' for function bodies
    CloseBrace,
}

/// Represents a token in the source code with its type and position information.
///
/// This structure combines the semantic type of the token with its location
/// in the source file, which is crucial for providing meaningful error messages.
#[derive(Debug, Clone)]
pub struct Token {
    /// The semantic type of the token
    pub token_type: TokenType,
    
    /// The line number where the token appears (1-based)
    pub line: usize,
    
    /// The column number where the token starts (1-based)
    pub column: usize,
}

/// Converts the source code into a sequence of tokens.
///
/// This function implements the lexical analysis phase, scanning the input
/// character by character to recognize and categorize the lexical elements
/// of the language.
///
/// # Arguments
///
/// * `source` - The source code to tokenize
/// * `source_path` - The path to the source file (for error reporting)
///
/// # Returns
///
/// A Result containing either:
/// * A vector of Token objects if tokenization was successful
/// * A formatted error message if a lexical error was encountered
pub fn tokenize(source: &str, source_path: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut column = 1;
    let mut chars = source.chars().peekable();

    // Process the source code character by character
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' => {
                // Skip whitespace but keep track of column position
                chars.next();
                column += 1;
            }
            '\n' => {
                // Record newlines for statement separation and error reporting
                tokens.push(Token {
                    token_type: TokenType::Newline,
                    line,
                    column,
                });
                chars.next();
                line += 1;
                column = 1;  // Reset column count for the new line
            }
            '/' => {
                chars.next();
                column += 1;

                if let Some('/') = chars.peek() {
                    // Single-line comment - skip everything until the end of line
                    chars.next();
                    column += 1;

                    while let Some(&next_c) = chars.peek() {
                        if next_c == '\n' {
                            break;
                        }
                        chars.next();
                        column += 1;
                    }
                } else {
                    // A standalone '/' is the division operator
                    tokens.push(Token {
                        token_type: TokenType::Slash,
                        line,
                        column: column - 1,
                    });
                }
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::OpenParen,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::CloseParen,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '=' => {
                tokens.push(Token {
                    token_type: TokenType::Equals,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::Plus,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::Minus,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '*' => {
                tokens.push(Token {
                    token_type: TokenType::Star,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::OpenBrace,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::CloseBrace,
                    line,
                    column,
                });
                chars.next();
                column += 1;
            }
            '"' => {
                // Process string literals enclosed in double quotes
                let start_column = column;
                chars.next(); // Skip opening quote
                column += 1;

                let mut string_content = String::new();
                let mut escaped = false;

                while let Some(&c) = chars.peek() {
                    if escaped {
                        // Handle escape sequences
                        match c {
                            'n' => string_content.push('\n'),
                            't' => string_content.push('\t'),
                            'r' => string_content.push('\r'),
                            '\\' => string_content.push('\\'),
                            '"' => string_content.push('"'),
                            _ => {
                                return Err(format_error(
                                    source_path,
                                    source,
                                    ErrorPosition { line, column },
                                    format!("Invalid escape sequence '\\{}'", c),
                                    "Valid escape sequences are: \\n, \\t, \\r, \\\", \\\\"
                                        .to_string(),
                                ));
                            }
                        }
                        escaped = false;
                    } else if c == '\\' {
                        // Start of escape sequence
                        escaped = true;
                    } else if c == '"' {
                        // End of string literal
                        break;
                    } else if c == '\n' {
                        // String literals cannot span multiple lines
                        return Err(format_error(
                            source_path,
                            source,
                            ErrorPosition {
                                line,
                                column: start_column,
                            },
                            "Unterminated string literal".to_string(),
                            "Add a closing quote to complete the string".to_string(),
                        ));
                    } else {
                        // Regular character in string
                        string_content.push(c);
                    }

                    chars.next();
                    column += 1;
                }

                // Check if the string was properly terminated
                if chars.peek().is_none() || chars.peek().unwrap() != &'"' {
                    return Err(format_error(
                        source_path,
                        source,
                        ErrorPosition {
                            line,
                            column: start_column,
                        },
                        "Unterminated string literal".to_string(),
                        "Add a closing quote to complete the string".to_string(),
                    ));
                }

                chars.next(); // Skip closing quote
                column += 1;

                tokens.push(Token {
                    token_type: TokenType::StringLiteral(string_content),
                    line,
                    column: start_column,
                });
            }
            '0'..='9' => {
                // Process numeric literals (integers and floats)
                let start_column = column;
                let mut number_str = String::new();
                let mut has_decimal = false;

                // Collect all digits and at most one decimal point
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        number_str.push(c);
                        chars.next();
                        column += 1;
                    } else if c == '.' && !has_decimal {
                        number_str.push(c);
                        has_decimal = true;
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                // Parse the collected string as a floating-point number
                match number_str.parse::<f64>() {
                    Ok(value) => {
                        tokens.push(Token {
                            token_type: TokenType::NumberLiteral(value),
                            line,
                            column: start_column,
                        });
                    },
                    Err(_) => {
                        return Err(format_error(
                            source_path,
                            source,
                            ErrorPosition {
                                line,
                                column: start_column,
                            },
                            format!("Invalid number format: {}", number_str),
                            "Ensure the number is correctly formatted".to_string(),
                        ));
                    }
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // Process identifiers and keywords
                let start_column = column;
                let mut identifier = String::new();

                // Collect all alphanumeric characters and underscores
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                // Check if the identifier is a reserved keyword
                match identifier.as_str() {
                    "print" => {
                        // Handle 'print' keyword and check for format string prefix
                        tokens.push(Token {
                            token_type: TokenType::Print,
                            line,
                            column: start_column,
                        });

                        // Check for '(' and potential format string prefix 'o'
                        if let Some(&c) = chars.peek() {
                            if c == '(' {
                                chars.next();
                                column += 1;

                                tokens.push(Token {
                                    token_type: TokenType::OpenParen,
                                    line,
                                    column: column - 1,
                                });

                                if let Some(&c) = chars.peek() {
                                    if c == 'o' {
                                        chars.next();
                                        column += 1;

                                        if let Some(&c) = chars.peek() {
                                            if c == '"' {
                                                tokens.push(Token {
                                                    token_type: TokenType::FormatStringPrefix,
                                                    line,
                                                    column: column - 1,
                                                });
                                            } else {
                                                return Err(format_error(
                                                    source_path,
                                                    source,
                                                    ErrorPosition { line, column },
                                                    "Expected '\"' after 'o' prefix".to_string(),
                                                    "Format strings should be written as: print(o\"...\")".to_string(),
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "let" => {
                        tokens.push(Token {
                            token_type: TokenType::Let,
                            line,
                            column: start_column,
                        });
                    }
                    "num" => {
                        tokens.push(Token {
                            token_type: TokenType::Num,
                            line,
                            column: start_column,
                        });
                    }
                    "plus" => {
                        // Support for readable operator keyword 'plus'
                        tokens.push(Token {
                            token_type: TokenType::Plus,
                            line,
                            column: start_column,
                        });
                    }
                    "minus" => {
                        // Support for readable operator keyword 'minus'
                        tokens.push(Token {
                            token_type: TokenType::Minus,
                            line,
                            column: start_column,
                        });
                    }
                    "times" | "multiply" => {
                        // Support for readable operator keywords 'times' and 'multiply'
                        tokens.push(Token {
                            token_type: TokenType::Star,
                            line,
                            column: start_column,
                        });
                    }
                    "divide" => {
                        // Support for readable operator keyword 'divide'
                        tokens.push(Token {
                            token_type: TokenType::Slash,
                            line,
                            column: start_column,
                        });
                    }
                    "newfn" => {
                        tokens.push(Token {
                            token_type: TokenType::NewFn,
                            line,
                            column: start_column,
                        });
                    }
                    "callfn" => {
                        tokens.push(Token {
                            token_type: TokenType::CallFn,
                            line,
                            column: start_column,
                        });
                    }
                    _ => {
                        // Regular identifier (variable name, etc.)
                        tokens.push(Token {
                            token_type: TokenType::Identifier(identifier),
                            line,
                            column: start_column,
                        });
                    }
                }
            }
            _ => {
                // Handle unexpected characters with detailed error message
                return Err(format_error(
                    source_path,
                    source,
                    ErrorPosition { line, column },
                    format!("Unexpected character '{}'", c),
                    "Remove or replace this character".to_string(),
                ));
            }
        }
    }

    // Add EOF token to mark the end of input
    tokens.push(Token {
        token_type: TokenType::EOF,
        line,
        column,
    });

    Ok(tokens)
}