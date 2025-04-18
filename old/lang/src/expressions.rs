use crate::tokenizer::{Token, replace_operator_keywords};
use crate::variables::VariableStore;
use crate::error::{VortError, VortErrorKind};

pub fn evaluate_expression(
    expr: &str,
    variables: &VariableStore,
) -> Result<f64, VortError> {
    let expr = replace_operator_keywords(expr);
    let tokens = crate::tokenizer::tokenize(&expr)
        .map_err(|e| VortError::eval_error(format!("Failed to tokenize expression: {}", e)))?;
    
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
                let mut found_matching_paren = false;
                while let Some(top) = op_stack.last() {
                    if matches!(top, Token::LeftParen) {
                        found_matching_paren = true;
                        break;
                    }
                    output.push(op_stack.pop().unwrap());
                }
                
                if !found_matching_paren {
                    return Err(VortError::new(
                        VortErrorKind::MismatchedParentheses,
                        "Mismatched parentheses: extra closing parenthesis".to_string()
                    ));
                }
                
                op_stack.pop();
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

    for op in op_stack.iter() {
        if matches!(op, Token::LeftParen) {
            return Err(VortError::new(
                VortErrorKind::MismatchedParentheses,
                "Mismatched parentheses: unclosed opening parenthesis".to_string()
            ));
        }
    }

    while let Some(op) = op_stack.pop() {
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
                        return Err(VortError::type_mismatch(
                            "number", "string", 
                            &format!("Variable '{}'", var_name)
                        )),
                    None => return Err(VortError::undefined_variable(var_name)),
                }
            }
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(VortError::eval_error(
                        format!("Not enough operands for operator '{}'", op)
                    ));
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match op.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0.0 {
                            return Err(VortError::new(
                                VortErrorKind::DivisionByZero,
                                "Division by zero".to_string()
                            ));
                        }
                        a / b
                    },
                    _ => return Err(VortError::eval_error(
                        format!("Unknown operator: {}", op)
                    )),
                };
                stack.push(result);
            }
            _ => return Err(VortError::eval_error(
                "Unexpected token in postfix expression".to_string()
            )),
        }
    }

    if stack.len() != 1 {
        return Err(VortError::eval_error("Invalid expression: too many values".to_string()));
    }

    Ok(stack.pop().unwrap())
}
