use core::fmt;

use crate::lexer::Token;

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
    // UnexpectedToken {
    //     found: Token,
    //     line: usize,
    //     message: String,
    // },
    // UnknownFunction {
    //     name: String,
    //     line: usize,
    //     message: String,
    // },
    GeneralError {
        // name: String,
        line: usize,
        message: String,
    },
    Other(String), // Catch-all for other types of errors
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, line} => {
                write!(
                    f,
                    "on line {}: Expected token: {:?}, but found token: {:?}",
                    line, expected, found
                )
            }
            ParseError::UnknownUnexpectedToken { found, line} => {
                write!(
                    f,
                    "on line {}: found token: {:?}, but it's not expected",
                    line, found
                )
            }
            ParseError::GeneralError {  line, message } => {
                write!(
                    f,
                    "on line {}: {}",
                    line, message
                )
            }
            // ParseError::UnknownFunction { name, line, message } => {
            //     write!(
            //         f,
            //         "on line {}: {}\nUnknown function: {}",
            //         line, message, name
            //     )
            // }
            ParseError::Other(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}