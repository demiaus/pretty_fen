use std::env;

// TODO: error handling
// TODO: input validation
// TODO: help
// TODO: Flip board

fn main() {
    let args: Vec<String> = env::args().collect();

    // Print default position if no arguments
    if args.len() == 1 {
        println!(" r  n  b  q  k  b  n  r  /");
        println!(" p  p  p  p  p  p  p  p  /");
        println!(" .  .  .  .  .  .  .  .  /");
        println!(" .  .  .  .  .  .  .  .  /");
        println!(" .  .  .  .  .  .  .  .  /");
        println!(" .  .  .  .  .  .  .  .  /");
        println!(" P  P  P  P  P  P  P  P  /");
        println!(" R  N  B  Q  K  B  N  R  ");
    }

    // Handle inputs of one argument
    else if args.len() == 2 {

        // First check for help arguments
        if args[1] == "h"
        || args[1] == "--help"
        || args[1] == "help" {
            println!("help!");
            todo!("help");
        }

        // Check that input is a valid coordinate
        if args[1].len() == 2 && valid_coord(&args[1]) {

            let coordinates = args[1..].to_vec();
            let mut indices: Vec<(u8, u8)> = Vec::new();
            for c in coordinates {
                indices.extend(parse_an(&c));
            }
            print_range(&indices);

        // Second argument is a fen string and no other arguments
        } else {
            let fen =  &args[1];
            let pretty_fen = format(&fen);
            match pretty_fen {
                Ok(s) => { println!("{}", s); },
                Err(e) => { println!("Parsing error: {}", e); }
            }
        }

    // Two arguments
    } else if args.len() == 3 {

        if args[1] == "r"
        || args[1] == "--revert"
        || args[1] == "revert" {
            println!("Reverting!");
            println!("{}", revert(&args[2]));
        }

    } else {

        let coordinates = args[2..].to_vec();
        let mut indices: Vec<(u8, u8)> = Vec::new();

        for c in coordinates {
            indices.extend(parse_an(&c));
        }
        print_range(&indices);
    }
}

fn revert(s: &str) -> String {

    let mut r = s.replace(&[' ', '(', ')', '\n',], "").to_string();
    r = r.replace("........", "8");
    r = r.replace(".......", "7");
    r = r.replace("......", "6");
    r = r.replace(".....", "5");
    r = r.replace("....", "4");
    r = r.replace("...", "3");
    r = r.replace("..", "2");
    r = r.replace(".", "1");
    r
}

fn valid_coord(c: &str) -> bool {
    c.len() == 2
        && valid_file(c.chars().nth(0).unwrap())
        && valid_rank(c.chars().nth(1).unwrap())
}

fn valid_file(f: char) -> bool {
    match f {
        'a'..='h' => true,
        _ => false
    }
}

fn valid_rank(r: char) -> bool {
    match r {
        '1'..='8' => true,
        _ => false
    }
}

fn parse_an(s: &str) -> Vec<(u8, u8)> {
    vec![(match s.chars().nth(0).unwrap() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!(),
    }, match s.chars().nth(1).unwrap() {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!(),
    })]
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

fn print_range(indices: &Vec<(u8, u8)>) {

    let r1 = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
    let r2 = ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'];
    let r3 = ['.', '.', '.', '.', '.', '.', '.', '.'];
    let r4 = ['.', '.', '.', '.', '.', '.', '.', '.'];
    let r5 = ['.', '.', '.', '.', '.', '.', '.', '.'];
    let r6 = ['.', '.', '.', '.', '.', '.', '.', '.'];
    let r7 = ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'];
    let r8 = ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'];

    let b = [r1, r2, r3, r4, r5, r6, r7, r8];

    for (i, r) in b.iter().rev().enumerate() {
        for (j, s) in r.iter().enumerate() {
            let mut highlight = false;
            for h in indices {
                if i as u8 == (7 - h.1) && j as u8 == h.0 {
                    highlight = true;
                }
            }
            match highlight {
                true => print!("({})", s),
                false => print!(" {} ", s),
            }
        }
        if i != 7 {
            println!(" /");
        } else {
            println!();
        }
    }

}


