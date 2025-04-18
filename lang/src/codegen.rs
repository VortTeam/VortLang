// codegen.rs - Code generation for the Vortlang compiler
//
// This module is responsible for translating the AST into C code, which serves
// as an intermediate representation before generating the final executable.
//
// The code generator traverses the AST and emits equivalent C code for each
// language construct, handling variable declarations, assignments, expressions,
// and statements according to the language semantics.

use crate::ast::{BinaryOperator, Expression, NumExpression, Statement};
use std::collections::HashSet;

/// Generates C code from the AST.
///
/// This function traverses the AST and generates equivalent C code that
/// preserves the semantics of the Vortlang program.
///
/// # Arguments
///
/// * `ast` - A slice of Statement objects representing the program
///
/// # Returns
///
/// A Result containing either:
/// * The generated C code as a String
/// * An error message if code generation fails
pub fn generate_c_code(ast: &[Statement]) -> Result<String, String> {
    let mut code = String::new();

    // Add standard includes required for the generated code
    code.push_str("#include <stdio.h>\n");
    code.push_str("#include <stdlib.h>\n");
    code.push_str("#include <string.h>\n");
    code.push_str("#include <math.h>\n\n");  // For numerical operations

    // Start main function
    code.push_str("int main() {\n");

    // Track declared variables to prevent redeclaration and ensure proper usage
    let mut str_variables = HashSet::new();
    let mut num_variables = HashSet::new();

    // Generate code for each statement
    for statement in ast {
        match statement {
            Statement::VariableDeclaration(name, expr, _line_number) => {
                // Check for variable redeclaration
                if str_variables.contains(name) || num_variables.contains(name) {
                    return Err(format!("Variable '{}' already declared", name));
                }

                // String variable declaration
                code.push_str("    char* ");
                code.push_str(name);
                code.push_str(" = ");

                match expr {
                    Expression::StringLiteral(value) => {
                        code.push_str("\"");
                        code.push_str(&escape_string(value));
                        code.push_str("\"");
                    }
                    Expression::Variable(var) => {
                        if !str_variables.contains(var) {
                            return Err(format!("Variable '{}' used before declaration", var));
                        }
                        code.push_str(var);
                    }
                }

                code.push_str(";\n");
                str_variables.insert(name.clone());
            }
            Statement::NumDeclaration(name, expr, _line_number) => {
                // Check for variable redeclaration
                if str_variables.contains(name) || num_variables.contains(name) {
                    return Err(format!("Variable '{}' already declared", name));
                }

                // Numerical variable declaration
                code.push_str("    double ");
                code.push_str(name);
                code.push_str(" = ");

                // Generate code for the numerical expression
                let expr_code = generate_num_expression(expr, &num_variables)?;
                code.push_str(&expr_code);

                code.push_str(";\n");
                num_variables.insert(name.clone());
            }
            Statement::VariableAssignment(name, expr, _line_number) => {
                // Check if the target variable is declared
                if !str_variables.contains(name) {
                    return Err(format!("Variable '{}' assigned before declaration", name));
                }
                code.push_str("    ");
                code.push_str(name);
                code.push_str(" = ");
                match expr {
                    Expression::StringLiteral(value) => {
                        code.push_str("\"");
                        code.push_str(&escape_string(value));
                        code.push_str("\"");
                    }
                    Expression::Variable(var) => {
                        if !str_variables.contains(var) {
                            return Err(format!("Variable '{}' used before declaration", var));
                        }
                        code.push_str(var);
                    }
                }
                code.push_str(";\n");
            }
            Statement::NumAssignment(name, expr, _line_number) => {
                // Check if the target variable is declared
                if !num_variables.contains(name) {
                    return Err(format!("Numerical variable '{}' assigned before declaration", name));
                }
                code.push_str("    ");
                code.push_str(name);
                code.push_str(" = ");
                let expr_code = generate_num_expression(expr, &num_variables)?;
                code.push_str(&expr_code);
                code.push_str(";\n");
            }
            Statement::Print(expr) => match expr {
                Expression::StringLiteral(value) => {
                    code.push_str("    printf(\"%s\\n\", \"");
                    code.push_str(&escape_string(value));
                    code.push_str("\");\n");
                }
                Expression::Variable(var) => {
                    if str_variables.contains(var) {
                        // String variable
                        code.push_str("    printf(\"%s\\n\", ");
                        code.push_str(var);
                        code.push_str(");\n");
                    } else if num_variables.contains(var) {
                        // Numerical variable
                        code.push_str("    printf(\"%g\\n\", ");
                        code.push_str(var);
                        code.push_str(");\n");
                    } else {
                        return Err(format!("Variable '{}' used before declaration", var));
                    }
                }
            },
            Statement::PrintFormat(expr) => {
                if let Expression::StringLiteral(value) = expr {
                    // Process format string with variable interpolation
                    let mut result = String::new();
                    let mut i = 0;
                    let chars: Vec<char> = value.chars().collect();
                    let mut format_parts = Vec::new();
                    let mut variables_to_print = Vec::new();

                    while i < chars.len() {
                        if chars[i] == '{' {
                            let start = i;
                            i += 1;
                            let mut var_name = String::new();

                            while i < chars.len() && chars[i] != '}' {
                                var_name.push(chars[i]);
                                i += 1;
                            }

                            if i < chars.len() && chars[i] == '}' {
                                // Valid variable reference found
                                if str_variables.contains(&var_name) {
                                    result.push_str("%s");
                                } else if num_variables.contains(&var_name) {
                                    result.push_str("%g");
                                } else {
                                    return Err(format!(
                                        "Variable '{}' used in format string before declaration",
                                        var_name
                                    ));
                                }

                                format_parts.push(result.clone());
                                result.clear();
                                variables_to_print.push(var_name);
                                i += 1;
                            } else {
                                // Unclosed brace, treat as literal
                                for j in start..i {
                                    result.push(chars[j]);
                                }
                            }
                        } else {
                            result.push(chars[i]);
                            i += 1;
                        }
                    }

                    if !result.is_empty() {
                        format_parts.push(result);
                    }

                    // Generate printf call
                    code.push_str("    printf(\"");
                    for part in &format_parts {
                        code.push_str(&escape_string(part));
                    }
                    code.push_str("\\n\"");

                    for var in &variables_to_print {
                        code.push_str(", ");
                        code.push_str(var);
                    }

                    code.push_str(");\n");
                }
            }
        }
    }

    // End main function
    code.push_str("    return 0;\n");
    code.push_str("}\n");

    Ok(code)
}

