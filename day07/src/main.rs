use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    part1();
}

fn part1() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let mut tree: HashMap<char, Node> = HashMap::new();
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\.").unwrap();

    for line in input.lines().map(|x| x.unwrap()) {
        let captures = re
            .captures(&line)
            .unwrap()
            .iter()
            .map(|x| x.unwrap().as_str().chars().next().unwrap())
            .collect::<Vec<char>>();

        let first = tree.entry(captures[1]).or_insert(Node {
            name: captures[1],
            ancestors: HashSet::new(),
            descendants: HashSet::new(),
        });
        first.descendants.insert(captures[2]);

        let second = tree.entry(captures[2]).or_insert(Node {
            name: captures[2],
            ancestors: HashSet::new(),
            descendants: HashSet::new(),
        });
        second.ancestors.insert(captures[1]);
    }

    let mut current = tree
        .iter()
        .find(|&(c, n)| n.ancestors.len() == 0)
        .unwrap()
        .1;

    let mut potentials = tree
        .iter()
        .filter(|(c, n)| n.ancestors.len() == 0)
        .map(|(c, n)| c.clone())
        .collect::<Vec<char>>();
    let mut used: HashSet<char> = HashSet::new();
    let mut order = String::new();

    while potentials.len() > 0 {
        let mut open = potentials
            .iter()
            .filter(|x| tree.get(x).unwrap().ancestors.difference(&used).count() == 0)
            .map(|x| x.clone())
            .collect::<Vec<char>>();

        open.sort();
        open.reverse();

        let current = open.pop().unwrap();
        used.insert(current);
        order.push(current);

        potentials = open;
        potentials.append(&mut tree.get(&current).unwrap().descendants.clone().iter().map(|c| c.clone()).collect::<Vec<char>>());
    }

    println!("{}", order);
}

#[derive(Debug)]
struct Node {
    name: char,
    ancestors: HashSet<char>,
    descendants: HashSet<char>,
}
