//! Supported operators.
//!
//! The following operators are supported:
//! + Add (input: +, plus)
//! + Minus (input: -, minus)
//! + Mul (input: *, x, times, mal)

use super::error::Error;
use super::number::Number;

use std::fmt;
use std::str::FromStr;


/// Enumeration for allowed operators.
#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    /// Add = a + b
    Add,
    /// Minus = a - b
    Minus,
    /// Mul = a * b
    Mul,
}

impl Operator {
    /// Calculate the result based on the operator.
    pub fn calc(&self, a: Number, b: Number) -> Number {
        match &self {
            Operator::Add => a + b,
            Operator::Minus => a - b,
            Operator::Mul => a * b,
        }
    }
}

impl FromStr for Operator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ["+", "plus"].contains(&s) {
            return Ok(Operator::Add);
        }
        if ["-", "minus"].contains(&s) {
            return Ok(Operator::Minus);
        }
        if ["*", "x", "times", "mal"].contains(&s) {
            return Ok(Operator::Mul);
        }
        Err(Error::at_operator(s))
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let o = match self {
            Self::Add => '+',
            Self::Minus => '-',
            Self::Mul => '*',
        };
        write!(f, "{}", o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Number = Number{value: 5.0};
    const B: Number = Number{value: 10.0};


    #[test]
    fn add_ok() {
        let r = Operator::Add.calc(A, B);
        assert_eq!(r, A + B);
    }

    #[test]
    fn minus_ok() {
        let r = Operator::Minus.calc(A, B);
        assert_eq!(r, A - B);
    }

    #[test]
    fn mul_ok() {
        let r = Operator::Mul.calc(A, B);
        assert_eq!(r, A * B);
    }

    #[test]
    fn parse_add_ok() {
        for i in ["+", "plus"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Add, op);
        }
    }

    #[test]
    fn parse_minus_ok() {
        for i in ["-", "minus"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Minus, op);
        }
    }

    #[test]
    fn parse_mul_ok() {
        for i in ["*", "x", "times", "mal"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Mul, op);
        }
    }

    #[test]
    fn parse_fail() {
        for i in ["", "/", "cat", "^"] {
            let f: Result<Operator, Error> = i.parse();
            assert!(f.is_err());
        }
    }
}
