use std::{fmt::Display, iter};

use anyhow::Result;
use itertools::Itertools;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 7)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn parse(challenge: &str) -> Vec<(usize, Vec<usize>)> {
    challenge
        .lines()
        .map(|line| {
            let (target, constants) = line.split_once(": ").unwrap();
            let target = target.parse().unwrap();
            let constants = constants
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (target, constants)
        })
        .collect()
}

enum Operator {
    Add,
    Multiply,
    Concat,
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(if b == 0 { 1 } else { b.ilog10() + 1 }) + b
}

fn apply_operators(constants: &[usize], operators: &[&Operator]) -> usize {
    constants
        .iter()
        .tuple_windows()
        .zip(operators)
        .fold(None, |acc, ((first, last), operator)| {
            Some(match operator {
                Operator::Add => acc.unwrap_or(*first) + last,
                Operator::Multiply => acc.unwrap_or(*first) * last,
                Operator::Concat => concat(acc.unwrap_or(*first), *last),
            })
        })
        .unwrap()
}

fn solve_part1(challenge: &str) -> usize {
    let operators = [Operator::Add, Operator::Multiply];
    parse(challenge)
        .iter()
        .filter(|(target, constants)| {
            iter::repeat(&operators)
                .take(constants.len() - 1)
                .multi_cartesian_product()
                .any(|operators| *target == apply_operators(constants, &operators))
        })
        .map(|(target, _)| target)
        .sum()
}

fn solve_part2(challenge: &str) -> usize {
    let operators = [Operator::Add, Operator::Multiply, Operator::Concat];
    parse(challenge)
        .iter()
        .filter(|(target, constants)| {
            iter::repeat(&operators)
                .take(constants.len() - 1)
                .multi_cartesian_product()
                .any(|operators| *target == apply_operators(constants, &operators))
        })
        .map(|(target, _)| target)
        .sum()
}
