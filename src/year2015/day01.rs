use std::error::Error;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(i32, i32), Box<dyn Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 1)?;

    let mut floor: i32 = 0;
    let mut instruction_count = 1;
    let mut negative_floor_instruction = 0;

    challenge.chars().for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if negative_floor_instruction == 0 && floor == -1 {
            negative_floor_instruction = instruction_count;
        }

        instruction_count += 1;
    });

    Ok((floor, negative_floor_instruction))
}
