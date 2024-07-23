use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    io::Write,
};

mod challenge;
mod menu;
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
            (6, year2015::day06::solve as Solution),
        ]),
    )]);

    let choices = solutions
        .iter()
        .map(|(year, days)| (*year, BTreeSet::from_iter(days.keys().copied())))
        .collect();

    print!("{}", menu::render((2015, 1), &choices));

    let (selected_year, selected_day) = user_selection(&choices).unwrap();

    match solutions.get(&selected_year).unwrap().get(&selected_day) {
        Some(solution) => {
            println!("Solving Year {}, Day {}...", selected_year, selected_day);

            let fetcher = challenge::Fetcher::new();
            match solution(&fetcher) {
                Ok((first, second)) => {
                    println!(
                        "Year {}, Day {:>02}: {}, {}",
                        selected_year, selected_day, first, second
                    )
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

fn user_selection(choices: &BTreeMap<i32, BTreeSet<i32>>) -> Result<(i32, i32), Box<dyn Error>> {
    let mut selected_year = *choices.keys().max().unwrap();
    let mut selected_day = *choices.get(&selected_year).unwrap().iter().max().unwrap();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    stdout.execute(cursor::Hide)?;

    let mut selecting_days = true;
    loop {
        stdout.execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        print!("{}", menu::render((selected_year, selected_day), choices));
        stdout.flush()?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Esc => {
                    if selecting_days {
                        selecting_days = false;
                    } else {
                        break;
                    }
                }
                KeyCode::Enter => {
                    if selecting_days {
                        break;
                    } else {
                        selecting_days = true;
                    }
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    if selecting_days && choices.contains_key(&(selected_day - 10)) {
                        selected_day -= 10;
                    } else {
                        selected_day = *choices.get(&selected_year).unwrap().iter().min().unwrap();
                    }
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    if selecting_days && choices.contains_key(&(selected_day + 10)) {
                        selected_day += 10;
                    } else {
                        selected_day = *choices.get(&selected_year).unwrap().iter().max().unwrap();
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if selecting_days {
                        if choices
                            .get(&selected_year)
                            .unwrap()
                            .contains(&(selected_day - 1))
                        {
                            selected_day -= 1;
                        }
                    } else if choices.contains_key(&(selected_year - 1)) {
                        selected_year -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if selecting_days {
                        if choices
                            .get(&selected_year)
                            .unwrap()
                            .contains(&(selected_day + 1))
                        {
                            selected_day += 1;
                        }
                    } else if choices.contains_key(&(selected_year + 1)) {
                        selected_year += 1;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok((selected_year, selected_day))
}
