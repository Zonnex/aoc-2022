use std::collections::HashMap;

use itertools::Itertools;

use crate::{utils::vector_2d::Vector2D, Solution, SolutionPair};

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
    position: Vector2D,
}

struct Map {
    map: HashMap<Vector2D, u8>,
    highest_rock: isize,
    width: isize,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            map: Default::default(),
            highest_rock: Default::default(),
            width: 7,
        }
    }
}

type Shape = [[u8; 4]; 4];

impl Map {
    fn try_fit(&self, rock: &Rock, destination: &Vector2D) -> bool {
        if destination.y < 0 {
            return false;
        }
        if destination.x <= 0 {
            return false;
        }
        for (dy, row) in rock.shape.iter().rev().enumerate() {
            for (dx, _) in row.iter().enumerate().filter(|(_, &c)| c == ROCK) {
                let pixel = *destination + (dx, dy);
                if self.width <= pixel.x {
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

    fn place_rock(&mut self, rock: Rock) {
        let mut max = self.highest_rock;
        for (y, row) in rock.shape.iter().rev().enumerate() {
            for (x, v) in row.iter().enumerate().filter(|(_, &c)| c == ROCK) {
                let position = rock.position + Vector2D::new(x as isize, y as isize);
                self.map.insert(position, *v);
                max = max.max(position.y);
            }
        }
        self.highest_rock = max;
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let dxs = input
        .as_bytes()
        .iter()
        .map(|b| match b {
            b'>' => 1_isize,
            b'<' => -1,
            _ => unreachable!(),
        })
        .collect_vec();

    let mut jet_cycle = dxs.iter().cycle();
    let mut rock_cycle = ROCK_FORMATIONS.iter().cycle();
    let mut map = Map::default();

    for _n in 1..=2022 {
        let mut rock = Rock {
            shape: *rock_cycle.next().unwrap(),
            position: Vector2D {
                x: 2,
                y: map.highest_rock + 3,
            },
        };

        loop {
            let dx = jet_cycle.next().unwrap();

            map.try_move_rock(&mut rock, (*dx, 0));
            if !map.try_move_rock(&mut rock, (0, -1)) {
                map.place_rock(rock);
                break;
            }
        }
    }

    let p1 = map.highest_rock + 1;
    let p2: u64 = 0;

    (Solution::ISize(p1), Solution::U64(p2))
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve() {
        let input = include_str!("../../input/day17/test.txt");
        _ = super::solve(input);
    }
}
