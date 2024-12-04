use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 20)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn num_presents_part1(n: usize) -> usize {
    (1..=(n as f64).sqrt() as usize)
        .flat_map(|i| if n % i == 0 { vec![i, n / i] } else { vec![] })
        .map(|i| 10 * i)
        .sum::<usize>()
}

fn solve_part1(challenge: &str) -> String {
    (2..)
        .find(|&i| num_presents_part1(i) >= challenge.trim().parse::<usize>().unwrap())
        .unwrap()
        .to_string()
}

fn num_presents_part2(n: usize) -> usize {
    (1..=(n as f64).sqrt() as usize)
        .filter(|&i| n % i == 0)
        .flat_map(|i| {
            let mut divisors = Vec::new();
            if n / i <= 50 {
                divisors.push(i);
            }
            if i != n / i && (n / (n / i)) <= 50 {
                divisors.push(n / i);
            }
            divisors
        })
        .map(|i| 11 * i)
        .sum::<usize>()
}

fn solve_part2(challenge: &str) -> String {
    (2..)
        .find(|&i| num_presents_part2(i) >= challenge.trim().parse::<usize>().unwrap())
        .unwrap()
        .to_string()
}
