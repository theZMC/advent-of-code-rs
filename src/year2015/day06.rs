use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 6)?;

    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn solve_part1(challenge: &str) -> usize {
    let mut hs = HashSet::new();

    challenge.lines().for_each(|line| {
        let toggling = line.starts_with("toggle");
        let turning_on = line.starts_with("turn on");
        let turning_off = line.starts_with("turn off");

        let mut start = (-1, -1);
        let mut end = (-1, -1);

        line.split(' ')
            .filter(|part| part.contains(','))
            .for_each(|pair| {
                let coords: Vec<&str> = pair.split(',').collect();
                let x = coords[0].parse::<i32>().unwrap();
                let y = coords[1].parse::<i32>().unwrap();
                if start == (-1, -1) {
                    start = (x, y);
                } else {
                    end = (x, y);
                }
            });

        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                let coord = (x, y);
                if toggling {
                    if hs.contains(&coord) {
                        hs.remove(&coord);
                    } else {
                        hs.insert(coord);
                    }
                }
                if turning_on {
                    hs.insert(coord);
                }
                if turning_off {
                    hs.remove(&coord);
                }
            }
        }
    });

    hs.len()
}

fn solve_part2(challenge: &str) -> i32 {
    let mut hm = HashMap::new();

    challenge.lines().for_each(|line| {
        let toggling = line.starts_with("toggle");
        let turning_on = line.starts_with("turn on");
        let turning_off = line.starts_with("turn off");

        let mut start = (-1, -1);
        let mut end = (-1, -1);

        line.split(' ')
            .filter(|part| part.contains(','))
            .for_each(|pair| {
                let coords: Vec<&str> = pair.split(',').collect();
                let x = coords[0].parse::<i32>().unwrap();
                let y = coords[1].parse::<i32>().unwrap();
                if start == (-1, -1) {
                    start = (x, y);
                } else {
                    end = (x, y);
                }
            });

        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                let coord = (x, y);
                if toggling {
                    hm.entry(coord)
                        .and_modify(|intensity| *intensity += 2)
                        .or_insert(2);
                }
                if turning_on {
                    hm.entry(coord)
                        .and_modify(|intensity| *intensity += 1)
                        .or_insert(1);
                }
                if turning_off {
                    hm.entry(coord)
                        .and_modify(|intensity| *intensity = (*intensity - 1).max(0))
                        .or_insert(0);
                }
            }
        }
    });

    hm.values().sum::<i32>()
}
