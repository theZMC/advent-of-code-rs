use std::error::Error;

use fancy_regex::Regex;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 5)?;

    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    let mut nice_count = 0;

    let vowel_rgx = Regex::new(r"(.*[aeiou]){3}").unwrap();
    let pair_rgx = Regex::new(r"(.)\1").unwrap();
    let naughty_rgx = Regex::new(r"(ab|cd|pq|xy)").unwrap();

    challenge.lines().for_each(|line| {
        if vowel_rgx.is_match(line).unwrap()
            && pair_rgx.is_match(line).unwrap()
            && !naughty_rgx.is_match(line).unwrap()
        {
            nice_count += 1;
        }
    });

    nice_count.to_string()
}

fn solve_part2(challenge: &str) -> String {
    let mut nice_count = 0;
    let pair_rgx = Regex::new(r"(..).*\1").unwrap();
    let sandwich_rgx = Regex::new(r"(.).\1").unwrap();

    challenge.lines().for_each(|line| {
        if pair_rgx.is_match(line).unwrap() && sandwich_rgx.is_match(line).unwrap() {
            nice_count += 1;
        }
    });

    nice_count.to_string()
}
