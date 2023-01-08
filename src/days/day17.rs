use std::{collections::HashMap, ops::Add};

use itertools::Itertools;

use crate::{Solution, SolutionPair};

#[derive(Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<(isize, isize)> for Position {
    type Output = Position;

    fn add(self, _rhs: (isize, isize)) -> Self::Output {
        todo!()
    }
}

impl Add<(isize, isize)> for &Position {
    type Output = Position;

    fn add(self, _rhs: (isize, isize)) -> Self::Output {
        todo!()
    }
}

impl Add<(usize, usize)> for &Position {
    type Output = Position;
    fn add(self, _rhs: (usize, usize)) -> Self::Output {
        todo!()
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Position;
    fn add(self, _rhs: (usize, usize)) -> Self::Output {
        todo!()
    }
}

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

struct Map {
    map: HashMap<Position, u8>,
    highest_rock: usize,
}

type Shape = [[u8; 4]; 4];

struct Rock {
    shape: Shape,
    position: Position,
}

impl Map {
    fn try_fit(&self, rock: &Rock, destination: &Position) -> bool {
        for (dy, row) in rock.shape.iter().rev().enumerate() {
            for (dx, _column) in row.iter().filter(|&&c| c == ROCK).enumerate() {
                let pixel = destination + (dx, dy);
                match self.map.get(&pixel) {
                    Some(_v) => todo!(),
                    None => todo!(),
                }
            }
        }
        false
    }

    fn try_move_rock(&self, rock: &mut Rock, dir: (isize, isize)) -> bool {
        let destination = &rock.position + dir;
        self.try_fit(rock, &destination)
    }

    fn place_rock(&mut self, rock: Rock) {
        let mut max = self.highest_rock;
        for (x, row) in rock.shape.iter().enumerate() {
            for (y, v) in row.iter().enumerate() {
                let position = Position { x, y };
                self.map.insert(position, *v);
                max = max.max(y);
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
            62 => 1_isize,
            _ => -1,
        })
        .collect_vec();

    let mut jet_cycle = dxs.iter().cycle();

    let mut rock_cycle = ROCK_FORMATIONS.iter().cycle();

    let mut map = Map {
        map: HashMap::new(),
        highest_rock: 0,
    };

    for _n in 1..=2022 {
        let mut rock = Rock {
            shape: *rock_cycle.next().unwrap(),
            position: Position {
                x: 2_usize,
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

    let p1: u64 = 0;
    let p2: u64 = 0;

    (Solution::U64(p1), Solution::U64(p2))
}
