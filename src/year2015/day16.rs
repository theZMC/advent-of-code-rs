use std::fmt::Display;

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 16)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

struct Aunt {
    pub id: i32,
    pub children: Option<i32>,
    pub cats: Option<i32>,
    pub samoyeds: Option<i32>,
    pub pomeranians: Option<i32>,
    pub akitas: Option<i32>,
    pub vizslas: Option<i32>,
    pub goldfish: Option<i32>,
    pub trees: Option<i32>,
    pub cars: Option<i32>,
    pub perfumes: Option<i32>,
}

impl Aunt {
    fn new(id: i32) -> Self {
        Self {
            id,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn from_string(s: &str) -> Self {
        let aunt_number = s.split_whitespace().nth(1).unwrap().trim_end_matches(':');
        let mut aunt = Aunt::new(aunt_number.parse().unwrap());
        for (k, v) in s
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.split(": "))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        {
            match k {
                "children" => aunt.children = Some(v.parse().unwrap()),
                "cats" => aunt.cats = Some(v.parse().unwrap()),
                "samoyeds" => aunt.samoyeds = Some(v.parse().unwrap()),
                "pomeranians" => aunt.pomeranians = Some(v.parse().unwrap()),
                "akitas" => aunt.akitas = Some(v.parse().unwrap()),
                "vizslas" => aunt.vizslas = Some(v.parse().unwrap()),
                "goldfish" => aunt.goldfish = Some(v.parse().unwrap()),
                "trees" => aunt.trees = Some(v.parse().unwrap()),
                "cars" => aunt.cars = Some(v.parse().unwrap()),
                "perfumes" => aunt.perfumes = Some(v.parse().unwrap()),
                _ => panic!("unexpected key"),
            }
        }
        aunt
    }
}

fn parse_aunts(challenge: &str) -> Vec<Aunt> {
    challenge.lines().map(Aunt::from_string).collect()
}

fn solve_part1(challenge: &str) -> String {
    parse_aunts(challenge)
        .iter()
        .find(|aunt| {
            let criteria = &Aunt::from_string("Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1");
            [
                match aunt.children {
                    Some(children) => children == criteria.children.unwrap(),
                    None => true,
                },
                match aunt.cats {
                    Some(cats) => cats == criteria.cats.unwrap(),
                    None => true,
                },
                match aunt.samoyeds {
                    Some(samoyeds) => samoyeds == criteria.samoyeds.unwrap(),
                    None => true,
                },
                match aunt.pomeranians {
                    Some(pomeranians) => pomeranians == criteria.pomeranians.unwrap(),
                    None => true,
                },
                match aunt.akitas {
                    Some(akitas) => akitas == criteria.akitas.unwrap(),
                    None => true,
                },
                match aunt.vizslas {
                    Some(vizslas) => vizslas == criteria.vizslas.unwrap(),
                    None => true,
                },
                match aunt.goldfish {
                    Some(goldfish) => goldfish == criteria.goldfish.unwrap(),
                    None => true,
                },
                match aunt.trees {
                    Some(trees) => trees == criteria.trees.unwrap(),
                    None => true,
                },
                match aunt.cars {
                    Some(cars) => cars == criteria.cars.unwrap(),
                    None => true,
                },
                match aunt.perfumes {
                    Some(perfumes) => perfumes == criteria.perfumes.unwrap(),
                    None => true,
                },
            ].iter().all(|&x| x)
        })
        .unwrap()
        .id
        .to_string()
}

fn solve_part2(challenge: &str) -> String {
    parse_aunts(challenge)
        .iter()
        .find(|aunt| {
            let criteria = &Aunt::from_string("Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1");
            [
                match aunt.children {
                    Some(children) => children == criteria.children.unwrap(),
                    None => true,
                },
                match aunt.cats {
                    Some(cats) => cats > criteria.cats.unwrap(),
                    None => true,
                },
                match aunt.samoyeds {
                    Some(samoyeds) => samoyeds == criteria.samoyeds.unwrap(),
                    None => true,
                },
                match aunt.pomeranians {
                    Some(pomeranians) => pomeranians < criteria.pomeranians.unwrap(),
                    None => true,
                },
                match aunt.akitas {
                    Some(akitas) => akitas == criteria.akitas.unwrap(),
                    None => true,
                },
                match aunt.vizslas {
                    Some(vizslas) => vizslas == criteria.vizslas.unwrap(),
                    None => true,
                },
                match aunt.goldfish {
                    Some(goldfish) => goldfish < criteria.goldfish.unwrap(),
                    None => true,
                },
                match aunt.trees {
                    Some(trees) => trees > criteria.trees.unwrap(),
                    None => true,
                },
                match aunt.cars {
                    Some(cars) => cars == criteria.cars.unwrap(),
                    None => true,
                },
                match aunt.perfumes {
                    Some(perfumes) => perfumes == criteria.perfumes.unwrap(),
                    None => true,
                },
            ].iter().all(|&x| x)
        })
        .unwrap()
        .id
        .to_string()
}
