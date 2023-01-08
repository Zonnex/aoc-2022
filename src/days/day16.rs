use std::collections::HashMap;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

const ALL: u64 = !0 << 2;

#[derive(Debug)]
struct Valve {
    id: u64,
    name: String,
    flow: usize,
    tunnels: Vec<String>,
}

impl Valve {
    fn parse(id: usize, line: &str) -> Valve {
        let id = 1 << id;
        let (valve, tunnels) = line.split_once(';').unwrap();
        let (valve, rate) = valve.split_once('=').unwrap();
        let tunnels = tunnels.split(", ").map(|s| s.to_owned()).collect_vec();

        let name = valve.to_owned();
        let flow = rate.parse().unwrap();
        Valve {
            id,
            name,
            flow,
            tunnels,
        }
    }
}

struct TunnelMap {
    map: HashMap<String, Valve>,
}

impl TunnelMap {
    fn max_pressure(&self, start: &str, minutes: usize) -> usize {
        let mut cache = HashMap::new();

        self.dfs(&mut cache, start, 0, minutes)
    }

    fn dfs(
        &self,
        cache: &mut HashMap<(u64, u64, usize), usize>,
        valve_key: &str,
        valves_open: u64,
        minutes_remaining: usize,
    ) -> usize {
        if minutes_remaining == 0 {
            return 0;
        }
        // Can't get any more pressure
        if valves_open == ALL {
            return 0;
        }
        let valve = self.map.get(valve_key).unwrap();
        let cache_key = (valve.id, valves_open, minutes_remaining);

        if let Some(v) = cache.get(&cache_key) {
            return *v;
        }

        let best_unopened = valve.tunnels
            .iter()
            .map(|tunnel| self.dfs(cache, tunnel, valves_open, minutes_remaining - 1))
            .max()
            .unwrap();

        let best = match valve.flow == 0 || is_opened(valves_open, valve.id) {
            true => best_unopened,
            _ => {
                let minutes_remaining = minutes_remaining - 1;
                let best_tunnel = self.dfs(cache, valve_key, open_valve(valves_open, valve.id), minutes_remaining);

                let value = valve.flow * minutes_remaining;
                best_unopened.max(value + best_tunnel)
            }
        };

        cache.insert(cache_key, best);
        best
    }
}

fn is_opened(mask: u64, bit: u64) -> bool {
    mask | bit == mask
}

fn open_valve(mask: u64, bit: u64) -> u64 {
    mask | bit
}

fn part_one(map: HashMap<String, Valve>, start: &str, minutes: usize) -> usize {
    let tunnel_map = TunnelMap { map };
    tunnel_map.max_pressure(start, minutes)
}

fn trim_input(input: &str) -> String {
    input
        .replace("valves", "valve")
        .replace("tunnels", "tunnel")
        .replace("leads", "lead")
        .replace(" tunnel lead to valve ", "")
        .replace("Valve ", "")
        .replace(" has flow rate", "")
}

pub fn solve(input: &str) -> SolutionPair {
    let input = trim_input(input);

    let valves = input
        .lines()
        .sorted()
        .enumerate()
        .map(|(i, l)| Valve::parse(i, l))
        .map(|v| (v.name.clone(), v))
        .collect::<HashMap<_, _>>();

    let p1 = part_one(valves, "AA", 30);
    let p2: u64 = 0;

    (Solution::USize(p1), Solution::U64(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    use super::is_opened;
    use super::open_valve;

    #[test]
    fn test_day() {
        let input = include_str!("../../input/day16/test.txt");
        let (p1, _) = super::solve(input);
        if let Solution::U32(p1) = p1 {
            assert_eq!(p1, 1651);
        }
    }

    #[test]
    fn test_bit_mask() {
        assert!(is_opened(1 | 3, 1));
        assert!(!is_opened(2 | 4 | 6, 8));
    }

    #[test]
    fn test_set() {
        assert_eq!(open_valve(0, 1), 1);
        assert_eq!(open_valve(1, 2), 3);
        assert_eq!(open_valve(3, 4), 7);
    }
}
