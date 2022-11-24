//! Possible errors.
//!
//! An error occurs if an input (number or operator) cannot be parsed.

use std::fmt;

/// Possible causes of an Error.
#[derive(Clone, Debug, Copy)]
pub enum ErrorCode {
    /// number cannot be parsed
    CannotParseNumber,

    /// operator cannot be parsed
    CannotParseOperator,
}

/// The Error used in wasn.
///
/// See ErrorCodes for possible reasons.
#[derive(Debug, Clone)]
pub struct Error {
    /// Message of the error
    message: String,
    /// Identifies the cause of the error.
    ec: ErrorCode,
}

impl Error {
    /// Generates a new Error when a number cannot be parsed.
    pub fn at_number(num: &str) -> Self {
        Error {
            message: format!("unable to parse number: {}", num),
            ec: ErrorCode::CannotParseNumber,
        }
    }

    /// Generates a new Error when the operator cannot be parsed.
    pub fn at_operator(op: &str) -> Self {
        Error {
            message: format!("unable to parse operator: {}", op),
            ec: ErrorCode::CannotParseOperator,
        }
    }

    /// Get the error code.
    pub fn code(&self) -> ErrorCode {
        self.ec
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
