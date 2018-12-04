extern crate chrono;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use chrono::NaiveDateTime;
use chrono::Timelike;
use regex::Regex;

fn main() {
    part1();
}

fn part1() {
    let input = BufReader::new(File::open("input.txt").expect("File not found!"));
    let regex = Regex::new(r"\[([^]]*)] (.*)").unwrap();

    let mut logs = input.lines().map(|x| x.unwrap())
        .map(|x| {
            let captures = regex.captures(&x).unwrap();
            let date = NaiveDateTime::parse_from_str(&captures[1], "%Y-%m-%d %H:%M").unwrap();
            return Log {date, entry: captures[2].to_owned()};
        })
        .collect::<Vec<Log>>();

    logs.sort();

    let guard_regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut guards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut current_guard: u32 = 0;
    let mut start_sleep: NaiveDateTime = NaiveDateTime::from_timestamp_opt(0, 0).unwrap();

    for log in logs.iter() {
        match guard_regex.captures(&log.entry) {
            Some(captures) => current_guard = captures[1].parse().unwrap(),
            None => match log.entry.as_ref() {
                "wakes up" => {
                    let duration = log.date - start_sleep;

                    for m in 0..duration.num_minutes() {
                        let guard = guards.entry(current_guard).or_insert(HashMap::new());
                        let minute = guard.entry(start_sleep.minute() + m as u32).or_insert(0);
                        *minute += 1;
                    }
                },
                "falls asleep" => start_sleep = log.date,
                _ => println!("Unknown message: {}", log.entry),
            },
        }
    }

    let highest = guards.iter()
        .max_by(|l, r| {
            let l_sum = l.1.values().sum::<u32>();
            let r_sum = r.1.values().sum::<u32>();

            return l_sum.cmp(&r_sum);
        }).unwrap();

    let highest_minute = highest.1.iter().max_by(|l, r| {
        return l.1.cmp(r.1);
    }).unwrap();

    println!("{} = {} * {}", highest.0 * highest_minute.0, highest.0, highest_minute.0);
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Log {
    date: NaiveDateTime,
    entry: String,
}

impl Ord for Log {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}
