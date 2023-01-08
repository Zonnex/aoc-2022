use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

use itertools::Itertools;

use crate::{Solution, SolutionPair};

const DIRS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Position(i32, i32, i32);

impl Add<(i32, i32, i32)> for Position {
    type Output = Position;

    fn add(self, (dx, dy, dz): (i32, i32, i32)) -> Self::Output {
        let Position(x, y, z) = self;
        Position(x + dx, y + dy, z + dz)
    }
}

struct GridSize {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for GridSize {
    fn default() -> Self {
        Self {
            x: i32::MIN,
            y: i32::MIN,
            z: i32::MIN,
        }
    }
}

fn part_one(boxes: &HashSet<Position>) -> usize {
    boxes
        .iter()
        .map(|Position(x, y, z)| {
            let mut sides = 6;
            for (dx, dy, dz) in DIRS {
                let check = Position(x + dx, y + dy, z + dz);
                if boxes.contains(&check) {
                    sides -= 1;
                }
            }
            sides
        })
        .sum()
}

fn part_two(boxes: &HashSet<Position>, range: GridSize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Position(0, 0, 0));
    let mut total = 0;

    while let Some(position) = queue.pop_front() {
        if visited.contains(&position) {
            continue;
        }

        for dir in DIRS.iter() {
            let side = position.add(*dir);
            if boxes.contains(&side) {
                total += 1;
            } else if (0..=range.x+5).contains(&side.0)
                && (0..=range.y+5).contains(&side.1)
                && (0..=range.z+5).contains(&side.2)
            {
                queue.push_back(side);
            }
        }

        visited.insert(position);
    }
    total
}

pub fn solve(input: &str) -> SolutionPair {
    let mut grid_size = GridSize::default();

    let boxes = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(x, y, z)| Position(x, y, z))
        .map(|Position(x, y, z)| {
            grid_size.x = grid_size.x.max(x);
            grid_size.y = grid_size.y.max(y);
            grid_size.z = grid_size.z.max(z);
            Position(x, y, z)
        })
        .collect::<HashSet<_>>();

    let p1 = part_one(&boxes);
    let p2 = part_two(&boxes, grid_size);

    (Solution::USize(p1), Solution::USize(p2))
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve() {
        let input = include_str!("../../input/day18/test.txt");
        let (p1, p2) = super::solve(input);
        println!("{}, {}", p1, p2);
    }
}
