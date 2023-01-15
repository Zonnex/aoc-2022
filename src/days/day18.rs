use std::{collections::HashSet, ops::Add};

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

fn bounds_check(Position(x, y, z): &Position, bounds: &GridSize) -> bool {
    let (x2, y2, z2) = (bounds.x + 1, bounds.y + 1, bounds.z + 1);

    (-1..=x2).contains(x) && (-1..=y2).contains(y) && (-1..=z2).contains(z)
}

fn part_two(boxes: &HashSet<Position>, bounds: GridSize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(Position(0, 0, 0));
    let mut total = 0;

    while let Some(position) = queue.pop() {
        if visited.contains(&position) {
            continue;
        }

        for dir in DIRS.iter() {
            let side = position.add(*dir);
            if boxes.contains(&side) {
                total += 1;
            } else if bounds_check(&side, &bounds) {
                queue.push(side);
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
