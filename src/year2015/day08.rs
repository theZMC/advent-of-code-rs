use std::error::Error;

use crate::{challenge::Fetcher, year2015::YEAR};

use fancy_regex::Regex;

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 8)?;

    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    let total_in_code: usize = challenge.lines().map(|line| line.len()).sum();
    let mut total_in_memory = 0;

    let escape_rgx = Regex::new(r#"(\\"|\\\\|\\x\h{2})"#).unwrap();

    challenge.lines().for_each(|line| {
        total_in_memory += escape_rgx.replace_all(&line[1..line.len() - 1], "_").len()
    });

    format!("{}", total_in_code - total_in_memory)
}

fn solve_part2(challenge: &str) -> String {
    let total_in_code: usize = challenge.lines().map(|line| line.len()).sum();
    let total_escaped: usize = challenge
        .lines()
        .map(|line| {
            line.chars().fold(String::new(), |mut acc, c| {
                match c {
                    '\\' => acc.push_str("\\\\"),
                    '"' => acc.push_str("\\\""),
                    _ => acc.push(c),
                }
                acc
            })
        })
        .map(|line| line.len() + 2)
        .sum();

    format!("{}", total_escaped - total_in_code)
}
