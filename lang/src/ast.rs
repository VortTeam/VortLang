// ast.rs - Abstract Syntax Tree for the Vortlang compiler
//
// This module defines the Abstract Syntax Tree (AST) structure that represents
// the parsed source code. The AST is the intermediate representation used
// between parsing and code generation phases.
//
// The structure closely follows the grammar of the Vortlang language, with
// nodes representing various language constructs like statements and expressions.
// This representation makes it easy to analyze and transform the code before
// generating the target output.

/// Represents a generic node in the AST. 
/// 
/// This is a convenience type that can represent either a Statement or an Expression,
/// allowing us to build a heterogeneous tree structure if needed.
#[derive(Debug, Clone)]
pub enum _Node {
    Statement(Statement),
    Expression(Expression),
}

/// Represents a statement in the Vortlang language.
/// 
/// Statements are top-level constructs that perform actions or declare variables.
/// They don't produce values directly but instead cause effects or define bindings.
#[derive(Debug, Clone)]
pub enum Statement {
    /// A print statement that outputs an expression's value to stdout.
    Print(Expression),
    
    /// A formatted print statement that supports interpolation of literals,
    /// variables, and function calls.
    PrintFormat(Vec<FormatPart>),
    
    /// A string variable declaration and assignment.
    VariableDeclaration(String, Expression, usize),
    
    /// A numerical variable declaration and assignment.
    NumDeclaration(String, NumExpression, usize),

