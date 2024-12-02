use reqwest::blocking::Client;
use std::error::Error;

fn format_url(year: i32, day: u32) -> String {
    format!("https://adventofcode.com/{}/day/{}/input", year, day)
}

pub struct Fetcher {
    client: Client,
    token: String,
}

impl Fetcher {
    pub fn new(token: String) -> Self {
        let client = Client::new();

        Self { client, token }
    }

    pub fn fetch_challenge(&self, year: i32, day: u32) -> Result<String, Box<dyn Error>> {
        Ok(self
            .client
            .get(format_url(year, day))
            .header("Cookie", format!("session={}", self.token))
            .send()?
            .text()?
            .trim()
            .to_string())
    }
}
