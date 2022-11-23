//! Possible errors.
//!
//! An error occurs if an input (number or operator) cannot be parsed.

use std::fmt;

const CANNOT_PARSE_NUMBER: i32 = 2;
const CANNOT_PARSE_OPERATOR: i32 = 3;

/// The Error used in wasn.
///
/// An Error can occur if a number or an operator cannot be parsed.
#[derive(Debug, Clone)]
pub struct Error {
    /// Message of the error
    message: String,
    /// Error code, can be used as exit code.
    code: i32,
}

impl Error {
    pub fn at_number(num: &str) -> Self {
        Error {
            message: format!("unable to parse number: {}", num),
            code: CANNOT_PARSE_NUMBER,
        }
    }

    pub fn at_operator(op: &str) -> Self {
        Error {
            message: format!("unable to parse operator: {}", op),
            code: CANNOT_PARSE_OPERATOR,
        }
    }

    pub fn code(&self) -> i32 {
        self.code
    }
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "[code {}] {}", self.code, self.message)
    }
}
