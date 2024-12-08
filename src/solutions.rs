use std::collections::BTreeMap;

use crate::{year2015, year2024, Solution};

pub fn all() -> BTreeMap<i32, BTreeMap<i32, Solution>> {
    BTreeMap::from([
        (
            2015,
            BTreeMap::from([
                (1, year2015::day01::solve as Solution),
                (2, year2015::day02::solve),
                (3, year2015::day03::solve),
                (4, year2015::day04::solve),
                (5, year2015::day05::solve),
                (6, year2015::day06::solve),
                (7, year2015::day07::solve),
                (8, year2015::day08::solve),
                (9, year2015::day09::solve),
                (10, year2015::day10::solve),
                (11, year2015::day11::solve),
                (12, year2015::day12::solve),
                (13, year2015::day13::solve),
                (14, year2015::day14::solve),
                (15, year2015::day15::solve),
                (16, year2015::day16::solve),
                (17, year2015::day17::solve),
                (18, year2015::day18::solve),
                (19, year2015::day19::solve),
                (20, year2015::day20::solve),
                (21, year2015::day21::solve),
                (22, year2015::day22::solve),
            ]),
        ),
        (
            2024,
            BTreeMap::from([
                (1, year2024::day01::solve as Solution),
                (2, year2024::day02::solve),
                (3, year2024::day03::solve),
                (4, year2024::day04::solve),
                (5, year2024::day05::solve),
                (6, year2024::day06::solve),
                (7, year2024::day07::solve),
                (8, year2024::day08::solve),
            ]),
        ),
    ])
}
