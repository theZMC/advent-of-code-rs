use std::{collections::HashMap, fmt::Display};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 23)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

#[derive(Debug)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

fn parse(challenge: &str) -> Vec<Instruction> {
    challenge
        .lines()
        .flat_map(|line| {
            let (instruction, params) = line.split_once(' ').unwrap();
            match instruction {
                "jmp" => Some(Instruction::Jmp(params.parse().unwrap())),
                "hlf" => Some(Instruction::Hlf(params.parse().unwrap())),
                "tpl" => Some(Instruction::Tpl(params.parse().unwrap())),
                "inc" => Some(Instruction::Inc(params.parse().unwrap())),
                "jie" => Some({
                    let (register, amount) = params.split_once(", ").unwrap();
                    Instruction::Jie(register.parse().unwrap(), amount.parse().unwrap())
                }),
                "jio" => Some({
                    let (register, amount) = params.split_once(", ").unwrap();
                    Instruction::Jio(register.parse().unwrap(), amount.parse().unwrap())
                }),
                _ => None,
            }
        })
        .collect()
}

fn solve_part1(challenge: &str) -> i32 {
    let program = parse(challenge);
    let mut state = HashMap::new();
    let mut ptr = 0;
    while ptr >= 0 && ptr < program.len() as i32 {
        match program.get(ptr as usize).unwrap() {
            Instruction::Hlf(r) => {
                state.entry(r).and_modify(|v| *v /= 2).or_insert(0);
                ptr += 1;
            }
            Instruction::Tpl(r) => {
                state.entry(r).and_modify(|v| *v *= 3).or_insert(0);
                ptr += 1;
            }
            Instruction::Inc(r) => {
                state.entry(r).and_modify(|v| *v += 1).or_insert(1);
                ptr += 1;
            }
            Instruction::Jmp(q) => ptr += q,
            Instruction::Jie(r, q) => {
                if state.get(r).is_some_and(|v| *v % 2 == 0) {
                    ptr += q
                } else {
                    ptr += 1
                }
            }
            Instruction::Jio(r, q) => {
                if state.get(r).is_some_and(|v| *v == 1) {
                    ptr += q
                } else {
                    ptr += 1
                }
            }
        }
    }

    *state.get(&'b').unwrap()
}

fn solve_part2(challenge: &str) -> i64 {
    let program = parse(challenge);
    let mut state = HashMap::new();
    state.insert(&'a', 1);
    let mut ptr = 0;
    while ptr >= 0 && ptr < program.len() as i64 {
        match program.get(ptr as usize).unwrap() {
            Instruction::Hlf(r) => {
                state.entry(r).and_modify(|v| *v /= 2).or_insert(0);
                ptr += 1;
            }
            Instruction::Tpl(r) => {
                state.entry(r).and_modify(|v| *v *= 3).or_insert(0);
                ptr += 1;
            }
            Instruction::Inc(r) => {
                state.entry(r).and_modify(|v| *v += 1).or_insert(1);
                ptr += 1;
            }
            Instruction::Jmp(q) => ptr += *q as i64,
            Instruction::Jie(r, q) => {
                if state.get(r).is_some_and(|v| *v % 2 == 0) {
                    ptr += *q as i64
                } else {
                    ptr += 1
                }
            }
            Instruction::Jio(r, q) => {
                if state.get(r).is_some_and(|v| *v == 1) {
                    ptr += *q as i64
                } else {
                    ptr += 1
                }
            }
        }
    }

    *state.get(&'b').unwrap()
}
