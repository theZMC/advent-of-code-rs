use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
};

use itertools::Itertools;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 1)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn parse_line(line: &str) -> (i64, i64) {
    let (left_part, right_part) = line.split_whitespace().collect_tuple().unwrap();
    (
        left_part.parse::<i64>().unwrap(),
        right_part.parse::<i64>().unwrap(),
    )
}

fn solve_part1(challenge: &str) -> String {
    let (left, right): (BinaryHeap<_>, BinaryHeap<_>) = challenge.lines().map(parse_line).unzip();

    left.into_sorted_vec()
        .into_iter()
        .zip(right.into_sorted_vec())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>()
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    let (left, right) = challenge.lines().map(parse_line).fold(
        (BinaryHeap::new(), HashMap::new()),
        |(mut left, mut right), (left_part, right_part)| {
            left.push(left_part);
            *right.entry(right_part).or_default() += 1;
            (left, right)
        },
    );

    left.into_iter()
        .fold(0, |acc, l| acc + right.get(&l).unwrap_or(&0) * l)
        .to_string()
}
