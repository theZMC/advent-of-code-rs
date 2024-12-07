use std::fmt::Display;

use anyhow::Result;
use fancy_regex::Regex;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 3)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn solve_part1(challenge: &str) -> String {
    let rgx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    rgx.captures_iter(challenge)
        .map(|caps| {
            let caps = caps.unwrap();
            let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>()
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    let rgx = Regex::new(r"(mul\((\d+),(\d+)\)|don't\(\)|do\(\))").unwrap();
    rgx.captures_iter(challenge)
        .fold((0, true), |(acc, we_good), caps| {
            let caps = caps.unwrap();
            match caps.get(1).unwrap().as_str() {
                "don't()" => (acc, false),
                "do()" => (acc, true),
                _ => {
                    let a = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let b = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                    (acc + if we_good { a * b } else { 0 }, we_good)
                }
            }
        })
        .0
        .to_string()
}
