use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fmt::Write,
    io::{stdout, Write as _},
};

use crossterm::{
    cursor::{self, Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType::All},
    ExecutableCommand,
};

#[derive(Clone, Copy)]
enum SelectType {
    Year,
    Day,
}

fn render(
    selected: (i32, i32),
    choices: &BTreeMap<i32, BTreeSet<i32>>,
    select_type: SelectType,
) -> String {
    let (selected_year, selected_day) = selected;
    let mut output = String::new();

    writeln!(&mut output, "┏━━━━━━━━━━━━━━━━━━━━┓\r").unwrap();
    match select_type {
        SelectType::Year => {
            writeln!(&mut output, "┃    Choose Year     ┃\r").unwrap();
        }
        SelectType::Day => {
            writeln!(&mut output, "┃     Choose Day     ┃\r").unwrap();
        }
    }
    writeln!(&mut output, "┗━━━━━━┳━━━━━━━━━━━━━┛\r").unwrap();

    for i in 1..=choices.len().max(10) {
        let mut line = String::new();

        match choices.keys().nth(i - 1) {
            Some(year) => {
                if *year == selected_year {
                    write!(&mut line, " {}", &color_bg_white(&format!(" {} ", year))).unwrap();
                } else {
                    write!(&mut line, "  {} ", year).unwrap();
                }
            }
            None => {
                write!(&mut line, "       ").unwrap();
            }
        }

        line += "┃";

        if i > 10 {
            continue;
        }

        for x in [0, 10, 20].iter() {
            if *x == 20 && i > 5 {
                break;
            }

            if i + x == selected_day as usize {
                match select_type {
                    SelectType::Year => {
                        write!(&mut line, " {:02} ", i + x).unwrap();
                    }
                    SelectType::Day => {
                        write!(&mut line, "{}", &color_bg_white(&format!(" {:02} ", i + x)))
                            .unwrap();
                    }
                }
            } else if choices
                .get(&selected_year)
                .unwrap()
                .contains(&((i + x) as i32))
            {
                write!(&mut line, " {:02} ", i + x).unwrap();
            } else {
                write!(&mut line, "{}", color_gray(&format!(" {:02} ", i + x))).unwrap();
            }
        }

        writeln!(&mut output, "{}\r", line).unwrap();
    }

    output
}

pub fn select(choices: &BTreeMap<i32, BTreeSet<i32>>) -> Result<(i32, i32), Box<dyn Error>> {
    let mut selected_year = *choices.keys().max().unwrap();
    let mut selected_day = *choices.get(&selected_year).unwrap().iter().max().unwrap();

    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(Hide)?;

    let mut select_type = SelectType::Year;
    loop {
        stdout.execute(Clear(All))?;
        stdout.execute(MoveTo(0, 0))?;

        if !choices.get(&selected_year).unwrap().contains(&selected_day) {
            selected_day = *choices.get(&selected_year).unwrap().iter().max().unwrap();
        }

        print!(
            "{}",
            render((selected_year, selected_day), choices, select_type)
        );
        stdout.flush()?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    stdout.execute(cursor::Show)?;
                    return Ok((0, 0));
                }
                KeyCode::Esc => match select_type {
                    SelectType::Year => {
                        continue;
                    }
                    SelectType::Day => {
                        select_type = SelectType::Year;
                    }
                },
                KeyCode::Enter => match select_type {
                    SelectType::Year => {
                        if choices.contains_key(&selected_year) {
                            select_type = SelectType::Day;
                            selected_day =
                                *choices.get(&selected_year).unwrap().iter().max().unwrap();
                        }
                    }
                    SelectType::Day => {
                        break;
                    }
                },
                KeyCode::Left | KeyCode::Char('h') => match select_type {
                    SelectType::Year => {
                        if let Some((year, _)) = choices.range(..selected_year).next_back() {
                            selected_year = *year;
                        }
                    }
                    SelectType::Day => {
                        if choices.contains_key(&(selected_day - 10)) {
                            selected_day -= 10;
                        } else {
                            selected_day =
                                *choices.get(&selected_year).unwrap().iter().min().unwrap();
                        }
                    }
                },
                KeyCode::Right | KeyCode::Char('l') => match select_type {
                    SelectType::Year => {
                        if let Some((year, _)) = choices.range(selected_year + 1..).next() {
                            selected_year = *year;
                        }
                    }
                    SelectType::Day => {
                        if choices.contains_key(&(selected_day + 10)) {
                            selected_day += 10;
                        } else {
                            selected_day =
                                *choices.get(&selected_year).unwrap().iter().max().unwrap();
                        }
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => match select_type {
                    SelectType::Year => {
                        if let Some((year, _)) = choices.range(..selected_year).next_back() {
                            selected_year = *year;
                        }
                    }
                    SelectType::Day => {
                        if choices
                            .get(&selected_year)
                            .unwrap()
                            .contains(&(selected_day - 1))
                        {
                            selected_day -= 1;
                        }
                    }
                },
                KeyCode::Down | KeyCode::Char('j') => match select_type {
                    SelectType::Year => {
                        if let Some((year, _)) = choices.range(selected_year + 1..).next() {
                            selected_year = *year;
                        }
                    }
                    SelectType::Day => {
                        if choices
                            .get(&selected_year)
                            .unwrap()
                            .contains(&(selected_day + 1))
                        {
                            selected_day += 1;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok((selected_year, selected_day))
}

fn color_gray(s: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", s)
}

fn color_bg_white(s: &str) -> String {
    format!("\x1b[30;47m{}\x1b[0m", s)
}
