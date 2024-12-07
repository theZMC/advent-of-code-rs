use std::fmt::Display;

use anyhow::Result;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 2)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn parse(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn n_minus_one_combos<T: Clone>(vec: Vec<T>) -> Vec<Vec<T>> {
    (0..vec.len())
        .map(|i| {
            vec.iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, v)| v.clone())
                .collect()
        })
        .collect()
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let levels = levels.to_owned();
    let is_sorted = levels.windows(2).all(|w| w[0] < w[1]);
    let is_reverse_sorted = levels.windows(2).all(|w| w[0] > w[1]);
    let is_safe = levels.windows(2).all(|w| (w[0] - w[1]).abs() < 4);
    is_safe && (is_sorted || is_reverse_sorted)
}

fn solve_part1(challenge: &str) -> String {
    challenge
        .lines()
        .map(parse)
        .filter(is_safe)
        .count()
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    challenge
        .lines()
        .map(parse)
        .map(n_minus_one_combos)
        .filter(|combos| combos.iter().any(is_safe))
        .count()
        .to_string()
}
