use std::fmt::Display;

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 18)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

struct LightGrid {
    grid: Vec<Vec<bool>>,
}

impl LightGrid {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        Self { grid }
    }

    fn from_string(s: &str) -> Self {
        Self::new(
            s.lines()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect(),
        )
    }

    fn step_part1(&mut self) {
        let mut new_grid = self.grid.clone();

        self.grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &cell)| {
                let neighbors_on = self.count_neighbors_on_part1(i as isize, j as isize);
                new_grid[i][j] = matches!((cell, neighbors_on), (true, 2) | (true, 3) | (false, 3));
            });
        });

        self.grid = new_grid;
    }

    fn step_part2(&mut self) {
        let mut new_grid = self.grid.clone();

        self.grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &cell)| {
                if is_corner(i, j, self.grid.len(), self.grid[0].len()) {
                    return;
                }
                let neighbors_on = self.count_neighbors_on_part2(i as isize, j as isize);
                new_grid[i][j] = matches!((cell, neighbors_on), (true, 2) | (true, 3) | (false, 3));
            });
        });

        self.grid = new_grid;
    }

    fn count_neighbors_on_part1(&self, x: isize, y: isize) -> usize {
        let mut count = 0;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                if i == x && j == y {
                    continue;
                }

                if is_corner(i as usize, j as usize, self.grid.len(), self.grid[0].len()) {
                    continue;
                }

                if self
                    .grid
                    .get(i as usize)
                    .and_then(|row| row.get(j as usize))
                    == Some(&true)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_neighbors_on_part2(&self, x: isize, y: isize) -> usize {
        let mut count = 0;
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                if i == x && j == y {
                    continue;
                }

                if self
                    .grid
                    .get(i as usize)
                    .and_then(|row| row.get(j as usize))
                    == Some(&true)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_on(&self) -> usize {
        self.grid.iter().flatten().filter(|&&x| x).count()
    }
}

fn is_corner(x: usize, y: usize, xlen: usize, ylen: usize) -> bool {
    (x == 0 || x == xlen - 1) && (y == 0 || y == ylen - 1)
}

fn solve_part1(challenge: &str) -> String {
    let mut grid = LightGrid::from_string(challenge);

    for _ in 0..100 {
        grid.step_part1();
    }

    grid.count_on().to_string()
}

fn solve_part2(challenge: &str) -> String {
    let mut grid = LightGrid::from_string(challenge);

    for _ in 0..100 {
        grid.step_part2();
    }

    grid.count_on().to_string()
}
