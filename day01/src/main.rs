use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));

    let result: i32 = input.lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("{}", result);
}
