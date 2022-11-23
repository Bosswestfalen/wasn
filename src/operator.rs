//! Supported operators.
//!
//! The following operators are supported:
//! + Add (input: +)

use std::fmt;
use std::str::FromStr;

/// Enumeration for allowed operators.
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
    type Err = i32;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Operator::Add)
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
}
