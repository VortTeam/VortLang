// parser.rs - Syntactic analyzer for the Vortlang compiler
//
// This module implements the parsing phase of compilation, which transforms
// the token stream from the lexer into an Abstract Syntax Tree (AST).
//
// The parser validates the syntax of the program according to the language
// grammar rules, building a structured representation that captures the
// hierarchical relationships between language constructs.
//
// The implementation uses recursive descent parsing, with separate functions
// for each non-terminal in the grammar. Error reporting includes contextual
// information to help users understand and fix syntax issues.

use crate::ast::{BinaryOperator, Expression, NumExpression, Statement};
use crate::errors::{ErrorPosition, format_error};
use crate::lexer::{Token, TokenType};

/// The parser structure that manages the token stream and builds the AST.
///
/// This struct maintains the current state of parsing, including the tokens
/// being processed, the current position in the token stream, and references
/// to the source code for error reporting.
pub struct Parser {
    /// The complete sequence of tokens from the lexer
    tokens: Vec<Token>,
    
    /// Current position in the token stream
    current: usize,
    
    /// Original source code (for error reporting)
    source: String,
    
    /// Path to the source file (for error reporting)
    source_path: String,
}

impl Parser {
    /// Creates a new Parser instance with the given tokens and source information.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The sequence of tokens to parse
    /// * `source` - The original source code (for error reporting)
    /// * `source_path` - The path to the source file (for error reporting)
    ///
    /// # Returns
    ///
    /// A new Parser instance ready to begin parsing
    pub fn new(tokens: Vec<Token>, source: String, source_path: String) -> Self {
        Parser {
            tokens,
            current: 0,
            source,
            source_path,
        }
    }

