use std::{collections::BTreeMap, fmt::Display};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 15)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

struct Ingredient {
    pub c: i32,
    pub d: i32,
    pub f: i32,
    pub t: i32,
    pub k: i32,
}

impl Ingredient {
    fn new(capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Self {
        Self {
            c: capacity,
            d: durability,
            f: flavor,
            t: texture,
            k: calories,
        }
    }
}

fn solve_part1(challenge: &str) -> String {
    let binding = parse_ingredients(challenge);
    let ing = binding.values().collect::<Vec<_>>();
    let mut best_score = 0;

    for i in 10..40 {
        for j in 10..40 {
            for k in 10..40 {
                let l = 100 - i - j - k;

                if l < 1 {
                    continue;
                }

                let cookie = [
                    0.max(i * ing[0].c + j * ing[1].c + k * ing[2].c + l * ing[3].c),
                    0.max(i * ing[0].d + j * ing[1].d + k * ing[2].d + l * ing[3].d),
                    0.max(i * ing[0].f + j * ing[1].f + k * ing[2].f + l * ing[3].f),
                    0.max(i * ing[0].t + j * ing[1].t + k * ing[2].t + l * ing[3].t),
                ];

                if cookie.iter().any(|&x| x < 1) {
                    continue;
                }

                best_score = best_score.max(cookie.iter().map(|&x| x as u64).product());
            }
        }
    }

    best_score.to_string()
}

fn parse_ingredients(challenge: &str) -> BTreeMap<String, Ingredient> {
    BTreeMap::from_iter(challenge.lines().map(|line| -> (String, Ingredient) {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        let name = parts.first().unwrap().trim_end_matches(':');

        let capacity = parts
            .get(2)
            .unwrap()
            .trim_matches(',')
            .parse::<i32>()
            .unwrap();
        let durability = parts
            .get(4)
            .unwrap()
            .trim_matches(',')
            .parse::<i32>()
            .unwrap();
        let flavor = parts
            .get(6)
            .unwrap()
            .trim_matches(',')
            .parse::<i32>()
            .unwrap();
        let texture = parts
            .get(8)
            .unwrap()
            .trim_matches(',')
            .parse::<i32>()
            .unwrap();
        let calories = parts
            .get(10)
            .unwrap()
            .trim_matches(',')
            .parse::<i32>()
            .unwrap();

        (
            name.to_string(),
            Ingredient::new(capacity, durability, flavor, texture, calories),
        )
    }))
}

fn solve_part2(challenge: &str) -> String {
    let binding = parse_ingredients(challenge);
    let ing = binding.values().collect::<Vec<_>>();
    let mut best_score = 0;

    for i in 10..40 {
        for j in 10..40 {
            for k in 10..40 {
                let l = 100 - i - j - k;

                if l < 1 {
                    continue;
                }

                let cals = i * ing[0].k + j * ing[1].k + k * ing[2].k + l * ing[3].k;

                if cals != 500 {
                    continue;
                }

                let cookie = [
                    0.max(i * ing[0].c + j * ing[1].c + k * ing[2].c + l * ing[3].c),
                    0.max(i * ing[0].d + j * ing[1].d + k * ing[2].d + l * ing[3].d),
                    0.max(i * ing[0].f + j * ing[1].f + k * ing[2].f + l * ing[3].f),
                    0.max(i * ing[0].t + j * ing[1].t + k * ing[2].t + l * ing[3].t),
                ];

                if cookie.iter().any(|&x| x < 1) {
                    continue;
                }

                best_score = best_score.max(cookie.iter().map(|&x| x as u64).product());
            }
        }
    }

    best_score.to_string()
}
