use itertools::Itertools;
use std::collections::BTreeMap;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 9)?;

    let mut cities = BTreeMap::<&str, BTreeMap<&str, u32>>::new();

    challenge.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse::<u32>().unwrap();

        if !cities.contains_key(from) {
            cities.insert(from, BTreeMap::new());
        }

        if !cities.contains_key(to) {
            cities.insert(to, BTreeMap::new());
        }

        cities.get_mut(from).unwrap().insert(to, distance);
        cities.get_mut(to).unwrap().insert(from, distance);
    });

    let mut min_distance = u32::MAX;
    let mut max_distance = 0;

    cities
        .keys()
        .permutations(cities.len())
        .for_each(|permutation| {
            let distance = permutation
                .windows(2)
                .map(|pair| cities[pair[0]][pair[1]])
                .sum::<u32>();

            min_distance = min_distance.min(distance);
            max_distance = max_distance.max(distance);
        });

    Ok((min_distance.to_string(), max_distance.to_string()))
}
