use std::collections::HashMap;

mod fetcher;
mod year2015;

fn main() {
    let solutions = HashMap::from([(2015, [year2015::day01::solve, year2015::day02::solve])]);
    for (year, solvers) in solutions {
        for i in 1..=solvers.len() {
            match fetcher::fetch_challenge(i as u32, year) {
                Ok(challenge) => {
                    let (first, second) = solvers[i - 1](&challenge);
                    println!("Year {}, Day {}: {}, {}", year, i, first, second);
                }
                Err(e) => eprintln!("Error fetching challenge: {}", e),
            }
        }
    }
}
