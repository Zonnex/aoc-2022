use std::collections::HashMap;

use crate::{Solution, SolutionPair};
use itertools::Itertools;


struct Blueprint {
    ore: u8,
    clay: u8,
    obsidian: (u8, u8),
    geode: (u8, u8),
}

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode
}

struct State {
    ores: [u8; 4],
    robots: [u8; 4],
}

impl Default for State {
    fn default() -> Self {
        Self {
            ores: [1,0,0,0],
            robots: Default::default(),
        }
    }
}

impl State {
    fn _tick(&mut self) {
        for (i, robot) in self.robots.iter().enumerate() {
            self.ores[i] += robot;
        }
    }

    fn produce_geodes(&self, blueprint: &Blueprint) -> u32 {
        let mut cache = HashMap::new();
        self.try_options(blueprint, &mut cache, 24)
    }

    fn try_options(&self, blueprint: &Blueprint, _cache: &mut HashMap<(State,u8), u32>, minutes: u8) -> u32 {
        if minutes == 0 {
            return 0
        }
        if self.try_buy(blueprint, Resource::Geode) {
            
        }
        0
    }

    fn try_buy(&self, blueprint: &Blueprint, resource: Resource) -> bool {
        if self.can_afford(blueprint, resource) {
            return true
        }
        false
    }

    fn can_afford(&self, blueprint: &Blueprint, resource: Resource) -> bool {
        match resource {
            Resource::Ore => self.ores[0] >= blueprint.ore,
            Resource::Clay => self.ores[1] >= blueprint.clay,
            Resource::Obsidian => self.ores[0] > blueprint.ore && self.ores[1] >= blueprint.clay,
            Resource::Geode => self.ores[0] > blueprint.ore && self.ores[2] > blueprint.geode.1
        }
    }
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        let (_, rest) = line.split_once(':').unwrap();
        rest.split('.');

        let (clay, ore, obs_ore, obs_clay, geode_ore, geode_obsidian) = rest
            .split(|c: char| !c.is_ascii_digit()) // split everything that isn't a digit
            .filter_map(|w| w.parse::<u8>().ok())
            .collect_tuple()
            .unwrap();

        Blueprint {
            clay,
            ore,
            obsidian: (obs_ore, obs_clay),
            geode: (geode_ore, geode_obsidian),
        }
    }
}

fn part_one(blueprints: &[Blueprint]) -> u32 {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| (i + 1) as u32 * State::default().produce_geodes(bp))
        .sum()
}

pub fn solve(input: &str) -> SolutionPair {
    let blueprints = input.lines().map(Blueprint::parse).collect_vec();

    let p1 = part_one(&blueprints);
    let p2: u64 = 0;

    (Solution::U32(p1), Solution::U64(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = include_str!("../../input/day19/test.txt");
        solve(input);
    }
}