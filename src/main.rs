use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Takes one argument but {} was given", args.len() - 1);
        return
    }
    let string =  &args[1];

    let new_string = format(&string);

    match new_string {
        Ok(s) => { println!("{}", s); },
        Err(e) => { println!("Parsing error: {}", e); }
    }
}

fn format(input: &str) -> Result<String, i32> {
    let mut s = " ".to_string();
    for c in input.chars() {
        match c {
            ' ' => break,
            _ => {
                if c.is_numeric() {
                    let n = c.to_digit(10).unwrap();
                    for _ in 0..n {
                        s.push('.');
                        s.push(' ');
                    }
                } else {
                    s.push(c);
                }
            }
        }
        s.push(' ');
    }
    s = s.replace("/", "/\n ");
    s = s.replace("  ", " ");
    s = s.replace(" (", "(");
    s = s.replace(" )", ")");
    s = s.replace(") ", ")");
    s = s.replace("( ", "(");
    s.pop();

    Ok(s)
}
