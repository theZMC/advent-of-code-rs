use std::{collections::BTreeMap, error::Error, iter::once};

use itertools::Itertools;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 13)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    best_seating(&map_happiness(challenge)).to_string()
}

fn map_happiness(challenge: &str) -> BTreeMap<String, BTreeMap<String, i32>> {
    let mut happiness = BTreeMap::new();

    challenge.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let they = parts.first().unwrap();
        let them = parts.last().unwrap().trim_matches('.');
        let modifier = match parts.get(2) {
            Some(&"gain") => parts.get(3).unwrap().parse::<i32>().unwrap(),
            Some(&"lose") => -parts.get(3).unwrap().parse::<i32>().unwrap(),
            _ => 0,
        };

        happiness
            .entry(they.to_string())
            .or_insert_with(BTreeMap::new)
            .insert(them.to_string(), modifier);
    });

    happiness
}

fn best_seating(happy_map: &BTreeMap<String, BTreeMap<String, i32>>) -> i32 {
    happy_map
        .keys()
        .permutations(happy_map.len())
        .map(|seating| -> i32 {
            seating
                .windows(2)
                .chain(once(
                    &[*seating.last().unwrap(), *seating.first().unwrap()][..],
                ))
                .map(|pair| -> i32 {
                    let they = pair[0];
                    let them = pair[1];
                    happy_map.get(they).unwrap().get(them).unwrap()
                        + happy_map.get(them).unwrap().get(they).unwrap()
                })
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn solve_part2(challenge: &str) -> String {
    let mut happy_map = map_happiness(challenge);
    happy_map.insert("Me".to_string(), BTreeMap::new());

    happy_map.clone().keys().for_each(|them| {
        happy_map.get_mut(them).unwrap().insert("Me".to_string(), 0);
        happy_map.get_mut("Me").unwrap().insert(them.to_string(), 0);
    });

    best_seating(&happy_map).to_string()
}
