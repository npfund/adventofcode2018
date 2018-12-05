use std::fs::File;
use std::io::Read;

fn main() {
    part1();
}

fn part1() {
    let mut file = File::open("input.txt").expect("File not found!");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file!");
    input = input.trim().to_owned();

    loop {
        let mut reacted = String::new();
        {
            let mut iter = input.chars().peekable();
            loop {
                match (iter.next(), iter.peek()) {
                    (Some(next), Some(&peek)) => {
                        if (next.is_ascii_lowercase() && peek.is_ascii_uppercase() && next.to_ascii_uppercase() == peek)
                            || (next.is_ascii_uppercase() && peek.is_ascii_lowercase() && next.to_ascii_lowercase() == peek) {
                            iter.next();
                        } else {
                            reacted.push(next);
                        }
                    },
                    (Some(next), None) => {
                        reacted.push(next);
                    },
                    _ => break,
                }
            }
        }

        if reacted.len() == input.len() {
            input = reacted;
            break;
        } else {
            input = reacted;
        }
    }

    println!("{}", input.len());
}
