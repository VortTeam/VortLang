use crate::variables::{VariableStore, VariableValue};
use crate::error::VortError;
use crate::expressions;

pub fn parse(code: &str) -> Result<(), VortError> {
    let mut variables = VariableStore::new();
    
    for line in code.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed_line.splitn(2, ';').collect();
        let statement = parts[0].trim();
        if statement.is_empty() {
            continue;
        }

        if let Some(res) = statement_parser::parse_statement(statement, &mut variables) {
            res?;
        }
    }
    
    Ok(())
}

mod statement_parser {
    use super::*;
    
    pub fn parse_statement(
        statement: &str,
        variables: &mut VariableStore,
    ) -> Option<Result<(), VortError>> {
        if statement.starts_with("let") {
            Some(handle_let(statement, variables))
        } else if statement.starts_with("num") {
            Some(handle_num(statement, variables))
        } else if statement.starts_with("print") {
            Some(handle_print(statement, variables))
        } else {
            None
        }
    }

    fn handle_let(statement: &str, variables: &mut VariableStore) -> Result<(), VortError> {
        let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
        if assignment_parts.len() != 2 {
            return Err(VortError::ParseError("Invalid variable assignment".into()));
        }
    
        let var_declaration = assignment_parts[0].trim();
        let var_name = var_declaration
            .strip_prefix("let")
            .ok_or(VortError::ParseError("Missing 'let' in declaration".into()))?
            .trim();
    
        if var_name.is_empty() {
            return Err(VortError::ParseError("Empty variable name".into()));
        }
    
        let value_str = assignment_parts[1].trim();
        let value = if value_str.starts_with('"') && value_str.ends_with('"') {
            VariableValue::String(value_str[1..value_str.len() - 1].to_string())
        } else {
            return Err(VortError::ParseError(
                "String variables must be enclosed in quotes".into()
            ));
        };
        
        variables.insert(var_name.to_string(), value)?;
        Ok(())
    }

    fn handle_num(statement: &str, variables: &mut VariableStore) -> Result<(), VortError> {
        let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
        if assignment_parts.len() != 2 {
            return Err(VortError::ParseError("Invalid variable assignment".into()));
        }
    
        let var_declaration = assignment_parts[0].trim();
        let var_name = var_declaration
            .strip_prefix("num")
            .ok_or(VortError::ParseError("Missing 'num' in declaration".into()))?
            .trim();
    
        if var_name.is_empty() {
            return Err(VortError::ParseError("Empty variable name".into()));
        }
    
        let value_str = assignment_parts[1].trim();
        let value = expressions::evaluate_expression(value_str, variables)?;
        variables.insert(var_name.to_string(), VariableValue::Number(value))?;
        Ok(())
    }

    fn handle_print(statement: &str, variables: &VariableStore) -> Result<(), VortError> {
        let print_arg = statement
            .strip_prefix("print")
            .ok_or(VortError::ParseError("Invalid print statement".into()))?
            .trim();

        if !print_arg.starts_with('(') || !print_arg.ends_with(')') {
            return Err(VortError::ParseError(
                "Print statement requires parentheses".into()
            ));
        }

        let content = print_arg[1..print_arg.len() - 1].trim();

        if let Some(formatted_content) = content.strip_prefix("o\"") {
            let formatted_content = formatted_content
                .strip_suffix('"')
                .ok_or(VortError::ParseError("Unclosed formatted string".into()))?;

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
                        None => return Err(VortError::RuntimeError(
                            format!("Undefined variable: {}", var_name)
                        )),
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
                None => return Err(VortError::RuntimeError(
                    format!("Undefined variable: {}", content)
                )),
            }
        }
        Ok(())
    }
}
