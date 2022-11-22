use std::{env, process};
use std::num::ParseFloatError;
use std::str::FromStr;

const WRONG_PARAMETER_COUNT: i32 = 1;
const CANNOT_PARSE_NUMBER: i32 = 2;
const CANNOT_PARSE_OPERATOR: i32 = 3;


fn main() {
    const REQUIRED_ARGS: usize = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() != REQUIRED_ARGS {
        eprintln!("call {} A op B", args[0]);
        process::exit(WRONG_PARAMETER_COUNT);
    }

    let Ok(a) = parse_number(&args[1]) else {
        eprintln!("cannot parse number {}", args[1]);
        process::exit(CANNOT_PARSE_NUMBER);
    };

    let Some(op) = parse_operator(&args[2]) else {
        eprintln!("Cannot parse operator {}",  args[2]);
        process::exit(CANNOT_PARSE_OPERATOR);
    };

    let Ok(b) = parse_number(&args[3]) else {
        eprintln!("cannot parse number {}", args[3]);
        process::exit(CANNOT_PARSE_NUMBER);
    };

    let r = match op {
        '+' => a + b,
        _ => panic!("This must never happen"),
    };

    println!("{} {} {} = {}", a, op, b, r);
}


fn parse_number(arg: &str) -> Result<f64, ParseFloatError> {
    f64::from_str(arg)
}

fn parse_operator(arg: &str) -> Option<char> {
    if arg.len() != 1 {
        return None;
    }

    let arg = arg.chars().nth(0).unwrap();
    if arg == '+' {
        return Some(arg);
    }

    None
}
