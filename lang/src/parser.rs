use crate::variables::{VariableStore, VariableValue};
use crate::error::{VortError, context};
use crate::expressions;

pub fn parse(code: &str) -> Result<(), VortError> {
    let mut variables = VariableStore::new();
    
    for (line_num, line) in code.lines().enumerate() {
        let line_num = line_num + 1; // 1-based line numbering
        context::set_current_line(line_num);
        
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue; // Skip comments and empty lines
        }

        // Split on '#' to remove inline comments
        let line_parts: Vec<&str> = trimmed_line.splitn(2, '#').collect();
        let line_without_comment = line_parts[0].trim();
        
        if line_without_comment.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line_without_comment.splitn(2, ';').collect();
        let statement = parts[0].trim();
        if statement.is_empty() {
            continue;
        }

        if let Some(res) = statement_parser::parse_statement(statement, &mut variables, line_num) {
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
        line_num: usize,
    ) -> Option<Result<(), VortError>> {
        if statement.starts_with("let") {
            Some(handle_let(statement, variables, line_num))
        } else if statement.starts_with("num") {
            Some(handle_num(statement, variables, line_num))
        } else if statement.starts_with("print") {
            Some(handle_print(statement, variables, line_num))
        } else {
            // Unknown statement type - return a descriptive error
            Some(Err(context::with_span(
                VortError::parse_error(format!("Unknown statement type: '{}'", statement.split_whitespace().next().unwrap_or(""))),
                line_num,
                1, 
                statement.len()
            )))
        }
    }

    fn handle_let(statement: &str, variables: &mut VariableStore, line_num: usize) -> Result<(), VortError> {
        let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
        if assignment_parts.len() != 2 {
            return Err(context::with_span(
                VortError::parse_error("Invalid variable assignment, missing '='"),
                line_num,
                1,
                statement.len()
            ));
        }
    
        let var_declaration = assignment_parts[0].trim();
        let var_name = match var_declaration.strip_prefix("let") {
            Some(name) => name.trim(),
            None => return Err(context::with_span(
                VortError::parse_error("Missing 'let' in declaration"),
                line_num, 
                1,
                var_declaration.len()
            ))
        };
    
        if var_name.is_empty() {
            return Err(context::with_span(
                VortError::parse_error("Empty variable name"),
                line_num,
                statement.find("let").unwrap_or(0) + 3, // Position after 'let'
                1
            ));
        }
    
        let value_str = assignment_parts[1].trim();
        if !value_str.starts_with('"') || !value_str.ends_with('"') {
            return Err(context::with_span(
                VortError::parse_error("String variables must be enclosed in quotes"),
                line_num,
                statement.find('=').unwrap_or(0) + 1,
                value_str.len()
            ));
        }
        
        let string_value = value_str[1..value_str.len() - 1].to_string();
        variables.insert(var_name.to_string(), VariableValue::String(string_value))
            .map_err(|e| context::with_span(e, line_num, 1, statement.len()))?;
        
        Ok(())
    }

    fn handle_num(statement: &str, variables: &mut VariableStore, line_num: usize) -> Result<(), VortError> {
        let assignment_parts: Vec<&str> = statement.splitn(2, '=').collect();
        if assignment_parts.len() != 2 {
            return Err(context::with_span(
                VortError::parse_error("Invalid variable assignment, missing '='"),
                line_num,
                1,
                statement.len()
            ));
        }
    
        let var_declaration = assignment_parts[0].trim();
        let var_name = match var_declaration.strip_prefix("num") {
            Some(name) => name.trim(),
            None => return Err(context::with_span(
                VortError::parse_error("Missing 'num' in declaration"),
                line_num,
                1,
                var_declaration.len()
            ))
        };
    
        if var_name.is_empty() {
            return Err(context::with_span(
                VortError::parse_error("Empty variable name"),
                line_num,
                statement.find("num").unwrap_or(0) + 3, // Position after 'num'
                1
            ));
        }
    
        let value_str = assignment_parts[1].trim();
        let value = expressions::evaluate_expression(value_str, variables)
            .map_err(|e| context::with_span(e, line_num, statement.find('=').unwrap_or(0) + 1, value_str.len()))?;
            
        variables.insert(var_name.to_string(), VariableValue::Number(value))
            .map_err(|e| context::with_span(e, line_num, 1, statement.len()))?;
            
        Ok(())
    }

    fn handle_print(statement: &str, variables: &VariableStore, line_num: usize) -> Result<(), VortError> {
        let print_arg = match statement.strip_prefix("print") {
            Some(arg) => arg.trim(),
            None => return Err(context::with_span(
                VortError::parse_error("Invalid print statement"),
                line_num,
                1,
                statement.len()
            ))
        };

        if !print_arg.starts_with('(') || !print_arg.ends_with(')') {
            return Err(context::with_span(
                VortError::parse_error("Print statement requires parentheses"),
                line_num,
                statement.find("print").unwrap_or(0) + 5, // Position after 'print'
                print_arg.len()
            ));
        }

        let content = print_arg[1..print_arg.len() - 1].trim();

        if let Some(formatted_content) = content.strip_prefix("o\"") {
            let formatted_content = match formatted_content.strip_suffix('"') {
                Some(content) => content,
                None => return Err(context::with_span(
                    VortError::parse_error("Unclosed formatted string"),
                    line_num,
                    statement.find("o\"").unwrap_or(0) + 2, // Position after 'o"'
                    formatted_content.len()
                ))
            };

            let mut result = String::new();
            let mut chars = formatted_content.chars().peekable();
            let mut current_pos = 0;

            while let Some(c) = chars.next() {
                current_pos += 1;
                if c == '{' {
                    let _start_var_pos = current_pos; // Changed from start_var_pos to _start_var_pos
                    let mut var_name = String::new();
                    while let Some(c) = chars.next() {
                        current_pos += 1;
                        if c == '}' {
                            break;
                        }
                        var_name.push(c);
                    }
                    let var_name = var_name.trim();
                    match variables.get(var_name) {
                        Some(VariableValue::String(s)) => result.push_str(s),
                        Some(VariableValue::Number(n)) => result.push_str(&n.to_string()),
                        None => return Err(context::with_span(
                            VortError::undefined_variable(var_name),
                            line_num,
                            statement.find(var_name).unwrap_or(0),
                            var_name.len()
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
                None => return Err(context::with_span(
                    VortError::undefined_variable(content),
                    line_num,
                    statement.find(content).unwrap_or(0),
                    content.len()
                )),
            }
        }
        Ok(())
    }
}
