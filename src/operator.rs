//! Supported operators.
//!
//! The following operators are supported:
//! + Add (input: +, plus)

use super::error::Error;

use std::fmt;
use std::str::FromStr;

/// Enumeration for allowed operators.
#[derive(Debug, PartialEq)]
pub enum Operator {
    /// Add = a + b
    Add,
}

impl Operator {
    /// Calculate the result based on the operator.
    ///
    /// ```
    /// let add = wasn::Operator::Add;
    /// let r = add.calc(1.0, 2.5);
    /// assert_eq!(3.5, r);
    /// ```
    pub fn calc(&self, a: f64, b: f64) -> f64 {
        match &self {
            Operator::Add => a + b,
        }
    }
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ["+", "plus"].contains(&s) {
            return Ok(Operator::Add);
        }
        Err(Error::at_operator(s))
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let o = match self {
            Self::Add => '+',
        };
        write!(f, "{}", o)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
        
    const A: f64 = 5.0;
    const B: f64 = 10.0;

    #[test]
    fn add_ok() {
        let r = Operator::Add.calc(A, B);
        assert_eq!(r, A + B);
    }

    #[test]
    fn parse_add_ok() {
        for i in ["+", "plus"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Add, op);
        }
    }

    #[test]
    fn parse_fail() {
        for i in ["-", "*", "/", "cat", "^"] {
            let f: Result<Operator, Error> = i.parse();
            assert!(f.is_err());
        }
    }
}
