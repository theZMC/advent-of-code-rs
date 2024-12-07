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

fn concat(a: &usize, b: &usize) -> usize {
    a * 10_usize.pow(if b.eq(&0) { 1 } else { b.ilog10() + 1 }) + b
}

fn can_equal_target(target: &usize, constants: &[usize], operators: &[&Operator]) -> bool {
    constants
        .iter()
        .tuple_windows()
        .zip(operators)
        .try_fold(0, |mut acc, ((first, last), operator)| {
            if acc == 0 {
                acc = *first
            }
            let current = match operator {
                Operator::Add => acc + last,
                Operator::Multiply => acc * last,
                Operator::Concat => concat(&acc, last),
            };
            if current.gt(target) {
                None
            } else {
                Some(current)
            }
        })
        .is_some_and(|i| i.eq(target))
}

fn solve_part1(challenge: &str) -> usize {
    let operators = [Operator::Add, Operator::Multiply];
    parse(challenge)
        .iter()
        .filter(|(target, constants)| {
            iter::repeat(&operators)
                .take(constants.len() - 1)
                .multi_cartesian_product()
                .any(|operators| can_equal_target(target, constants, &operators))
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
                .any(|operators| can_equal_target(target, constants, &operators))
        })
        .map(|(target, _)| target)
        .sum()
}
