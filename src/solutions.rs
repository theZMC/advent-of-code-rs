use std::collections::BTreeMap;

use crate::{year2015, year2024, Solution};

pub fn all() -> BTreeMap<i32, BTreeMap<i32, Solution>> {
    BTreeMap::from([
        (
            2015,
            BTreeMap::from([
                (1, year2015::day01::solve as Solution),
                (2, year2015::day02::solve as Solution),
                (3, year2015::day03::solve as Solution),
                (4, year2015::day04::solve as Solution),
                (5, year2015::day05::solve as Solution),
                (6, year2015::day06::solve as Solution),
                (7, year2015::day07::solve as Solution),
                (8, year2015::day08::solve as Solution),
                (9, year2015::day09::solve as Solution),
                (10, year2015::day10::solve as Solution),
                (11, year2015::day11::solve as Solution),
                (12, year2015::day12::solve as Solution),
                (13, year2015::day13::solve as Solution),
                (14, year2015::day14::solve as Solution),
                (15, year2015::day15::solve as Solution),
                (16, year2015::day16::solve as Solution),
                (17, year2015::day17::solve as Solution),
                (18, year2015::day18::solve as Solution),
                (19, year2015::day19::solve as Solution),
            ]),
        ),
        (
            2024,
            BTreeMap::from([
                (1, year2024::day01::solve as Solution),
                (2, year2024::day02::solve as Solution),
            ]),
        ),
    ])
}
