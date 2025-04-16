use std::fmt;

#[derive(Debug)]
pub struct SourceSpan {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl SourceSpan {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self { line, column, length }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum VortErrorKind {
    // Parsing errors
    UnexpectedToken(String),
    MissingToken(String),
    InvalidSyntax(String),
    
    // Evaluation errors
    UndefinedVariable(String),
    TypeMismatch(String),
    DivisionByZero,
    MismatchedParentheses,
    
    // Runtime errors
    InvalidOperation(String),
    VariableRedefinition(String),
    TypeConversionError(String),
    
    // Other errors
    IOError(std::io::Error),
    Internal(String),
}

#[derive(Debug)]
pub struct VortError {
    pub kind: VortErrorKind,
    pub message: String,
    pub span: Option<SourceSpan>,
}

impl VortError {
    pub fn new(kind: VortErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            span: None,
        }
    }
    
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }
    
    pub fn parse_error(message: impl Into<String>) -> Self {
        let message_str = message.into();
        Self::new(
            VortErrorKind::InvalidSyntax(message_str.clone()),
            format!("Parse Error: {}", message_str)
        )
    }
    
    pub fn eval_error(message: impl Into<String>) -> Self {
        let message_str = message.into();
        Self::new(
            VortErrorKind::InvalidOperation(message_str.clone()),
            format!("Evaluation Error: {}", message_str)
        )
    }
    
    pub fn runtime_error(message: impl Into<String>) -> Self {
        let message_str = message.into();
        Self::new(
            VortErrorKind::Internal(message_str.clone()),
            format!("Runtime Error: {}", message_str)
        )
    }
    
    pub fn undefined_variable(name: impl Into<String>) -> Self {
        let name = name.into();
        Self::new(
            VortErrorKind::UndefinedVariable(name.clone()),
            format!("Undefined variable: '{}'", name)
        )
    }
    
    pub fn type_mismatch(expected: &str, got: &str, context: &str) -> Self {
        Self::new(
            VortErrorKind::TypeMismatch(format!("Expected {}, got {}", expected, got)),
            format!("Type mismatch in {}: expected {}, got {}", context, expected, got)
        )
    }
}

impl fmt::Display for VortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(f, "[Line {}, Column {}] {}", span.line, span.column, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for VortError {}

impl From<String> for VortError {
    fn from(s: String) -> Self {
        VortError::runtime_error(s)
    }
}

impl From<std::io::Error> for VortError {
    fn from(err: std::io::Error) -> Self {
        let error_message = format!("IO Error: {}", err);
        VortError::new(
            VortErrorKind::IOError(err),
            error_message
        )
    }
}

pub mod context {
    use super::*;
    use std::cell::RefCell;
    
    thread_local! {
        static CURRENT_LINE: RefCell<usize> = RefCell::new(1);
    }
    
    pub fn set_current_line(line: usize) {
        CURRENT_LINE.with(|current_line| {
            *current_line.borrow_mut() = line;
        });
    }

    #[allow(dead_code)]
    pub fn get_current_line() -> usize {
        CURRENT_LINE.with(|current_line| {
            *current_line.borrow()
        })
    }
    
    pub fn with_span(error: VortError, line: usize, column: usize, length: usize) -> VortError {
        error.with_span(SourceSpan::new(line, column, length))
    }
}
