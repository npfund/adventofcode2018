use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
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

fn part2() {
    let mut file = File::open("input.txt").expect("File not found!");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("Could not read file!");
    input = input.trim().to_owned();
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't' , 'u', 'v', 'w', 'x', 'y', 'z'];

    let mut lengths: HashMap<char, usize> = HashMap::new();

    for c in chars.iter() {
        let mut filtered: String = input.chars().filter(|x| x.to_ascii_lowercase() != *c).collect();
        loop {
            let mut reacted = String::new();
            {
                let mut iter = filtered.chars().peekable();
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

            if reacted.len() == filtered.len() {
                filtered = reacted;
                break;
            } else {
                filtered = reacted;
            }
        }

        lengths.insert(*c, filtered.len());
    }

    println!("{}", lengths.values().min().unwrap());
}
