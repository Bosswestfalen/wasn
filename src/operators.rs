//! Supported operators and helper functions.
//!
//! The following operators are supported:
//!     + (add)


/// Parses a string to get the operator.
///
/// ```
/// let add = String::from("+");
/// assert_eq!(wasn::operators::parse(&add), Some('+'));
/// ```
///
/// **Todo:** Implement missing operators.
pub fn parse(arg: &str) -> Option<char> {
    if arg.len() != 1 {
        return None;
    }

    let arg = arg.chars().nth(0).unwrap();
    if arg == '+' {
        return Some(arg);
    }

    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plus_is_ok() {
        for op in ["+"] {
            let r = parse(&op);
            assert!(r.is_some());
            assert_eq!(r.unwrap(), op.chars().nth(0).unwrap());
        }
    }
}
