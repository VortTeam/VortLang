use std::fmt;

#[derive(Debug)]
pub enum VortError {
    ParseError(String),
    EvalError(String),
    RuntimeError(String),
}

impl fmt::Display for VortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            Self::EvalError(msg) => write!(f, "Evaluation Error: {}", msg),
            Self::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for VortError {}