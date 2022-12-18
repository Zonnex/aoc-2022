use std::collections::HashSet;

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

#[derive(Hash, Eq, PartialEq)]
struct Position(i32, i32, i32);

struct GridRange {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,
}

impl Default for GridRange {
    fn default() -> Self {
        Self {
            x_min: i32::MAX,
            x_max: i32::MIN,
            y_min: i32::MAX,
            y_max: i32::MIN,
            z_min: i32::MAX,
            z_max: i32::MIN,
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

fn part_two(boxes: &HashSet<Position>, visible_sides: usize, range: GridRange) -> usize {

    // try flood fill instead
    0
    // let mut air_pockets = Vec::new();

    // for x in range.x_min..=range.x_max {
    //     for y in range.y_min..=range.y_max {
    //         for z in range.z_min..=range.z_max {
    //             match boxes.get(&Position(x, y, z)) {
    //                 None => {
    //                     // might be an air pocket
    //                     if DIRS.iter().all(|(dx, dy, dz)| {
    //                         let side = Position(x + dx, y + dy, z + dz);
    //                         boxes.contains(&side)
    //                     }) {
    //                         air_pockets.push((x, y, z));
    //                     }
    //                 }
    //                 _ => continue,
    //             }
    //         }
    //     }
    // }
    // visible_sides - air_pockets.len() * 6
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day18/real.txt");
    let mut grid_range = GridRange::default();

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
            grid_range.x_min = grid_range.x_min.min(x);
            grid_range.x_max = grid_range.x_max.max(x);
            grid_range.y_min = grid_range.y_min.min(y);
            grid_range.y_max = grid_range.y_max.max(y);
            grid_range.z_min = grid_range.z_min.min(z);
            grid_range.z_max = grid_range.z_max.max(z);
            Position(x, y, z)
        })
        .collect::<HashSet<_>>();

    let p1 = part_one(&boxes);
    let p2 = part_two(&boxes, p1, grid_range);

    (Solution::USize(p1), Solution::USize(p2))
}


#[cfg(test)]
mod tests {

    #[test]
    fn solve() {
        let (p1, p2) = super::solve();
    }
}