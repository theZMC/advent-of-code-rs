use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

pub fn render(selected: (i32, i32), choices: &BTreeMap<i32, BTreeSet<i32>>) -> String {
    let (selected_year, selected_day) = selected;
    let mut output = String::new();

    writeln!(&mut output, "----------------------\r").unwrap();
    writeln!(&mut output, "|    Choose a Day    |\r").unwrap();
    writeln!(&mut output, "----------------------\r").unwrap();

    for i in 1..=choices.len().max(10) {
        let mut line = String::new();

        match choices.keys().nth(i - 1) {
            Some(year) => {
                if *year == selected_year {
                    write!(&mut line, "{}", &color_bg_white(&format!(" {} ", year))).unwrap();
                } else {
                    write!(&mut line, "{}", color_gray(&format!(" {} ", year))).unwrap();
                }
            }
            None => {
                write!(&mut line, "      ").unwrap();
            }
        }

        line += " | ";

        if i > 10 {
            continue;
        }

        for x in [0, 10, 20].iter() {
            if *x == 20 && i > 5 {
                break;
            }

            if i + x == selected_day as usize {
                write!(&mut line, "{}", &color_bg_white(&format!(" {:02} ", i + x))).unwrap();
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

fn color_gray(s: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", s)
}

fn color_bg_white(s: &str) -> String {
    format!("\x1b[30;47m{}\x1b[0m", s)
}
