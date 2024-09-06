use core::fmt;

use crate::{lexer::Token, parser::Expr};

// pub type Result<T> = std::result::Result<T, ParseError>;

// error.rs
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: Token,
        found: Token,
        line: usize,
    },
    UnknownUnexpectedToken {
        found: Token,
        line: usize,
    },
    UnknownFunction {
        name: String,
        line: usize,
    },
    GeneralError {
        line: usize,
        message: String,
    },
    // Since token isnt tokenized yet
    LexerUnexpectedChar {
        found: String,
        line: usize,
    },
    ArgumentMismatch {
        expected: usize,
        found: usize,
        line: usize,
    },
    TypeError {
        expected: Expr,
        found: Expr,
        line: usize,
    },
    Other(String), // Catch-all for other types of errors
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                line,
            } => {
                write!(
                    f,
                    "on line {}: Expected token: {:?}, but found token: {:?}",
                    line, expected, found
                )
            }
            ParseError::UnknownUnexpectedToken { found, line } => {
                write!(
                    f,
                    "on line {}: found token: {:?}, but it's not expected",
                    line, found
                )
            }
            ParseError::GeneralError {
                line,
                message,
            } => {
                write!(f, "on line {}: {}", line, message)
            }
            ParseError::UnknownFunction {
                name,
                line,
            } => {
                write!(
                    f,
                    "on line {}: unknown function: {}",
                    line, name
                )
            }
            ParseError::ArgumentMismatch {
                expected,
                found,
                line,
            } => {
                write!(
                    f,
                    "on line {}: expected {} arguments, but found {}",
                    line, expected, found
                )
            },
            ParseError::TypeError {
                expected,
                found,
                line,
            } => {
                write!(
                    f,
                    "on line {}: expected type {:?}, but found type {:?}",
                    line, expected, found
                )
            },
            ParseError::LexerUnexpectedChar {
                found,
                line,
            } => {
                write!(f, "on line {}: found token {:?}, but it's not expected", line, found)
            }
            ParseError::Other(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
