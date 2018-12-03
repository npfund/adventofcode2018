extern crate regex;

use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let regex = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut fabric: HashMap<(i32, i32), i32> = HashMap::new();

    for line in input.lines().map(|x| x.unwrap()) {
        let captures = regex.captures(&line).unwrap();
        let start = (captures[1].parse::<i32>().unwrap(), captures[2].parse::<i32>().unwrap());
        let width = captures[3].parse::<i32>().unwrap();
        let height = captures[4].parse::<i32>().unwrap();

        for h in 0..height {
            for w in 0..width {
                let coord = (start.0 + w, start.1 + h);
                let entry = fabric.entry(coord).or_insert(0);
                *entry += 1;
            }
        }
    }

    println!("{}", fabric.values().filter(|&&x| x > 1).count());
}

fn part2() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"))
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut fabric: HashMap<(i32, i32), i32> = HashMap::new();

    for line in input.clone() {
        let captures = regex.captures(&line).unwrap();
        let start = (captures[2].parse::<i32>().unwrap(), captures[3].parse::<i32>().unwrap());
        let width = captures[4].parse::<i32>().unwrap();
        let height = captures[5].parse::<i32>().unwrap();

        for h in 0..height {
            for w in 0..width {
                let coord = (start.0 + w, start.1 + h);
                let entry = fabric.entry(coord).or_insert(0);
                *entry += 1;
            }
        }
    }

    'outer: for line in input {
        let captures = regex.captures(&line).unwrap();
        let start = (captures[2].parse::<i32>().unwrap(), captures[3].parse::<i32>().unwrap());
        let width = captures[4].parse::<i32>().unwrap();
        let height = captures[5].parse::<i32>().unwrap();

        for h in 0..height {
            for w in 0..width {
                let coord = (start.0 + w, start.1 + h);
                let value = fabric.get(&coord).unwrap();
                if *value > 1 {
                    continue 'outer;
                }
            }
        }

        println!("{}", captures[1].parse::<i32>().unwrap());
        return;
    }
}
