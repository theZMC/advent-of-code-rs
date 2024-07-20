use std::{collections::HashSet, error::Error};

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 3)?;

    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut last_visited = (0, 0);

    visited.insert(last_visited);

    challenge.chars().for_each(|c| {
        let (last_x, last_y) = last_visited;

        match c {
            '^' => last_visited = (last_x, last_y + 1),
            'v' => last_visited = (last_x, last_y - 1),
            '>' => last_visited = (last_x + 1, last_y),
            '<' => last_visited = (last_x - 1, last_y),
            _ => {}
        }

        visited.insert(last_visited);
    });

    visited.len().to_string()
}

fn solve_part2(challenge: &str) -> String {
    let mut visited = HashSet::new();

    let mut santa_last_visited = (0, 0);
    let mut robot_last_visited = (0, 0);

    visited.insert(santa_last_visited);

    challenge.chars().enumerate().for_each(|(i, c)| {
        let (mut last_x, mut last_y) = santa_last_visited;
        let mut last_visited = &mut santa_last_visited;

        if i % 2 == 0 {
            (last_x, last_y) = robot_last_visited;
            last_visited = &mut robot_last_visited;
        }

        match c {
            '^' => *last_visited = (last_x, last_y + 1),
            'v' => *last_visited = (last_x, last_y - 1),
            '>' => *last_visited = (last_x + 1, last_y),
            '<' => *last_visited = (last_x - 1, last_y),
            _ => {}
        }

        visited.insert(*last_visited);
    });

    visited.len().to_string()
}
