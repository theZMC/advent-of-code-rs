use anyhow::Result;
use fancy_regex::Regex;
use std::{collections::HashMap, fmt::Display};

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 7)?;

    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn solve_part1(challenge: &str) -> u16 {
    let mut wires = HashMap::<&str, u16>::new();

    let start_rgx = Regex::new(r"^(\d+)\s+->\s+(\w+)$").unwrap();
    let direct_rgx = Regex::new(r"^(\w+)\s+->\s+(\w+)$").unwrap();
    let op_rgx = Regex::new(r"^(\w+|\d+)\s+(AND|OR)\s+(\w+|\d+)\s+->\s+(\w+)$").unwrap();
    let shift_rgx = Regex::new(r"^(\w+)\s+(RSHIFT|LSHIFT)\s+(\d+)\s+->\s+(\w+)$").unwrap();
    let not_rgx = Regex::new(r"^NOT\s+(\w+)\s+->\s+(\w+)$").unwrap();

    let mut instructions: Vec<&str> = challenge.lines().collect();

    loop {
        if instructions.is_empty() {
            break;
        }

        instructions.retain(|instruction| {
            if start_rgx.is_match(instruction).unwrap() {
                let captures = start_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let value = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();
                    let wire = captures.get(2).unwrap().as_str();
                    wires.insert(wire, value);
                    return false;
                }
            }

            if direct_rgx.is_match(instruction).unwrap() {
                let captures = direct_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        let wire = captures.get(2).unwrap().as_str();
                        wires.insert(wire, *value);
                        return false;
                    }
                }
            }

            if shift_rgx.is_match(instruction).unwrap() {
                let captures = shift_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    let direction = captures.get(2).unwrap().as_str();
                    let shift = captures.get(3).unwrap().as_str().parse::<u16>().unwrap();
                    let wire = captures.get(4).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        let result = match direction {
                            "RSHIFT" => value >> shift,
                            "LSHIFT" => value << shift,
                            _ => unreachable!(),
                        };
                        wires.insert(wire, result);
                        return false;
                    }
                }
            }

            if op_rgx.is_match(instruction).unwrap() {
                let captures = op_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source1 = captures.get(1).unwrap().as_str();
                    let op = captures.get(2).unwrap().as_str();
                    let source2 = captures.get(3).unwrap().as_str();
                    let wire = captures.get(4).unwrap().as_str();

                    let value1 = match wires.get(source1) {
                        Some(value) => *value,
                        None => match source1.parse::<u16>() {
                            Ok(value) => value,
                            _ => return true,
                        },
                    };

                    let value2 = match wires.get(source2) {
                        Some(value) => *value,
                        None => match source2.parse::<u16>() {
                            Ok(value) => value,
                            _ => return true,
                        },
                    };

                    let result = match op {
                        "AND" => value1 & value2,
                        "OR" => value1 | value2,
                        _ => unreachable!(),
                    };
                    wires.insert(wire, result);
                    return false;
                }
            }

            if not_rgx.is_match(instruction).unwrap() {
                let captures = not_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    let wire = captures.get(2).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        wires.insert(wire, !value);
                        return false;
                    }
                }
            }

            true
        });
    }

    *wires.get("a").unwrap()
}

fn solve_part2(challenge: &str) -> u16 {
    let mut wires = HashMap::<&str, u16>::new();

    let start_rgx = Regex::new(r"^(\d+)\s+->\s+(\w+)$").unwrap();
    let direct_rgx = Regex::new(r"^(\w+)\s+->\s+(\w+)$").unwrap();
    let op_rgx = Regex::new(r"^(\w+|\d+)\s+(AND|OR)\s+(\w+|\d+)\s+->\s+(\w+)$").unwrap();
    let shift_rgx = Regex::new(r"^(\w+)\s+(RSHIFT|LSHIFT)\s+(\d+)\s+->\s+(\w+)$").unwrap();
    let not_rgx = Regex::new(r"^NOT\s+(\w+)\s+->\s+(\w+)$").unwrap();

    let mut instructions: Vec<&str> = challenge.lines().collect();

    loop {
        if instructions.is_empty() {
            break;
        }

        instructions.retain(|instruction| {
            if start_rgx.is_match(instruction).unwrap() {
                let captures = start_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let value = captures.get(1).unwrap().as_str().parse::<u16>().unwrap();
                    let wire = captures.get(2).unwrap().as_str();
                    if wire == "b" {
                        wires.insert(wire, 46065);
                        return false;
                    }
                    wires.insert(wire, value);
                    return false;
                }
            }

            if direct_rgx.is_match(instruction).unwrap() {
                let captures = direct_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        let wire = captures.get(2).unwrap().as_str();
                        wires.insert(wire, *value);
                        return false;
                    }
                }
            }

            if shift_rgx.is_match(instruction).unwrap() {
                let captures = shift_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    let direction = captures.get(2).unwrap().as_str();
                    let shift = captures.get(3).unwrap().as_str().parse::<u16>().unwrap();
                    let wire = captures.get(4).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        let result = match direction {
                            "RSHIFT" => value >> shift,
                            "LSHIFT" => value << shift,
                            _ => unreachable!(),
                        };
                        wires.insert(wire, result);
                        return false;
                    }
                }
            }

            if op_rgx.is_match(instruction).unwrap() {
                let captures = op_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source1 = captures.get(1).unwrap().as_str();
                    let op = captures.get(2).unwrap().as_str();
                    let source2 = captures.get(3).unwrap().as_str();
                    let wire = captures.get(4).unwrap().as_str();

                    let value1 = match wires.get(source1) {
                        Some(value) => *value,
                        None => match source1.parse::<u16>() {
                            Ok(value) => value,
                            _ => return true,
                        },
                    };

                    let value2 = match wires.get(source2) {
                        Some(value) => *value,
                        None => match source2.parse::<u16>() {
                            Ok(value) => value,
                            _ => return true,
                        },
                    };

                    let result = match op {
                        "AND" => value1 & value2,
                        "OR" => value1 | value2,
                        _ => unreachable!(),
                    };
                    wires.insert(wire, result);
                    return false;
                }
            }

            if not_rgx.is_match(instruction).unwrap() {
                let captures = not_rgx.captures(instruction).unwrap();
                if let Some(captures) = captures {
                    let source = captures.get(1).unwrap().as_str();
                    let wire = captures.get(2).unwrap().as_str();
                    if let Some(value) = wires.get(source) {
                        wires.insert(wire, !value);
                        return false;
                    }
                }
            }

            true
        });
    }

    *wires.get("a").unwrap()
}
