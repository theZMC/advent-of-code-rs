use std::fmt::Display;

use anyhow::Result;

use crate::challenge::Fetcher;

use super::YEAR;

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 4)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

trait WordSearch {
    fn find_all(&self, word: &str) -> Vec<(i32, i32)>;
}

trait XWordSearch {
    fn find_all_x(&self, word: &str) -> Vec<(i32, i32)>;
}

impl WordSearch for Vec<Vec<char>> {
    fn find_all(&self, word: &str) -> Vec<(i32, i32)> {
        let row_size = self.len() as i32;
        let col_size = self[0].len() as i32;
        let directions = [
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, 0),
            (0, -1),
            (-1, -1),
        ];
        let in_bounds = |row_pos, col_pos| {
            row_pos >= 0 && row_pos < row_size && col_pos >= 0 && col_pos < col_size
        };
        let chars = || word.chars().collect::<Vec<_>>();

        self.iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(_, &c)| c == chars()[0])
                    .map(move |(c, _)| (r, c))
            })
            .flat_map(|(row_pos, col_pos)| {
                directions
                    .iter()
                    .filter(move |(row_translation, col_translation)| {
                        (0..word.len()).all(|i| {
                            in_bounds(
                                row_pos as i32 + i as i32 * row_translation,
                                col_pos as i32 + i as i32 * col_translation,
                            )
                        })
                    })
                    .filter(move |(row_translation, col_translation)| {
                        (0..word.len()).all(|i| {
                            let this_row_pos =
                                (row_pos as i32 + i as i32 * row_translation) as usize;
                            let this_col_pos =
                                (col_pos as i32 + i as i32 * col_translation) as usize;

                            self[this_row_pos][this_col_pos] == chars()[i]
                        })
                    })
            })
            .map(|(x, y)| (*x, *y))
            .collect()
    }
}

impl XWordSearch for Vec<Vec<char>> {
    fn find_all_x(&self, word: &str) -> Vec<(i32, i32)> {
        if word.len() % 2 == 0 {
            return vec![];
        }
        let axes = [((-1, -1), (1, 1)), ((-1, 1), (1, -1))];
        let span_size = word.len() / 2;
        let center = word.chars().collect::<Vec<_>>()[span_size];
        let row_size = self.len();
        let col_size = self[0].len();
        let in_bounds = |row_pos, col_pos| {
            row_pos >= 0 && row_pos < row_size as i32 && col_pos >= 0 && col_pos < col_size as i32
        };

        self.iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |(_, &c)| c == center)
                    .map(move |(c, _)| (r, c))
            })
            .filter(move |(row_pos, col_pos)| {
                axes.iter().all(|axis| {
                    [axis.0, axis.1]
                        .iter()
                        .all(|(row_translation, col_translation)| {
                            (0..=span_size).all(|i| {
                                in_bounds(
                                    *row_pos as i32 + i as i32 * *row_translation,
                                    *col_pos as i32 + i as i32 * *col_translation,
                                )
                            })
                        })
                })
            })
            .filter(move |(row_pos, col_pos)| {
                axes.iter().all(|axis| {
                    let check_word = (0..=span_size)
                        .fold(vec![center], |mut acc, i| {
                            if i == 0 {
                                return acc;
                            }
                            let forward_char_pos = (
                                *row_pos as i32 + i as i32 * axis.0 .0,
                                *col_pos as i32 + i as i32 * axis.0 .1,
                            );
                            let backward_char_pos = (
                                *row_pos as i32 + i as i32 * axis.1 .0,
                                *col_pos as i32 + i as i32 * axis.1 .1,
                            );
                            let forward_char =
                                self[forward_char_pos.0 as usize][forward_char_pos.1 as usize];
                            let backward_char =
                                self[backward_char_pos.0 as usize][backward_char_pos.1 as usize];
                            acc.push(forward_char);
                            acc.insert(0, backward_char);
                            acc
                        })
                        .iter()
                        .collect::<String>();

                    check_word == word || check_word.chars().rev().collect::<String>() == word
                })
            })
            .map(|(x, y)| (x as i32, y as i32))
            .collect()
    }
}

fn parse_word_search(challenge: &str) -> Vec<Vec<char>> {
    challenge
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve_part1(challenge: &str) -> String {
    parse_word_search(challenge)
        .find_all("XMAS")
        .len()
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    parse_word_search(challenge)
        .find_all_x("MAS")
        .len()
        .to_string()
}