    /// Reassignment of an existing string variable.
    VariableAssignment(String, Expression, #[allow(dead_code)] usize),

    /// Reassignment of an existing numeric variable.
    NumAssignment(String, NumExpression, #[allow(dead_code)] usize),
    
    /// Definition of a function with a name and a body of statements.
    FunctionDefinition(String, Vec<Statement>),
    
    /// A standalone call to a function.
    FunctionCall(String),
}

/// Represents a part of a formatted print statement.
/// 
/// Used in PrintFormat to represent either a literal string or an expression
/// (variable reference or function call) that appears within the format string.
#[derive(Debug, Clone)]
pub enum FormatPart {
    /// A literal string portion of the format string.
    Literal(String),
    
    /// An expression (variable or function call) to be evaluated or executed.
    Expression(Expression),
}

/// Represents an expression in the Vortlang language.
/// 
/// Expressions are constructs that can be evaluated to produce a value.
/// They can appear within statements or within other expressions.
#[derive(Debug, Clone)]
pub enum Expression {
    /// A string literal enclosed in double quotes.
    StringLiteral(String),
    
    /// A reference to a previously defined variable.
    Variable(String),
    
    /// A call to a function, used within format strings.
    FunctionCall(String),
}

/// Represents a numerical expression in the Vortlang language.
/// 
/// Numerical expressions are used specifically for mathematical operations
/// and can be nested to form complex calculations.
#[derive(Debug, Clone)]
pub enum NumExpression {
    /// A literal numerical value (integer or float).
    NumberLiteral(f64),
    
    /// A reference to a previously defined numerical variable.
    Variable(String),
    
    /// A binary operation between two numerical expressions.
    BinaryOp(Box<NumExpression>, BinaryOperator, Box<NumExpression>),
    
    /// A parenthesized numerical expression for precedence control.
    Grouping(Box<NumExpression>),
}

/// Represents binary mathematical operators in the Vortlang language.
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    /// Addition operator (+)
    Add,
    
    /// Subtraction operator (-)
    Subtract,
    
    /// Multiplication operator (*)
    Multiply,
    
    /// Division operator (/)
    Divide,
}

/// Analyzes the AST for semantic errors and optimization opportunities.
///
/// This function performs static analysis on the program to detect issues
/// like unused variables, and could be extended to implement optimizations
/// such as constant folding or dead code elimination.
///
/// # Arguments
///
/// * `ast` - A vector of Statement objects representing the program
///
/// # Returns
///
/// A tuple containing:
/// * The potentially transformed AST
/// * A vector of warning messages
pub fn analyze(ast: Vec<Statement>) -> (Vec<Statement>, Vec<String>) {
    // Use HashSet for efficient membership testing of variable usage
    let mut used_variables = std::collections::HashSet::new();
    
    // Track where variables are declared to provide precise warning locations
    let mut declared_variables = std::collections::HashMap::new();
    
    // Accumulate warnings for reporting to the user
    let mut warnings = Vec::new();

    // First pass: collect all declared variables with their positions
    // This allows us to know all variables before checking their usage
    for stmt in ast.iter() {
        match stmt {
            Statement::VariableDeclaration(name, _, line_number) => {
                // Store the actual line number from the source code for warning messages
                declared_variables.insert(name.clone(), *line_number);
            },
            Statement::NumDeclaration(name, _, line_number) => {
                // Also track numerical variable declarations with source line numbers
                declared_variables.insert(name.clone(), *line_number);
            },
            Statement::FunctionDefinition(_, body) => {
                // Recursively collect variables from function bodies since all variables are global
                for body_stmt in body {
                    match body_stmt {
                        Statement::VariableDeclaration(name, _, line_number) => {
                            declared_variables.insert(name.clone(), *line_number);
                        },
                        Statement::NumDeclaration(name, _, line_number) => {
                            declared_variables.insert(name.clone(), *line_number);
                        },
                        _ => {}
                    }
                }
            },
            _ => {}  // Skip other statement types
        }
    }

    // Second pass: find all variable usages across the program, including inside functions
    for stmt in &ast {
        match stmt {
            Statement::Print(expr) => {
                // Check for variable usage in print statements
                if let Expression::Variable(name) = expr {
                    used_variables.insert(name.clone());
                }
            },
            Statement::PrintFormat(parts) => {
                // Handle format strings which may contain variable references or function calls
                for part in parts {
                    if let FormatPart::Expression(expr) = part {
                        match expr {
                            Expression::Variable(name) => {
                                used_variables.insert(name.clone());
                            },
                            Expression::FunctionCall(_) => {
                                // Function calls don't produce values, so no variable usage to track here
                            },
                            Expression::StringLiteral(_) => {},
                        }
                    }
                }
            },
            Statement::NumDeclaration(_, expr, _) => {
                // Check for variable usage in numerical expressions
                collect_num_expr_variables(expr, &mut used_variables);
            },
            Statement::NumAssignment(_, expr, _) => {
                collect_num_expr_variables(expr, &mut used_variables);
            },
            Statement::FunctionDefinition(_, body) => {
                // Analyze function body for variable usage
                for body_stmt in body {
                    match body_stmt {
                        Statement::Print(expr) => {
                            if let Expression::Variable(name) = expr {
                                used_variables.insert(name.clone());
                            }
                        },
                        Statement::PrintFormat(parts) => {
                            for part in parts {
                                if let FormatPart::Expression(expr) = part {
                                    if let Expression::Variable(name) = expr {
                                        used_variables.insert(name.clone());
                                    }
                                }
                            }
                        },
                        Statement::NumDeclaration(_, expr, _) => {
                            collect_num_expr_variables(expr, &mut used_variables);
                        },
                        Statement::NumAssignment(_, expr, _) => {
                            collect_num_expr_variables(expr, &mut used_variables);
                        },
                        _ => {},
                    }
                }
            },
            _ => {}  // Skip other statement types
        }
    }

    // Find unused variables and generate appropriate warnings
    for (var_name, &line_number) in &declared_variables {
        if !used_variables.contains(var_name) {
            warnings.push(format!(
                "Unused variable '{}' at line {}",
                var_name, line_number
            ));
        }
    }

    // Return the AST (potentially optimized in a more advanced implementation)
    // along with any warnings that should be displayed to the user
    (ast, warnings)
}

/// Helper function to collect all variable references in a numerical expression.
///
/// Recursively traverses a numerical expression to find all variable references
/// and adds them to the provided HashSet.
///
/// # Arguments
///
/// * `expr` - The numerical expression to analyze
/// * `used_variables` - Set of used variables to update
fn collect_num_expr_variables(
    expr: &NumExpression,
    used_variables: &mut std::collections::HashSet<String>
) {
    match expr {
        NumExpression::Variable(name) => {
            // Record variable usage
            used_variables.insert(name.clone());
        },
        NumExpression::BinaryOp(left, _, right) => {
            // Recursively check both sides of binary operations
            collect_num_expr_variables(left, used_variables);
            collect_num_expr_variables(right, used_variables);
        },
        NumExpression::Grouping(inner) => {
            // Recursively check inside parenthesis groups
            collect_num_expr_variables(inner, used_variables);
        },
        NumExpression::NumberLiteral(_) => {
            // Literals don't reference variables
        },
    }
}