use crate::tokenizer::{Token, replace_operator_keywords};
use crate::variables::VariableStore;
use crate::error::VortError;

pub fn evaluate_expression(
    expr: &str,
    variables: &VariableStore,
) -> Result<f64, VortError> {
    let expr = replace_operator_keywords(expr);
    let tokens = crate::tokenizer::tokenize(&expr)?;
    let postfix = shunting_yard(tokens)?;
    evaluate_postfix(&postfix, variables)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, VortError> {
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
                op_stack.pop().ok_or(VortError::EvalError("Mismatched parentheses".into()))?;
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
            return Err(VortError::EvalError("Mismatched parentheses".into()));
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_postfix(
    tokens: &[Token],
    variables: &VariableStore,
) -> Result<f64, VortError> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Variable(var_name) => {
                match variables.get(var_name) {
                    Some(crate::variables::VariableValue::Number(n)) => stack.push(*n),
                    Some(crate::variables::VariableValue::String(_)) => 
                        return Err(VortError::EvalError(
                            format!("Variable '{}' is a string, expected number", var_name)
                        )),
                    None => return Err(VortError::EvalError(
                        format!("Undefined variable: {}", var_name)
                    )),
                }
            }
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(VortError::EvalError(
                        "Not enough operands for operator".into()
                    ));
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => return Err(VortError::EvalError(
                        format!("Unknown operator: {}", op)
                    ),
                )};
                stack.push(result);
            }
            _ => return Err(VortError::EvalError(
                "Unexpected token in postfix expression".into()
            )),
        }
    }

    if stack.len() != 1 {
        return Err(VortError::EvalError("Invalid expression".into()));
    }

    Ok(stack.pop().unwrap())
}