fn main() {
    const REQUIRED_ARGS: usize = 4;
    let args: Vec<String> = std::env::args().collect();
    if args.len() != REQUIRED_ARGS {
        eprintln!("call {} A op B", args[0]);
        const WRONG_PARAMETER_COUNT: i32 = 1;
        std::process::exit(WRONG_PARAMETER_COUNT);
    }

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);

        use wasn::ErrorCode;
        let exit = match e.code() {
            ErrorCode::CannotParseOperator => 2,
            ErrorCode::CannotParseNumber => 3,
        };

        std::process::exit(exit);
    }
}

fn run(args: Vec<String>) -> Result<(), wasn::Error> {
    use std::io::Write;
    use wasn::number;
    use wasn::Operator;

    let mut a = number::parse(&args[1])?;
    let mut o: Operator = args[2].parse()?;
    let mut b = number::parse(&args[3])?;

    loop {
        let r = o.calc(a, b);

        print!("{} {} {} = {}\t", a, o, b, r);
        let _ = std::io::stdout().flush();

        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let parts = split_line(line.trim());
        if parts.is_none() {
            return Ok(());
        }

        a = r;
        let parts = parts.unwrap();
        o = parts.0.parse()?;
        b = number::parse(parts.1)?;
    }
}

fn split_line(line: &str) -> Option<(&str, &str)> {
    if line.is_empty() {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 2 {
        return Some((parts[0], parts[1]));
    }
    if parts.len() > 2 {
        eprintln!("Entered too many tokens");
        return None;
    }

    let num_idx = line.find(|c| matches!(c, '.' | '0'..='9'));

    let Some(num_idx) = num_idx else {
        // no number found
        return None;
    };

    if num_idx == 0 {
        // number is first entry
        return None;
    }

    Some((&line[..num_idx], &line[num_idx..]))
}
