use reqwest::blocking::Client;
use std::error::Error;

const DEFAULT_SESSION: &str = "53616c7465645f5fba9ed0fc541aa1e2856119f1ad635de3c5cf043ee565d20e2f5e395b2833c86ebd674afaf9f88f803c2205e3a2d726289ff87b2a0ac89952";

fn format_url(year: i32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

pub struct Fetcher {
    client: Client,
    session: String,
}

impl Fetcher {
    pub fn new() -> Self {
        let client = Client::new();
        let session = std::env::var("AOC_SESSION").unwrap_or(DEFAULT_SESSION.to_owned());

        Self { client, session }
    }

    pub fn fetch_challenge(&self, year: i32, day: u32) -> Result<String, Box<dyn Error>> {
        Ok(self
            .client
            .get(format_url(year, day))
            .header("Cookie", format!("session={}", self.session))
            .send()?
            .text()?)
    }
}