    /// Returns the current token without consuming it.
    ///
    /// If the end of the token stream has been reached, returns the EOF token.
    ///
    /// # Returns
    ///
    /// A reference to the current token
    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1] // Return EOF token
        } else {
            &self.tokens[self.current]
        }
    }

    /// Consumes the current token and advances to the next one.
    ///
    /// # Returns
    ///
    /// A reference to the token that was just consumed
    fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.previous()
    }

    /// Returns the token that was most recently consumed.
    ///
    /// # Returns
    ///
    /// A reference to the previously consumed token
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Checks if the current token matches the specified type without consuming it.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The token type to check against
    ///
    /// # Returns
    ///
    /// `true` if the current token matches the specified type, `false` otherwise
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
    }

    /// Consumes the current token if it matches the specified type.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The token type to match
    ///
    /// # Returns
    ///
    /// `true` if the token was matched and consumed, `false` otherwise
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(&token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the current token if it matches the specified type.
    /// Otherwise, reports an error with the given message.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The expected token type
    /// * `message` - The error message to report if the token doesn't match
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A reference to the consumed token if successful
    /// * A formatted error message if the token doesn't match
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let token = self.peek();
            Err(format_error(
                &self.source_path,
                &self.source,
                ErrorPosition {
                    line: token.line,
                    column: token.column,
                },
                message.to_string(),
                "Check your syntax and try again".to_string(),
            ))
        }
    }

    /// Checks if the parser has reached the end of the token stream.
    ///
    /// # Returns
    ///
    /// `true` if at the end of the token stream, `false` otherwise
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::EOF)
    }

    /// Skips any consecutive newline tokens in the token stream.
    ///
    /// This is used to handle blank lines in the source code.
    fn skip_newlines(&mut self) {
        while matches!(self.peek().token_type, TokenType::Newline) {
            self.advance();
        }
    }

    /// Parses the entire token stream into an AST.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A vector of Statement objects representing the program
    /// * A formatted error message if parsing fails
    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        self.skip_newlines();

        // Parse statements until we reach the end of the file
        while !self.is_at_end() {
            statements.push(self.statement()?);
            self.skip_newlines();
        }

        Ok(statements)
    }

    /// Parses a single statement.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A Statement object
    /// * A formatted error message if parsing fails
    fn statement(&mut self) -> Result<Statement, String> {
        // Check for standalone assignments first
        if let TokenType::Identifier(_) = self.peek().token_type {
            let next_pos = self.current + 1;
            if next_pos < self.tokens.len() && matches!(self.tokens[next_pos].token_type, TokenType::Equals) {
                return self.assignment_statement();
            }
        }
    
    if self.match_token(TokenType::Print) {
        self.print_statement()
    } else if self.match_token(TokenType::Let) {
        self.let_statement()
    } else if self.match_token(TokenType::Num) {
        self.num_statement()
    } else {
        let token = self.peek().clone();
        Err(format_error(
            &self.source_path,
            &self.source,
            ErrorPosition {
                line: token.line,
                column: token.column,
            },
            "Expected statement".to_string(),
            "Valid statements are 'print', 'let', and 'num'".to_string(),
        ))
    }
    }

    fn assignment_statement(&mut self) -> Result<Statement, String> {
        let line_number = self.peek().line;
        
        // Get the variable name
        let name_token = self.advance();
        let name = match &name_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };
    
        self.consume(TokenType::Equals, "Expected '=' after variable name")?;
    
        // Try parsing as numeric expression first
        match self.num_expression() {
            Ok(num_expr) => {
                Ok(Statement::NumAssignment(name, num_expr, line_number))
            }
            Err(_) => {
                // If numeric parsing fails, try string expression
                match self.expression() {
                    Ok(str_expr) => {
                        Ok(Statement::VariableAssignment(name, str_expr, line_number))
                    }
                    Err(_) => {
                        Err(format_error(
                            &self.source_path,
                            &self.source,
                            ErrorPosition {
                                line: line_number,
                                column: self.peek().column,
                            },
                            format!("Invalid assignment to variable '{}'", name),
                            "Variables can only be assigned string or numeric values".to_string(),
                        ))
                    }
                }
            }
        }
    }

    /// Parses a print statement.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A Print or PrintFormat Statement object
    /// * A formatted error message if parsing fails
    fn print_statement(&mut self) -> Result<Statement, String> {
        self.consume(TokenType::OpenParen, "Expected '(' after 'print'")?;

        let format_string = matches!(self.peek().token_type, TokenType::FormatStringPrefix);
        if format_string {
            self.advance(); // Consume the format string prefix
        }

        let expression = self.expression()?;

        self.consume(TokenType::CloseParen, "Expected ')' after expression")?;

        if format_string {
            Ok(Statement::PrintFormat(expression))
        } else {
            Ok(Statement::Print(expression))
        }
    }

    /// Parses a string variable declaration statement.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A VariableDeclaration Statement object
    /// * A formatted error message if parsing fails
    fn let_statement(&mut self) -> Result<Statement, String> {
        // Store the current line number for error reporting
        let line_number = self.peek().line;
        
        let name_token = self.consume(
            TokenType::Identifier("".to_string()),
            "Expected variable name",
        )?;
        let name = match &name_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        self.consume(TokenType::Equals, "Expected '=' after variable name")?;

        let initializer = self.expression()?;

        Ok(Statement::VariableDeclaration(name, initializer, line_number))
    }

    /// Parses a numerical variable declaration statement.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A NumDeclaration Statement object
    /// * A formatted error message if parsing fails
    fn num_statement(&mut self) -> Result<Statement, String> {
        // Store the current line number for error reporting
        let line_number = self.peek().line;
        
        // Get the variable name
        let name_token = self.consume(
            TokenType::Identifier("".to_string()),
            "Expected variable name after 'num'",
        )?;
        let name = match &name_token.token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => unreachable!(),
        };

        // Expect assignment operator
        self.consume(TokenType::Equals, "Expected '=' after variable name")?;

        // Parse the numerical expression
        let initializer = self.num_expression()?;

        Ok(Statement::NumDeclaration(name, initializer, line_number))
    }

    /// Parses a string expression.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * An Expression object
    /// * A formatted error message if parsing fails
    fn expression(&mut self) -> Result<Expression, String> {
        match &self.peek().token_type {
            TokenType::StringLiteral(_) => {
                let token = self.advance();
                if let TokenType::StringLiteral(value) = &token.token_type {
                    Ok(Expression::StringLiteral(value.clone()))
                } else {
                    unreachable!()
                }
            }
            TokenType::Identifier(_) => {
                let token = self.advance();
                if let TokenType::Identifier(name) = &token.token_type {
                    Ok(Expression::Variable(name.clone()))
                } else {
                    unreachable!()
                }
            }
            _ => {
                let token = self.peek().clone();
                Err(format_error(
                    &self.source_path,
                    &self.source,
                    ErrorPosition {
                        line: token.line,
                        column: token.column,
                    },
                    "Expected expression".to_string(),
                    "Valid expressions are string literals and variable identifiers".to_string(),
                ))
            }
        }
    }

    /// Parses a numerical expression using recursive descent parsing.
    /// This handles precedence and associativity of mathematical operators.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A NumExpression object
    /// * A formatted error message if parsing fails
    fn num_expression(&mut self) -> Result<NumExpression, String> {
        // Start with the lowest precedence: addition and subtraction
        self.num_addition()
    }

    /// Parses an addition or subtraction expression.
    /// Addition and subtraction have the same precedence level.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A NumExpression object
    /// * A formatted error message if parsing fails
    fn num_addition(&mut self) -> Result<NumExpression, String> {
        // Start with the next higher precedence
        let mut expr = self.num_multiplication()?;

        // Keep consuming addition and subtraction operators
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOperator::Add,
                TokenType::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            
            // Parse the right operand with higher precedence
            let right = self.num_multiplication()?;
            
            // Build the binary operation expression
            expr = NumExpression::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses a multiplication or division expression.
    /// Multiplication and division have the same precedence level,
    /// which is higher than addition and subtraction.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A NumExpression object
    /// * A formatted error message if parsing fails
    fn num_multiplication(&mut self) -> Result<NumExpression, String> {
        // Start with the highest precedence: primary expressions
        let mut expr = self.num_primary()?;

        // Keep consuming multiplication and division operators
        while self.match_token(TokenType::Star) || self.match_token(TokenType::Slash) {
            let operator = match self.previous().token_type {
                TokenType::Star => BinaryOperator::Multiply,
                TokenType::Slash => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            
            // Parse the right operand
            let right = self.num_primary()?;
            
            // Build the binary operation expression
            expr = NumExpression::BinaryOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses a primary numerical expression (literals, variables, and parenthesized expressions).
    /// This is the highest precedence level in the expression grammar.
    ///
    /// # Returns
    ///
    /// A Result containing either:
    /// * A NumExpression object
    /// * A formatted error message if parsing fails
    fn num_primary(&mut self) -> Result<NumExpression, String> {
        // Check each possible primary expression type
        if self.match_token(TokenType::NumberLiteral(0.0)) {
            // Handle numeric literals
            if let TokenType::NumberLiteral(value) = self.previous().token_type {
                Ok(NumExpression::NumberLiteral(value))
            } else {
                unreachable!()
            }
        } else if self.match_token(TokenType::Identifier("".to_string())) {
            // Handle variable references
            if let TokenType::Identifier(name) = &self.previous().token_type {
                Ok(NumExpression::Variable(name.clone()))
            } else {
                unreachable!()
            }
        } else if self.match_token(TokenType::OpenParen) {
            // Handle parenthesized expressions
            let expr = self.num_expression()?;
            self.consume(TokenType::CloseParen, "Expected ')' after expression")?;
            Ok(NumExpression::Grouping(Box::new(expr)))
        } else {
            // Error: unexpected token
            let token = self.peek().clone();
            Err(format_error(
                &self.source_path,
                &self.source,
                ErrorPosition {
                    line: token.line,
                    column: token.column,
                },
                "Expected numerical expression".to_string(),
                "Valid expressions are numbers, variables, or parenthesized expressions".to_string(),
            ))
        }
    }
}

/// Convenience function to parse a token stream into an AST.
///
/// # Arguments
///
/// * `tokens` - The token stream to parse
///
/// # Returns
///
/// A Result containing either:
/// * A vector of Statement objects representing the program
/// * A formatted error message if parsing fails
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, String> {
    // In a real implementation, we would pass the actual source code and path
    let source = String::new(); 
    let source_path = String::new();
    let mut parser = Parser::new(tokens, source, source_path);
    parser.parse()
}