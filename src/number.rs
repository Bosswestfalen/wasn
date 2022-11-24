//! Representation of a number.
//!
//! A number is a f64, but this module is used
//! to offer some extra functionality.

use super::error::Error;

use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

/// A number contains its numerical value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Number {
    #[cfg(test)] //better way?
    pub value: f64,

    #[cfg(not(test))] //better way?
    value: f64,
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Ok(value) = f64::from_str(s) else {
            return Err(Error::at_number(s));
        };

        Ok(Number { value })
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let value = self.value + rhs.value;
        Number { value }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let value = self.value - rhs.value;
        Number { value }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = self.value * rhs.value;
        Number { value }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let value = self.value / rhs.value;
        Number { value }
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let value = self.value % rhs.value;
        Number { value }
    }
}

impl Number {
    /// Calculate A raised by the power of B.
    ///
    /// **Note:** no overflow check is performed.
    pub fn pow(self, e: Self) -> Self {
        let value = self.value.powf(e.value);
        Number { value }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let n: Result<Number, Error> = "1.23".parse();
        assert!(n.is_ok());
        assert_eq!(n.unwrap().value, 1.23);
    }

    #[test]
    fn parse_fail() {
        for s in ["", "cat", "1,3"] {
            let n: Result<Number, Error> = s.parse();
            assert!(n.is_err());
        }
    }

    const A: f64 = 10.5;
    const B: f64 = 3.5;

    #[test]
    fn test_add() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a + b;
        assert_eq!(r.value, A + B);
    }

    #[test]
    fn test_minus() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a - b;
        assert_eq!(r.value, A - B);
    }

    #[test]
    fn test_mul() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a * b;
        assert_eq!(r.value, A * B);
    }

    #[test]
    fn test_div() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a / b;
        assert_eq!(r.value, A / B);
    }

    #[test]
    fn test_mod() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a % b;
        assert_eq!(r.value, A % B);
    }

    #[test]
    fn test_pow() {
        let a = Number { value: A };
        let b = Number { value: B };
        let r = a.pow(b);
        assert_eq!(r.value, A.powf(B));
    }
}
