use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 22)?;
    Ok((
        Box::new(solve_part1(&challenge)),
        Box::new(solve_part2(&challenge)),
    ))
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
struct Game {
    player_mana: i32,
    player_hp: i32,
    player_armor: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mana_spent: i32,
    is_hard_mode: bool,
}

impl Game {
    fn new(hp: i32, dmg: i32, hard_mode: bool) -> Game {
        Game {
            player_mana: 500,
            player_hp: 50,
            player_armor: 0,
            boss_hp: hp,
            boss_damage: dmg,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            mana_spent: 0,
            is_hard_mode: hard_mode,
        }
    }

    fn next_states(&self) -> impl Iterator<Item = Game> + '_ {
        all_spells().filter_map(|spell| {
            let mut game = *self;
            if game.is_hard_mode {
                game.player_hp -= 1;
                if game.player_hp <= 0 {
                    return None;
                }
            }
            if !game.apply_effects() {
                return Some(game);
            }

            if !game.can_cast(spell) {
                return None;
            }

            if game.cast_spell(spell) && !game.process_boss_turn() {
                return None;
            }

            Some(game)
        })
    }

    fn can_cast(&self, spell: Spell) -> bool {
        if self.player_mana < spell_cost(spell) {
            return false;
        }
        match spell {
            Spell::Shield => self.shield_timer <= 1,
            Spell::Poison => self.poison_timer <= 1,
            Spell::Recharge => self.recharge_timer <= 1,
            _ => true,
        }
    }

    fn cast_spell(&mut self, spell: Spell) -> bool {
        self.player_mana -= spell_cost(spell);
        self.mana_spent += spell_cost(spell);

        match spell {
            Spell::MagicMissile => {
                self.boss_hp -= 4;
            }
            Spell::Drain => {
                self.boss_hp -= 2;
                self.player_hp += 2;
            }
            Spell::Shield => {
                self.shield_timer = 6;
            }
            Spell::Poison => {
                self.poison_timer = 6;
            }
            Spell::Recharge => {
                self.recharge_timer = 5;
            }
        }

        self.boss_hp > 0
    }

    fn apply_effects(&mut self) -> bool {
        self.player_armor = if self.shield_timer > 0 { 7 } else { 0 };

        if self.poison_timer > 0 {
            self.boss_hp -= 3;
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
        }

        self.shield_timer = (self.shield_timer - 1).max(0);
        self.poison_timer = (self.poison_timer - 1).max(0);
        self.recharge_timer = (self.recharge_timer - 1).max(0);

        self.boss_hp > 0
    }

    fn process_boss_turn(&mut self) -> bool {
        self.apply_effects();

        self.player_hp -= (self.boss_damage - self.player_armor).max(1);

        self.player_hp > 0
    }
}
impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

fn spell_cost(spell: Spell) -> i32 {
    match spell {
        Spell::MagicMissile => 53,
        Spell::Drain => 73,
        Spell::Shield => 113,
        Spell::Poison => 173,
        Spell::Recharge => 229,
    }
}

fn all_spells() -> impl Iterator<Item = Spell> {
    [
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge,
    ]
    .into_iter()
}

fn parse(challenge: &str) -> (i32, i32) {
    challenge.lines().fold((0, 0), |(hp, dmg), line| {
        let (stat, value) = line.split_once(": ").unwrap();
        match stat {
            "Hit Points" => (value.parse().unwrap(), dmg),
            "Damage" => (hp, value.parse().unwrap()),
            _ => (hp, dmg),
        }
    })
}

fn solve_part1(challenge: &str) -> i32 {
    let (boss_hp, boss_damage) = parse(challenge);
    let game_start = Game::new(boss_hp, boss_damage, false);
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(game_start);

    while let Some(game) = queue.pop() {
        for next in game.next_states() {
            if next.boss_hp <= 0 {
                return next.mana_spent;
            }

            if seen.insert(next) {
                queue.push(next);
            }
        }
    }

    0
}

fn solve_part2(challenge: &str) -> i32 {
    let (boss_hp, boss_damage) = parse(challenge);
    let game_start = Game::new(boss_hp, boss_damage, true);
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    queue.push(game_start);

    while let Some(game) = queue.pop() {
        for next in game.next_states() {
            if next.boss_hp <= 0 {
                return next.mana_spent;
            }

            if seen.insert(next) {
                queue.push(next);
            }
        }
    }

    0
}
