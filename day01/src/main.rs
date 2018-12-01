use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));

    let result: i32 = input.lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("{}", result);
}

fn part2() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));

    let mut current = 0;
    let mut found: HashSet<i32> = HashSet::new();
    found.insert(0);

    for x in input.lines().map(|x| x.unwrap()).collect::<Vec<String>>().iter().cycle() {
        current += x.parse::<i32>().unwrap();
        if found.contains(&current) {
            break;
        } else {
            found.insert(current);
        }
    }

    println!("{}", current);
}
