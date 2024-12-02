use std::error::Error;

use itertools::Itertools;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 17)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    let containers = challenge
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut possible_combinations = 0;

    for i in 1..containers.len() {
        for combination in containers.iter().combinations(i) {
            if combination.iter().copied().sum::<u32>() == 150 {
                possible_combinations += 1;
            }
        }
    }

    possible_combinations.to_string()
}

fn solve_part2(challenge: &str) -> String {
    let containers = challenge
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut possible_combinations = 0;

    for i in 1..containers.len() {
        for combination in containers.iter().combinations(i) {
            if combination.iter().copied().sum::<u32>() == 150 {
                possible_combinations += 1;
            }
        }

        if possible_combinations > 0 {
            break;
        }
    }

    possible_combinations.to_string()
}
