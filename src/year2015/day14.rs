use std::{collections::BTreeMap, fmt::Display};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 14)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

struct Stat {
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

impl Stat {
    fn new(speed: i32, fly_time: i32, rest_time: i32) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
        }
    }

    fn distance(&self, time: i32) -> i32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = time / cycle_time;
        let remaining = time % cycle_time;
        let fly_time = std::cmp::min(remaining, self.fly_time);
        cycles * self.speed * self.fly_time + fly_time * self.speed
    }
}

fn solve_part1(challenge: &str) -> String {
    reindeer_stats(challenge)
        .values()
        .map(|stat| stat.distance(2503))
        .max()
        .unwrap()
        .to_string()
}

fn reindeer_stats(challenge: &str) -> BTreeMap<String, Stat> {
    BTreeMap::from_iter(challenge.lines().map(|line| -> (String, Stat) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let name = parts.first().unwrap();
        let speed = parts.get(3).unwrap().parse::<i32>().unwrap();
        let fly_time = parts.get(6).unwrap().parse::<i32>().unwrap();
        let rest_time = parts.get(13).unwrap().parse::<i32>().unwrap();

        (name.to_string(), Stat::new(speed, fly_time, rest_time))
    }))
}

fn solve_part2(challenge: &str) -> String {
    let mut points = BTreeMap::from_iter(
        reindeer_stats(challenge)
            .keys()
            .map(|name| (name.to_string(), 0)),
    );

    let stats = reindeer_stats(challenge);

    for time in 1..=2503 {
        let distances: BTreeMap<_, _> = stats
            .iter()
            .map(|(name, stat)| (name, stat.distance(time)))
            .collect();
        let max_distance = *distances.values().max().unwrap();
        for (name, distance) in distances {
            if distance == max_distance {
                *points.get_mut(name).unwrap() += 1;
            }
        }
    }

    points.values().max().unwrap().to_string()
}
