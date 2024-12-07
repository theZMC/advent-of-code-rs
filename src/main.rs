use std::{collections::BTreeSet, fmt::Display};

use anyhow::Result;

mod challenge;
mod conf;
mod menu;
mod solutions;
mod year2015;
mod year2024;

type Solution = fn(&challenge::Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)>;

fn main() {
    let conf = conf::Conf::load_or_create().unwrap();
    let solutions = solutions::all();

    let choices = solutions
        .iter()
        .map(|(year, days)| (*year, BTreeSet::from_iter(days.keys().copied())))
        .collect();

    let (selected_year, selected_day) = menu::select(&choices).unwrap();

    if selected_year == 0 || selected_day == 0 {
        return;
    }

    match solutions.get(&selected_year).unwrap().get(&selected_day) {
        Some(solution) => {
            println!("Solving Year {}, Day {}...", selected_year, selected_day);

            let fetcher = challenge::Fetcher::new(conf.token);
            match solution(&fetcher) {
                Ok((part_one, part_two)) => {
                    println!("Part 1\n{}", part_one);
                    println!("Part 2\n{}", part_two);
                }
                Err(e) => eprintln!("Error solving challenge: {}", e),
            }
        }
        None => eprintln!(
            "No solution found for Year {}, Day {}",
            selected_year, selected_day
        ),
    }
}
