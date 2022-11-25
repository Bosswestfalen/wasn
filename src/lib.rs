//! The wasn lib implements functionality for a simple calculator.

pub mod error;
pub mod operator;

pub use error::Error;
pub use error::ErrorCode;
pub use operator::Operator;

/// Number parsing
pub mod number {
    /// Parse a string to a number, allowing both: . (English) and , (German).
    ///
    /// ```
    /// let e = wasn::number::parse("1.2").unwrap();
    /// let g = wasn::number::parse("1,2").unwrap();
    /// assert_eq!(e, g);
    /// ```
    pub fn parse(s: &str) -> Result<f64, crate::error::Error> {
        let x = s.replacen(',', ".", 1);

        use std::str::FromStr;
        let Ok(value) = f64::from_str(&x) else {
            return Err(crate::error::Error::at_number(s));
        };

        Ok(value)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_ok() {
            let n = parse("1.23");
            assert!(n.is_ok());
            assert_eq!(n.unwrap(), 1.23);
        }

        #[test]
        fn parse_fail() {
            for s in ["", "cat", "1_23"] {
                let n = parse(s);
                assert!(n.is_err());
            }
        }
    }
}
