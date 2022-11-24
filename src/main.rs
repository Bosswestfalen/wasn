use std::{env, process};


fn main() {
    const REQUIRED_ARGS: usize = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() != REQUIRED_ARGS {
        eprintln!("call {} A op B", args[0]);
        const WRONG_PARAMETER_COUNT: i32 = 1;
        process::exit(WRONG_PARAMETER_COUNT);
    }

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(e.code());
    }
}

//parse input
//if error: print and exit

//loop
//  calculate
//  print
//  read input
//  if input empty: exit
//  else: parse input
//  use result as A, and redo loop

fn run(args: Vec<String>) -> Result<(), wasn::Error> {
    use wasn::Number;
    use wasn::Operator;

    let a: Number = args[1].parse()?;
    let op: Operator = args[2].parse()?;
    let b: Number = args[3].parse()?;
    let r = op.calc(a, b);

    println!("{} {} {} = {}", a, op, b, r);

    Ok(())
}
