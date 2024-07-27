use serde_json::Value;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(String, String), Box<dyn std::error::Error>> {
    let challenge = fetcher.fetch_challenge(YEAR, 12)?;
    Ok((solve_part1(&challenge), solve_part2(&challenge)))
}

fn solve_part1(challenge: &str) -> String {
    fn f(v: &Value) -> i32 {
        match v {
            Value::Object(o) => o.values().map(|v| -> i32 { f(v) }).sum(),
            Value::Number(n) => n.as_i64().unwrap() as i32,
            Value::Array(a) => a.iter().map(|v| -> i32 { f(v) }).sum(),
            _ => 0,
        }
    }
    f(&serde_json::from_str(challenge).unwrap()).to_string()
}

fn solve_part2(challenge: &str) -> String {
    fn f(v: &Value) -> i32 {
        match v {
            Value::Object(o) => {
                if o.values().any(|v| v == "red") {
                    0
                } else {
                    o.values().map(|v| -> i32 { f(v) }).sum()
                }
            }
            Value::Number(n) => n.as_i64().unwrap() as i32,
            Value::Array(a) => a.iter().map(|v| -> i32 { f(v) }).sum(),
            _ => 0,
        }
    }
    f(&serde_json::from_str(challenge).unwrap()).to_string()
}
