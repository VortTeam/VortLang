use std::collections::HashMap;
use std::error::Error;
use regex::Regex;

#[derive(Debug, Clone)]
enum VariableValue {
    String(String),
    Number(f64),
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(String),
    LeftParen,
    RightParen,
}

pub fn parse(code: &str) -> Result<(), Box<dyn Error>> {
    let mut variables = HashMap::<String, VariableValue>::new();
    let lines = code.lines();

    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed_line.splitn(2, ';').collect();
        let statement = parts[0].trim();
        if statement.is_empty() {
            continue;
        }

        if statement.starts_with("let") {
            let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
            if assignment_parts.len() != 2 {
                return Err("Invalid variable assignment".into());
            }

            let var_declaration = assignment_parts[0].trim();
            let var_name = var_declaration
                .strip_prefix("let")
                .ok_or("Missing 'let' in declaration")?
                .trim();

            if var_name.is_empty() {
                return Err("Empty variable name".into());
            }

            let value_str = assignment_parts[1].trim();
            let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                VariableValue::String(value_str[1..value_str.len() - 1].to_string())
            } else {
                return Err("String variables must be enclosed in quotes".into());
            };
            
            variables.insert(var_name.to_string(), value);

        } else if statement.starts_with("num") {
            let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
            if assignment_parts.len() != 2 {
                return Err("Invalid variable assignment".into());
            }

            let var_declaration = assignment_parts[0].trim();
            let var_name = var_declaration
                .strip_prefix("num")
                .ok_or("Missing 'num' in declaration")?
                .trim();

            if var_name.is_empty() {
                return Err("Empty variable name".into());
            }

            let value_str = assignment_parts[1].trim();
            let value = evaluate_expression(value_str, &variables)?;
            variables.insert(var_name.to_string(), VariableValue::Number(value));

        } else if statement.starts_with("print") {
            let print_arg = statement
                .strip_prefix("print")
                .ok_or("Invalid print statement")?
                .trim();

            if !print_arg.starts_with('(') || !print_arg.ends_with(')') {
                return Err("Print statement requires parentheses".into());
            }

            let content = print_arg[1..print_arg.len() - 1].trim();

            if let Some(formatted_content) = content.strip_prefix("o\"") {
                let formatted_content = formatted_content
                    .strip_suffix('"')
                    .ok_or("Unclosed formatted string")?;

                let mut result = String::new();
                let mut chars = formatted_content.chars().peekable();

                while let Some(c) = chars.next() {
                    if c == '{' {
                        let mut var_name = String::new();
                        while let Some(c) = chars.next() {
                            if c == '}' {
                                break;
                            }
                            var_name.push(c);
                        }
                        let var_name = var_name.trim();
                        match variables.get(var_name) {
                            Some(VariableValue::String(s)) => result.push_str(s),
                            Some(VariableValue::Number(n)) => result.push_str(&n.to_string()),
                            None => return Err(format!("Undefined variable: {}", var_name).into()),
                        }
                    } else {
                        result.push(c);
                    }
                }

                println!("{}", result);
            } else if content.starts_with('"') && content.ends_with('"') {
                println!("{}", &content[1..content.len() - 1]);
            } else {
                match variables.get(content) {
                    Some(VariableValue::String(s)) => println!("{}", s),
                    Some(VariableValue::Number(n)) => println!("{}", n),
                    None => return Err(format!("Undefined variable: {}", content).into()),
                }
            }
        } else {
            return Err(format!("Unknown command: {}", statement).into());
        }
    }

    Ok(())
}

fn replace_operator_keywords(expr: &str) -> String {
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

fn tokenize(expr: &str) -> Result<Vec<Token>, Box<dyn Error>> {
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
            let num = num_str.replace('_', "").parse::<f64>()?;
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
            return Err(format!("Unexpected character: {}", c).into());
        }
    }

    Ok(tokens)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut output = Vec::new();
    let mut op_stack = Vec::new();

    fn precedence(op: &str) -> u8 {
        match op {
            "+" | "-" => 2,
            "*" | "/" => 3,
            _ => 0,
        }
    }

    for token in tokens {
        match token {
            Token::Number(_) | Token::Variable(_) => output.push(token),
            Token::LeftParen => op_stack.push(token),
            Token::RightParen => {
                while let Some(top) = op_stack.last() {
                    if matches!(top, Token::LeftParen) {
                        break;
                    }
                    output.push(op_stack.pop().unwrap());
                }
                op_stack.pop().ok_or("Mismatched parentheses")?;
            }
            Token::Operator(op) => {
                while let Some(Token::Operator(stack_op)) = op_stack.last() {
                    if precedence(&op) <= precedence(stack_op) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(Token::Operator(op));
            }
        }
    }

    while let Some(op) = op_stack.pop() {
        if matches!(op, Token::LeftParen | Token::RightParen) {
            return Err("Mismatched parentheses".into());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_postfix(tokens: &[Token], variables: &HashMap<String, VariableValue>) -> Result<f64, Box<dyn Error>> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Variable(var_name) => {
                match variables.get(var_name) {
                    Some(VariableValue::Number(n)) => stack.push(*n),
                    Some(VariableValue::String(_)) => return Err(format!("Variable '{}' is a string, expected number", var_name).into()),
                    None => return Err(format!("Undefined variable: {}", var_name).into()),
                }
            }
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Not enough operands for operator".into());
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => return Err(format!("Unknown operator: {}", op).into()),
                };
                stack.push(result);
            }
            _ => return Err("Unexpected token in postfix expression".into()),
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression".into());
    }

    Ok(stack.pop().unwrap())
}

fn evaluate_expression(
    expr: &str,
    variables: &HashMap<String, VariableValue>,
) -> Result<f64, Box<dyn Error>> {
    let expr = replace_operator_keywords(expr);
    let tokens = tokenize(&expr)?;
    let postfix = shunting_yard(tokens)?;
    evaluate_postfix(&postfix, variables)
}
