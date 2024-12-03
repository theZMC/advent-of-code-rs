use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::{Captures, Regex};

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 19)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    let (replacements, molecule) = challenge.split_once("\n\n").unwrap();

    replacements
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .into_group_map()
        .into_iter()
        .flat_map(|(from, repls)| {
            molecule.match_indices(from).flat_map(move |(pos, _)| {
                repls.clone().into_iter().map(move |repl| {
                    let mut new_molecule = molecule.to_string();
                    new_molecule.replace_range(pos..pos + from.len(), repl);
                    new_molecule
                })
            })
        })
        .collect::<HashSet<_>>()
        .len()
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    let (replacements, molecule) = challenge.split_once("\n\n").unwrap();

    let elements = replacements
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            (to, from)
        })
        .map(|(to, from)| {
            (
                to.chars().rev().collect::<String>(),
                from.chars().rev().collect::<String>(),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut molecule = molecule.trim().chars().rev().collect::<String>();

    let pattern = format!(
        "({})",
        elements.keys().cloned().collect::<Vec<_>>().join("|")
    );

    let rgx = Regex::new(&pattern).unwrap();
    let mut steps = 0;

    while molecule != "e" {
        molecule = rgx
            .replacen(&molecule, 1, |caps: &Captures| {
                let replacement = caps.get(1).unwrap().as_str();
                elements[replacement].clone()
            })
            .into_owned();

        steps += 1;
    }

    steps.to_string()
}
