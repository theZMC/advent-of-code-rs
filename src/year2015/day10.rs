use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 10)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn look_and_say(input: &str, times: usize) -> String {
    if times == 0 {
        return input.to_string();
    }

    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if c == next {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        result.push_str(&count.to_string());
        result.push(c);
    }

    look_and_say(&result, times - 1)
}

fn solve_part1(challenge: &str) -> String {
    look_and_say(challenge, 40).len().to_string()
}

fn solve_part2(challenge: &str) -> String {
    look_and_say(challenge, 50).len().to_string()
}
