use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 8)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

struct Airwaves((i32, i32), HashMap<char, Vec<(i32, i32)>>);

fn parse(challenge: &str) -> Airwaves {
    let lines: Vec<&str> = challenge.lines().collect();

    Airwaves(
        ((lines[0].len() - 1) as i32, (lines.len() - 1) as i32),
        lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(move |(x, c)| (c, (x as i32, y as i32)))
            })
            .into_group_map(),
    )
}

fn in_bounds(l: &(i32, i32), bounds: &(i32, i32)) -> bool {
    l.0 <= bounds.0 && l.1 <= bounds.1 && l.0 >= 0 && l.1 >= 0
}

fn adjust(o: &(i32, i32), a: &(i32, i32)) -> (i32, i32) {
    (o.0 + a.0, o.1 + a.1)
}

fn solve_part1(challenge: &str) -> usize {
    let Airwaves(bounds, antennae) = parse(challenge);
    antennae
        .iter()
        .flat_map(|(_, locations)| {
            locations.iter().combinations(2).flat_map(|combo| {
                let p1 = combo.first().unwrap();
                let p2 = combo.last().unwrap();

                let a1 = (-(p2.0 - p1.0), -(p2.1 - p1.1));
                let a2 = (-(p1.0 - p2.0), -(p1.1 - p2.1));

                [adjust(p1, &a1), adjust(p2, &a2)]
            })
        })
        .filter(|l| in_bounds(l, &bounds))
        .collect::<HashSet<_>>()
        .len()
}

fn solve_part2(challenge: &str) -> usize {
    let Airwaves(bounds, antennae) = parse(challenge);
    antennae
        .iter()
        .flat_map(|(_, locations)| {
            locations.iter().combinations(2).flat_map(|combo| {
                let mut antinodes = HashSet::new();

                let p1 = combo.first().unwrap();
                let p2 = combo.last().unwrap();

                antinodes.insert(**p1);
                antinodes.insert(**p2);

                let a1 = (-(p2.0 - p1.0), -(p2.1 - p1.1));
                let a2 = (-(p1.0 - p2.0), -(p1.1 - p2.1));

                let mut p1 = adjust(p1, &a1);

                while in_bounds(&p1, &bounds) {
                    antinodes.insert(p1);
                    p1 = adjust(&p1, &a1);
                }

                let mut p2 = adjust(p2, &a2);

                while in_bounds(&p2, &bounds) {
                    antinodes.insert(p2);
                    p2 = adjust(&p2, &a2);
                }

                antinodes
            })
        })
        .collect::<HashSet<_>>()
        .len()
}
