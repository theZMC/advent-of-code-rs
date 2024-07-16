use reqwest::blocking::Client;
use std::error::Error;

pub fn fetch_challenge(day: u32, year: u32) -> Result<String, Box<dyn Error>> {
    let default_session = "53616c7465645f5fba9ed0fc541aa1e2856119f1ad635de3c5cf043ee565d20e2f5e395b2833c86ebd674afaf9f88f803c2205e3a2d726289ff87b2a0ac89952".to_owned();
    let aoc_session = std::env::var("AOC_SESSION").unwrap_or(default_session);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = Client::new();

    Ok(client
        .get(url)
        .header("Cookie", format!("session={}", aoc_session))
        .send()?
        .text()?)
}
