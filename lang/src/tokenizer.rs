use regex::Regex;
use crate::error::VortError;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Variable(String),
    Operator(String),
    LeftParen,
    RightParen,
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, VortError> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c.is_ascii_digit() || c == '.' {
            let mut num_str = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() || c == '.' || c == '_' {
                    num_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let num = num_str.replace('_', "").parse::<f64>()
                .map_err(|e| VortError::ParseError(e.to_string()))?;
            tokens.push(Token::Number(num));
        } else if c.is_alphabetic() || c == '_' {
            let mut var_str = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphanumeric() || c == '_' {
                    var_str.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Variable(var_str));
        } else if "+-*/".contains(c) {
            tokens.push(Token::Operator(c.to_string()));
            chars.next();
        } else if c == '(' {
            tokens.push(Token::LeftParen);
            chars.next();
        } else if c == ')' {
            tokens.push(Token::RightParen);
            chars.next();
        } else {
            return Err(VortError::ParseError(format!("Unexpected character: {}", c)));
        }
    }

    Ok(tokens)
}

pub fn replace_operator_keywords(expr: &str) -> String {
    let re = Regex::new(r"\b(plus|minus|times|multiply|divide)\b").unwrap();
    re.replace_all(expr, |caps: &regex::Captures| {
        match &caps[1] {
            "plus" => "+",
            "minus" => "-",
            "times" | "multiply" => "*",
            "divide" => "/",
            _ => unreachable!(),
        }
    }).into_owned()
}