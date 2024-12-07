use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 5)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn parse(challenge: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let (rule_set, page_updates) = challenge.split_once("\n\n").unwrap();

    let rule_set = rule_set
        .lines()
        .map(|line| {
            line.split_once('|')
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .into_group_map();

    let page_updates = page_updates
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rule_set, page_updates)
}

trait RuleSet {
    fn allows(&self, pages: &[usize]) -> bool;
}

impl RuleSet for HashMap<usize, Vec<usize>> {
    fn allows(&self, pages: &[usize]) -> bool {
        pages
            .iter()
            .rev()
            .try_fold(HashSet::new(), |mut seen, page| {
                let allowed = !seen
                    .iter()
                    .any(|seen| self.get(seen).map_or(false, |set| set.contains(page)));
                seen.insert(*page);
                allowed.then_some(seen)
            })
            .is_some()
    }
}

fn middle_of(pages: &[usize]) -> usize {
    *pages.get(pages.len() / 2).unwrap()
}

fn solve_part1(challenge: &str) -> usize {
    let (rule_set, page_updates) = parse(challenge);
    page_updates
        .iter()
        .filter(|pages| rule_set.allows(pages))
        .map(|pages| middle_of(pages))
        .sum()
}

fn fix_pages(rule_set: &HashMap<usize, Vec<usize>>, pages: &[usize]) -> Vec<usize> {
    pages.iter().rev().fold(Vec::new(), |acc, &page| {
        (0..=acc.len())
            .map(|i| {
                let mut fixed = acc.clone();
                fixed.insert(i, page);
                fixed
            })
            .find(|fixed| rule_set.allows(fixed))
            .unwrap_or(acc)
    })
}

fn solve_part2(challenge: &str) -> usize {
    let (rule_set, page_updates) = parse(challenge);
    page_updates
        .iter()
        .filter(|pages| !rule_set.allows(pages))
        .map(|pages| fix_pages(&rule_set, pages))
        .map(|pages| middle_of(&pages))
        .sum()
}
