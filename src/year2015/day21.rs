use std::{collections::BTreeSet, fmt::Display};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 21)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Item {
    cost: usize,
    dmg: usize,
    def: usize,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Loadout {
    weapon: Item,
    armor: Option<Item>,
    rings: [Option<Item>; 2],
}

struct Boss {
    hp: usize,
    dmg: usize,
    def: usize,
}

trait Cost {
    fn cost(&self) -> usize;
}

trait Battle {
    fn defeats(&self, boss: &Boss) -> bool;
    fn armor(&self) -> usize;
    fn damage(&self) -> usize;
}

impl Cost for Loadout {
    fn cost(&self) -> usize {
        self.weapon.cost
            + self.armor.map_or(0, |a| a.cost)
            + self.rings.iter().flatten().map(|r| r.cost).sum::<usize>()
    }
}

impl Battle for Loadout {
    fn defeats(&self, boss: &Boss) -> bool {
        let damage = self.damage().saturating_sub(boss.def).max(1);
        let boss_damage = boss.dmg.saturating_sub(self.armor()).max(1);

        let turns = (boss.hp + damage - 1) / damage;
        let boss_turns = (100 + boss_damage - 1) / boss_damage;

        turns <= boss_turns
    }

    fn armor(&self) -> usize {
        self.armor.map_or(0, |a| a.def)
    }
    fn damage(&self) -> usize {
        self.weapon.dmg + self.rings.iter().flatten().map(|r| r.dmg).sum::<usize>()
    }
}

impl Ord for Loadout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for Loadout {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn all_loadouts() -> BTreeSet<Loadout> {
    let weapons = [
        Item {
            cost: 8,
            dmg: 4,
            def: 0,
        },
        Item {
            cost: 10,
            dmg: 5,
            def: 0,
        },
        Item {
            cost: 25,
            dmg: 6,
            def: 0,
        },
        Item {
            cost: 40,
            dmg: 7,
            def: 0,
        },
        Item {
            cost: 74,
            dmg: 8,
            def: 0,
        },
    ];
    let armors = [
        Item {
            cost: 13,
            dmg: 0,
            def: 1,
        },
        Item {
            cost: 31,
            dmg: 0,
            def: 2,
        },
        Item {
            cost: 53,
            dmg: 0,
            def: 3,
        },
        Item {
            cost: 75,
            dmg: 0,
            def: 4,
        },
        Item {
            cost: 102,
            dmg: 0,
            def: 5,
        },
    ];
    let rings = [
        Item {
            cost: 25,
            dmg: 1,
            def: 0,
        },
        Item {
            cost: 50,
            dmg: 2,
            def: 0,
        },
        Item {
            cost: 100,
            dmg: 3,
            def: 0,
        },
        Item {
            cost: 20,
            dmg: 0,
            def: 1,
        },
        Item {
            cost: 40,
            dmg: 0,
            def: 2,
        },
        Item {
            cost: 80,
            dmg: 0,
            def: 3,
        },
    ];

    let ring_combinations = std::iter::once(vec![None]) // no rings
        .chain(rings.iter().map(|r| vec![Some(*r)])) // just one ring
        .chain(rings.iter().flat_map(|r1| {
            rings
                .iter()
                .filter(move |r2| r1 != *r2)
                .map(move |r2| vec![Some(*r1), Some(*r2)])
        }));

    let armor_combinations = std::iter::once(None).chain(armors.iter().map(|a| Some(*a)));

    weapons
        .iter()
        .flat_map(|w| {
            let ring_combinations = ring_combinations.clone();
            armor_combinations.clone().flat_map(move |a| {
                ring_combinations.clone().map(move |r| Loadout {
                    weapon: *w,
                    armor: a,
                    rings: [
                        r.first().copied().unwrap_or_default(),
                        r.get(1).copied().unwrap_or_default(),
                    ],
                })
            })
        })
        .collect()
}

fn parse_boss(challenge: &str) -> Boss {
    challenge.lines().fold(
        Boss {
            hp: 0,
            dmg: 0,
            def: 0,
        },
        |boss, line| {
            let (stat, value) = line.split_once(": ").unwrap();
            let value = value.parse().unwrap();
            match stat {
                "Hit Points" => Boss { hp: value, ..boss },
                "Damage" => Boss { dmg: value, ..boss },
                "Armor" => Boss { def: value, ..boss },
                _ => boss,
            }
        },
    )
}

fn solve_part1(challenge: &str) -> usize {
    let boss = parse_boss(challenge);
    all_loadouts()
        .into_iter()
        .find(|l| l.defeats(&boss))
        .unwrap()
        .cost()
}

fn solve_part2(challenge: &str) -> usize {
    let boss = parse_boss(challenge);
    all_loadouts()
        .iter()
        .rev()
        .find(|l| !l.defeats(&boss))
        .unwrap()
        .cost()
}
