// codegen.rs - Code generation for the Vortlang compiler
//
// This module is responsible for translating the AST into C code, which serves
// as an intermediate representation before generating the final executable.
//
// The code generator traverses the AST and emits equivalent C code for each
// language construct, handling variable declarations, assignments, expressions,
// and statements according to the language semantics.

use crate::ast::{BinaryOperator, Expression, NumExpression, Statement, FormatPart};
use std::collections::HashSet;
use std::fmt::Write;

/// Enum to differentiate between regular and C code functions during code generation
#[derive(Clone)]
enum FunctionType {
    Regular(Vec<Statement>),
    CCode(String),
}

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
    write!(
        code,
        "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\n#include <math.h>\n\n"
    ).unwrap();

    // Collect all variable declarations from the program, including inside functions
    let mut str_variables = HashSet::new();
    let mut num_variables = HashSet::new();
    collect_variables(ast, &mut str_variables, &mut num_variables);

    // Generate global variable declarations
    for var in &str_variables {
        code.push_str(&format!("char* {};\n", var));
    }
    for var in &num_variables {
        code.push_str(&format!("double {};\n", var));
    }
    code.push_str("\n");

   // Collect both regular and C code function definitions
   let mut functions = Vec::new();
   let mut main_statements = Vec::new();
   for stmt in ast {
       match stmt {
           Statement::FunctionDefinition(name, body) => {
               functions.push((name.clone(), FunctionType::Regular(body.clone())));
           }
           Statement::CFunctionDefinition(name, c_code) => {
               functions.push((name.clone(), FunctionType::CCode(c_code.clone())));
           }
           _ => {
               main_statements.push(stmt.clone());
           }
       }
   }

   // Generate function definitions
   for (name, func_type) in functions {
       match func_type {
           FunctionType::Regular(body) => {
               code.push_str(&format!("void {}(void) {{\n", name));
               for stmt in body {
                   let stmt_code = generate_statement(&stmt, &str_variables, &num_variables)?;
                   code.push_str(&stmt_code);
               }
               code.push_str("}\n\n");
           }
           FunctionType::CCode(c_code) => {
               // Directly insert the raw C code into the function body
               code.push_str(&format!("void {}(void) {{ {} }}\n\n", name, c_code));
           }
       }
   }

   code.push_str("int main() {\n");
   for stmt in main_statements {
       let stmt_code = generate_statement(&stmt, &str_variables, &num_variables)?;
       code.push_str(&stmt_code);
   }
   code.push_str("    return 0;\n");
   code.push_str("}\n");

   Ok(code)
}

/// Collects all variable declarations from the AST, including inside functions.
///
/// Since all variables are global, this traverses the entire AST to gather
/// string and numerical variable names.
///
/// # Arguments
///
/// * `statements` - The list of statements to analyze
/// * `str_vars` - Set to store string variable names
/// * `num_vars` - Set to store numerical variable names
fn collect_variables(
    statements: &[Statement],
    str_vars: &mut HashSet<String>,
    num_vars: &mut HashSet<String>,
) {
    for stmt in statements {
        match stmt {
            Statement::VariableDeclaration(name, _, _) => {
                str_vars.insert(name.clone());
            }
            Statement::NumDeclaration(name, _, _) => {
                num_vars.insert(name.clone());
            }
            Statement::FunctionDefinition(_, body) => {
                collect_variables(body, str_vars, num_vars);
            }
            _ => {}
        }
    }
}

/// Generates C code for a single statement.
///
/// # Arguments
///
/// * `stmt` - The statement to generate code for
/// * `str_vars` - Set of declared string variables
/// * `num_vars` - Set of declared numerical variables
///
/// # Returns
///
/// A Result containing either:
/// * The generated C code for the statement
/// * An error message if code generation fails
fn generate_statement(
    stmt: &Statement,
    str_vars: &HashSet<String>,
    num_vars: &HashSet<String>,
) -> Result<String, String> {
    let mut code = String::new();
    match stmt {
        Statement::VariableDeclaration(name, expr, _) => {
                        // Treat as assignment since variable is declared globally
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
                                if !str_vars.contains(var) {
                                    return Err(format!("Variable '{}' used before declaration", var));
                                }
                                code.push_str(var);
                            }
                            _ => return Err("Invalid expression for variable declaration".to_string()),
                        }
                        code.push_str(";\n");
            }
        Statement::NumDeclaration(name, expr, _) => {
                // Treat as assignment since variable is declared globally
                code.push_str("    ");
                code.push_str(name);
                code.push_str(" = ");
                let expr_code = generate_num_expression(expr, num_vars)?;
                code.push_str(&expr_code);
                code.push_str(";\n");
            }
        Statement::VariableAssignment(name, expr, _) => {
                if !str_vars.contains(name) {
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
                        if !str_vars.contains(var) {
                            return Err(format!("Variable '{}' used before declaration", var));
                        }
                        code.push_str(var);
                    }
                    _ => return Err("Invalid expression for variable assignment".to_string()),
                }
                code.push_str(";\n");
            }
        Statement::NumAssignment(name, expr, _) => {
                if !num_vars.contains(name) {
                    return Err(format!("Numerical variable '{}' assigned before declaration", name));
                }
                code.push_str("    ");
                code.push_str(name);
                code.push_str(" = ");
                let expr_code = generate_num_expression(expr, num_vars)?;
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
                    if str_vars.contains(var) {
                        code.push_str("    printf(\"%s\\n\", ");
                        code.push_str(var);
                        code.push_str(");\n");
                    } else if num_vars.contains(var) {
                        code.push_str("    printf(\"%g\\n\", ");
                        code.push_str(var);
                        code.push_str(");\n");
                    } else {
                        return Err(format!("Variable '{}' used before declaration", var));
                    }
                }
                _ => return Err("Invalid expression for print statement".to_string()),
            },
        Statement::PrintFormat(parts) => {
                // Generate separate statements for each part
                for part in parts {
                    match part {
                        FormatPart::Literal(s) => {
                            code.push_str(&format!("    printf(\"%s\", \"{}\");", escape_string(s)));
                        }
                        FormatPart::Expression(expr) => {
                            match expr {
                                Expression::Variable(name) => {
                                    if str_vars.contains(name) {
                                        code.push_str(&format!("    printf(\"%s\", {});", name));
                                    } else if num_vars.contains(name) {
                                        code.push_str(&format!("    printf(\"%g\", {});", name));
                                    } else {
                                        return Err(format!("Variable '{}' used before declaration", name));
                                    }
                                }
                                Expression::FunctionCall(name) => {
                                    code.push_str(&format!("    {}();", name));
                                }
                                _ => return Err("Invalid expression in format string".to_string()),
                            }
                        }
                    }
                }
                code.push_str("    printf(\"\\n\");\n");
            }
        Statement::FunctionCall(name) => {
                code.push_str("    ");
                code.push_str(name);
                code.push_str("();\n");
            }
            Statement::FunctionDefinition(_, _) => {
            }
        Statement::CFunctionDefinition(_, _) => todo!(),
    }
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
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            _ => result.push(c),
        }
    }
    result
}
