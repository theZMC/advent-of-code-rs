use std::fmt::Display;

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 11)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

fn solve_part1(challenge: &str) -> String {
    find_next_password(challenge)
}

fn solve_part2(challenge: &str) -> String {
    find_next_password(base26_increment(find_next_password(challenge).as_str()).as_str())
}

fn find_next_password(s: &str) -> String {
    let mut new_password = s.to_string();
    while !is_valid_password(&new_password) {
        new_password = base26_increment(&new_password);
    }
    new_password
}

fn base26_increment(s: &str) -> String {
    let mut result = String::new();
    let mut carry = true;
    for c in s.chars().rev() {
        if carry {
            if c == 'z' {
                result.push('a');
            } else if c == 'h' || c == 'k' || c == 'n' {
                // i, l, and o are invalid characters so we skip them
                result.push((c as u8 + 2) as char);
                carry = false;
            } else {
                result.push((c as u8 + 1) as char);
                carry = false;
            }
        } else {
            result.push(c);
        }
    }
    result.chars().rev().collect()
}

fn is_valid_password(s: &str) -> bool {
    let mut has_straight = false;
    let mut pair_count = 0;
    let mut possible_straight: Vec<u8> = Vec::new();
    let mut last_char = '\0';

    for c in s.chars() {
        if pair_count < 2 {
            if last_char == c {
                pair_count += 1;
                last_char = '\0'; // reset to avoid overlapping pairs
            } else {
                last_char = c;
            }
        }
        if !has_straight {
            if possible_straight.is_empty() {
                possible_straight.push(c as u8);
                continue;
            }

            if let Some(&last) = possible_straight.last() {
                if last == c as u8 - 1 {
                    if possible_straight.len() == 2 {
                        has_straight = true;
                    }
                    possible_straight.push(c as u8);
                    continue;
                } else {
                    possible_straight.clear();
                    possible_straight.push(c as u8);
                }
            }
        }
    }

    has_straight && pair_count == 2
}
