use std::collections::BTreeMap;

mod challenge;
mod year2015;

type Solution = fn(&challenge::Fetcher) -> Result<(String, String), Box<dyn std::error::Error>>;

fn main() {
    let solutions = BTreeMap::from([(
        2015,
        BTreeMap::from([
            (1, year2015::day01::solve as Solution),
            (2, year2015::day02::solve as Solution),
            (3, year2015::day03::solve as Solution),
            (4, year2015::day04::solve as Solution),
            (5, year2015::day05::solve as Solution),
        ]),
    )]);

    let fetcher = challenge::Fetcher::new();

    for (year, days) in solutions {
        for (day, solution) in days {
            match solution(&fetcher) {
                Ok((first, second)) => {
                    println!("Year {}, Day {:>02}: {}, {}", year, day, first, second)
                }
                Err(e) => eprintln!("Error solving challenge: {}", e),
            }
        }
    }
}
