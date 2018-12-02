use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));

    let mut exactly2 = 0;
    let mut exactly3 = 0;

    for x in input.lines().map(|x| x.unwrap()) {
        let mut chars: HashMap<char, u32> = HashMap::new();
        for c in x.chars() {
            let value = chars.entry(c).or_insert(0);
            *value += 1;
        }

        if chars.values().any(|&x| x == 2) {
            exactly2 += 1;
        }

        if chars.values().any(|&x| x == 3) {
            exactly3 += 1;
        }
    }

    println!("{}", exactly2 * exactly3);
}

fn part2() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let left_lines = input.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let right_lines = left_lines.clone();

    for x in left_lines {
        for y in right_lines.clone() {
            let chars = x.chars()
                .zip(y.chars())
                .filter(|(l, r)| l == r)
                .map(|(l, _r)| l)
                .collect::<String>();

            if chars.len() == 25 {
                println!("{}", chars);
                return;
            }
        }
    }
}
