mod day01;
mod fetcher;

fn main() {
    let challenge = fetcher::fetch_challenge(1, 2015);
    match challenge {
        Ok(challenge) => {
            let (floor, instruction) = day01::solve(&challenge);
            println!("{}, {}", floor, instruction);
        }
        Err(e) => eprintln!("Error fetching challenge: {}", e),
    }
}
