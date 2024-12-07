use std::{collections::HashSet, fmt::Display};

use anyhow::Result;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 6)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

type Position = (i32, i32);

#[derive(Debug)]
struct RestrictedArea {
    obstacles: HashSet<Position>,
    guard_pos: Position,
    bounds: (i32, i32),
    guard: char,
}

impl RestrictedArea {
    fn new() -> RestrictedArea {
        RestrictedArea {
            obstacles: HashSet::new(),
            guard_pos: (0, 0),
            bounds: (0, 0),
            guard: '-',
        }
    }
}

fn parse(challenge: &str) -> RestrictedArea {
    challenge
        .lines()
        .enumerate()
        .fold(RestrictedArea::new(), |mut ra, (y, line)| {
            ra.bounds.1 = ra.bounds.1.max(y as i32);
            line.chars().enumerate().for_each(|(x, c)| {
                ra.bounds.0 = ra.bounds.0.max(x as i32);
                match c {
                    '#' => _ = ra.obstacles.insert((x as i32, y as i32)),
                    '<' | '^' | 'v' | '>' => {
                        ra.guard_pos = (x as i32, y as i32);
                        ra.guard = c;
                    }
                    _ => (),
                }
            });
            ra
        })
}

fn in_bounds(g: (i32, i32), b: (i32, i32)) -> bool {
    g.0 <= b.0 && g.1 <= b.1 && g.0 >= 0 && g.1 >= 0
}

fn turn_guard(guard: char) -> char {
    match guard {
        '<' => '^',
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        _ => guard,
    }
}

fn next_pos(guard_pos: (i32, i32), guard: char) -> (i32, i32) {
    match guard {
        '<' => (guard_pos.0 - 1, guard_pos.1),
        '>' => (guard_pos.0 + 1, guard_pos.1),
        '^' => (guard_pos.0, guard_pos.1 - 1),
        'v' => (guard_pos.0, guard_pos.1 + 1),
        _ => guard_pos,
    }
}

fn solve_part1(challenge: &str) -> usize {
    let ra = parse(challenge);
    let mut guard_pos = ra.guard_pos;
    let mut guard = ra.guard;
    let mut visited = HashSet::new();

    while in_bounds(guard_pos, ra.bounds) {
        visited.insert(guard_pos);
        let next = next_pos(guard_pos, guard);

        if ra.obstacles.contains(&next) {
            guard = turn_guard(guard);
            continue;
        }

        guard_pos = next;
    }

    visited.len()
}

fn would_loop(
    bounds: (i32, i32),
    test_pos: (i32, i32),
    mut guard_pos: (i32, i32),
    mut guard: char,
    mut obstacles: HashSet<(i32, i32)>,
) -> bool {
    if !in_bounds(test_pos, bounds) {
        return false;
    }
    obstacles.insert(test_pos);

    let mut visited = HashSet::new();

    while in_bounds(guard_pos, bounds) {
        if !visited.insert((guard_pos, guard)) {
            return true;
        }

        let next = next_pos(guard_pos, guard);

        if obstacles.contains(&next) {
            guard = turn_guard(guard);
            continue;
        }

        guard_pos = next;
    }
    false
}

fn solve_part2(challenge: &str) -> usize {
    let ra = parse(challenge);

    let mut guard_pos = ra.guard_pos;
    let mut guard = ra.guard;
    let mut visited = HashSet::new();
    let mut loop_locations = HashSet::new();

    let mut first_move = None;

    while in_bounds(guard_pos, ra.bounds) {
        visited.insert(guard_pos);

        let next = next_pos(guard_pos, guard);

        if first_move.is_none() {
            first_move = Some(next);
        }

        if ra.obstacles.contains(&next) {
            guard = turn_guard(guard);
            continue;
        }

        if first_move.is_some_and(|m| m != next)
            && next != ra.guard_pos
            && would_loop(
                ra.bounds,
                next,
                ra.guard_pos,
                ra.guard,
                ra.obstacles.clone(),
            )
        {
            loop_locations.insert(next);
        }

        guard_pos = next;
    }

    loop_locations.len()
}
