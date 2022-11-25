//! Supported operators.
//!
//! The following operators are supported:
//! + Add (input: +, plus)
//! + Minus (input: -, minus)
//! + Mul (input: *, x, times, mal)
//! + Div (input: /, :, div, durch)
//! + Mod (input: %, mod, modulo)
//! + Pow (input: ^, **, pow, hoch)

use super::error::Error;

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
    /// Div = a / b
    Div,
    /// Mod = a % b
    Mod,
    /// Pow = a ^ b
    Pow,
}

impl Operator {
    /// Calculate the result based on the operator.
    pub fn calc(&self, a: f64, b: f64) -> f64 {
        match &self {
            Operator::Add => a + b,
            Operator::Minus => a - b,
            Operator::Mul => a * b,
            Operator::Div => a / b,
            Operator::Mod => a % b,
            Operator::Pow => a.powf(b),
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
        if ["/", ":", "div", "durch"].contains(&s) {
            return Ok(Operator::Div);
        }
        if ["%", "mod", "modulo"].contains(&s) {
            return Ok(Operator::Mod);
        }
        if ["^", "**", "pow", "hoch"].contains(&s) {
            return Ok(Operator::Pow);
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
            Self::Div => '/',
            Self::Mod => '%',
            Self::Pow => '^',
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
    fn div_ok() {
        let r = Operator::Div.calc(A, B);
        assert_eq!(r, A / B);
    }

    #[test]
    fn mod_ok() {
        let r = Operator::Mod.calc(A, B);
        assert_eq!(r, A % B);
    }

    #[test]
    fn pow_ok() {
        let r = Operator::Pow.calc(A, B);
        assert_eq!(r, A.powf(B));
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
    fn parse_div_ok() {
        for i in ["/", ":", "div", "durch"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Div, op);
        }
    }

    #[test]
    fn parse_mod_ok() {
        for i in ["%", "mod", "modulo"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Mod, op);
        }
    }

    #[test]
    fn parse_pow_ok() {
        for i in ["^", "**", "pow", "hoch"] {
            let op: Operator = i.parse().unwrap();
            assert_eq!(Operator::Pow, op);
        }
    }

    #[test]
    fn parse_fail() {
        for i in ["", "cat", "_"] {
            let f: Result<Operator, Error> = i.parse();
            assert!(f.is_err());
        }
    }
}
