use std::collections::HashMap;

use itertools::Itertools;

use crate::{utils::vector_2d::Vector2, Solution, SolutionPair};

// chars:
const VOID: u8 = b'.';
const ROCK: u8 = b'#';

const ROCK_FORMATIONS: [[[u8; 4]; 4]; 5] = [
    [
        [VOID, VOID, VOID, VOID],
        [VOID, VOID, VOID, VOID],
        [VOID, VOID, VOID, VOID],
        [ROCK, ROCK, ROCK, ROCK],
    ],
    [
        [VOID, VOID, VOID, VOID],
        [VOID, ROCK, VOID, VOID],
        [ROCK, ROCK, ROCK, VOID],
        [VOID, ROCK, VOID, VOID],
    ],
    [
        [VOID, VOID, VOID, VOID],
        [VOID, VOID, ROCK, VOID],
        [VOID, VOID, ROCK, VOID],
        [ROCK, ROCK, ROCK, VOID],
    ],
    [
        [ROCK, VOID, VOID, VOID],
        [ROCK, VOID, VOID, VOID],
        [ROCK, VOID, VOID, VOID],
        [ROCK, VOID, VOID, VOID],
    ],
    [
        [VOID, VOID, VOID, VOID],
        [VOID, VOID, VOID, VOID],
        [ROCK, ROCK, VOID, VOID],
        [ROCK, ROCK, VOID, VOID],
    ],
];

struct Rock {
    shape: Shape,
    position: Vector2,
}

struct Map {
    map: HashMap<Vector2, u8>,
    rocks: usize,
    height: usize,
    width: usize,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            map: Default::default(),
            height: Default::default(),
            width: 7,
            rocks: 0,
        }
    }
}

type Shape = [[u8; 4]; 4];

impl Map {
    fn try_fit(&self, rock: &Rock, destination: &Vector2) -> bool {
        if destination.y < 0 {
            return false;
        }
        if destination.x < 0 {
            return false;
        }
        for (dy, row) in rock.shape.iter().rev().enumerate() {
            for (dx, _) in row.iter().enumerate().filter(|(_, &c)| c == ROCK) {
                let pixel = *destination + (dx, dy);
                if self.width <= pixel.x as usize {
                    return false;
                }
                if let Some(_v) = self.map.get(&pixel) {
                    return false;
                }
            }
        }
        true
    }

    fn try_move_rock(&self, rock: &mut Rock, dir: (isize, isize)) -> bool {
        let destination = rock.position + dir;
        if self.try_fit(rock, &destination) {
            rock.position = destination;
            return true;
        }
        false
    }

    fn place_rock(&mut self, rock: &Rock) {
        let mut max = self.height;
        for (y, row) in rock.shape.iter().rev().enumerate() {
            for (x, v) in row.iter().enumerate().filter(|(_, &c)| c == ROCK) {
                let position = rock.position + Vector2::try_from((x, y)).unwrap();
                self.map.insert(position, *v);
                max = max.max(position.y as usize + 1);
            }
        }
        self.height = max;
        self.rocks += 1;
    }

    fn print(&self) {
        for y in (0..=self.height).rev() {
            for x in 0..self.width {
                match self.map.get(&Vector2::try_from((x, y)).unwrap()) {
                    Some(_) => print!("#"),
                    None => print!("."),
                }
            }
            println!()
        }
        println!()
    }
}

fn simulate(jets: &[isize], iterations: usize) -> usize {
    let mut jet_cycle = jets.iter().enumerate().cycle();
    let mut rock_cycle = ROCK_FORMATIONS.iter().enumerate().cycle();
    let mut seen: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();
    let mut map = Map::default();
    let mut cycle_height = None;
    let mut iteration = 0;

    while iteration < iterations {
        let (rock_index, shape) = rock_cycle.next().unwrap();
        let mut jet_index = 0;
        let mut rock = Rock {
            shape: *shape,
            position: Vector2 {
                x: 2,
                y: map.height as isize + 3,
            },
        };

        loop {
            let (_jet_index, dx) = jet_cycle.next().unwrap();

            map.try_move_rock(&mut rock, (*dx, 0));
            if !map.try_move_rock(&mut rock, (0, -1)) {
                map.place_rock(&rock);
                break;
            }
            jet_index = _jet_index;
        }

        iteration += 1;

        if cycle_height.is_none() {
            let key = (rock_index, jet_index);

            if let Some((2, rocks, height)) = seen.get(&key) {
                let dy = map.height - *height;
                let pieces = map.rocks - rocks;
                let repeats = (iterations - iteration) / pieces;
                iteration += repeats * pieces;
                cycle_height = Some(repeats * dy)
            }

            seen.entry(key)
                .and_modify(|(count, old_rock_count, old_height)| {
                    *count += 1;
                    *old_rock_count = map.rocks;
                    *old_height = map.height;
                })
                .or_insert((1, map.rocks, map.height));
        }
    }

    map.height + cycle_height.unwrap_or_default()
}

pub fn solve(input: &str) -> SolutionPair {
    let jets = input
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'>' => 1,
            b'<' => -1,
            _ => unreachable!(),
        })
        .collect_vec();

    let p1 = simulate(&jets, 2022);
    let p2 = simulate(&jets, 1_000_000_000_000);

    // missing 46..
    (Solution::USize(p1), Solution::USize(p2))
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve() {
        let input = include_str!("../../input/day17/test.txt");
        _ = super::solve(input);
    }
}