/// Generates C code for a numerical expression.
///
/// # Arguments
///
/// * `expr` - The numerical expression to generate code for
/// * `variables` - Set of declared numerical variables
///
/// # Returns
///
/// A Result containing either:
/// * The generated C code for the expression
/// * An error message if code generation fails
fn generate_num_expression(
    expr: &NumExpression,
    variables: &HashSet<String>,
) -> Result<String, String> {
    match expr {
        NumExpression::NumberLiteral(value) => {
            // Format the number with enough precision
            Ok(format!("{}", value))
        }
        NumExpression::Variable(name) => {
            if variables.contains(name) {
                Ok(name.clone())
            } else {
                Err(format!("Numerical variable '{}' used before declaration", name))
            }
        }
        NumExpression::BinaryOp(left, op, right) => {
            // Generate code for the left and right operands
            let left_code = generate_num_expression(left, variables)?;
            let right_code = generate_num_expression(right, variables)?;

            // Apply the operator
            let operator = match op {
                BinaryOperator::Add => "+",
                BinaryOperator::Subtract => "-",
                BinaryOperator::Multiply => "*",
                BinaryOperator::Divide => "/",
            };

            // Wrap in parentheses to preserve operator precedence
            Ok(format!("({}{}{})", left_code, operator, right_code))
        }
        NumExpression::Grouping(inner) => {
            // Generate code for the inner expression with parentheses
            let inner_code = generate_num_expression(inner, variables)?;
            Ok(format!("({})", inner_code))
        }
    }
}

/// Escapes special characters in strings for C string literals.
///
/// # Arguments
///
/// * `s` - The string to escape
///
/// # Returns
///
/// The escaped string
fn escape_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .replace("\r", "\\r")
}