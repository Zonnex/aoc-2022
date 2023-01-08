use std::collections::HashMap;

use itertools::Itertools;

use crate::{Solution, SolutionPair};

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(u32),
    Turn(Turn),
}

type Grove = HashMap<(usize, usize), Tile>;

fn password(((column, row), (dx, dy)): ((usize, usize), (isize, isize))) -> usize {
    let f = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    row * 1000 + column * 4 + f
}

fn parse_map(map: &str) -> Grove {
    map.lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, tile)| {
                let position = (x + 1, y + 1);
                match tile {
                    b'.' => Some((position, Tile::Open)),
                    b'#' => Some((position, Tile::Wall)),
                    _ => None,
                }
            })
        })
        .collect()
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    let turns = instructions
        .split(|c: char| c.is_ascii_digit())
        .filter_map(|b| match b {
            "R" => Some(Turn::R),
            "L" => Some(Turn::L),
            _ => None,
        })
        .map(Instruction::Turn);

    let moves = instructions
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|s| s.parse().ok())
        .map(Instruction::Move);

    moves.interleave(turns).collect()
}

fn rotate_point(point: (isize, isize), turn: &Turn) -> (isize, isize) {
    let angle = match turn {
        Turn::L => -90.0,
        Turn::R => 90.0,
    };
    let angle_radians = angle * std::f64::consts::PI / 180.0;
    let rotation_matrix = [
        [angle_radians.cos(), -angle_radians.sin()],
        [angle_radians.sin(), angle_radians.cos()],
    ];
    let x = point.0 as f64;
    let y = point.1 as f64;
    let rotated_x = (rotation_matrix[0][0] * x + rotation_matrix[0][1] * y) as isize;
    let rotated_y = (rotation_matrix[1][0] * x + rotation_matrix[1][1] * y) as isize;
    (rotated_x, rotated_y)
}


fn part_one(map: &Grove, instructions: &[Instruction]) -> usize {
    fn add_with_wrap(grid_size: (usize, usize, usize, usize), (x, y): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
        let (mut x, mut y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
    
        if x < grid_size.0 {
            x = grid_size.1;
        }
        if grid_size.1 < x {
            x = grid_size.0;
        }
        if y < grid_size.2 {
            y = grid_size.3
        }
        if grid_size.3 < y {
            y = grid_size.2
        }
    
    
        (x,y)
    }
    let x = (1..usize::MAX)
        .find(|x| map.contains_key(&(*x, 1)))
        .unwrap();

    let map_size = map.keys().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |(x1, x2, y1, y2), &(x, y)| (x1.min(x), x2.max(x), y1.min(y), y2.max(y)),
    );

    let y: usize = 1;

    let final_state =
        instructions
            .iter()
            .fold(
                ((x, y), (1, 0)),
                |((x, y), (dx, dy)), instruction| match instruction {
                    Instruction::Turn(t) => ((x, y), rotate_point((dx, dy), t)),
                    Instruction::Move(steps) => {
                        let mut position = (x, y);
                        for _ in 1..=*steps {
                            let mut next = add_with_wrap(map_size, position, (dx, dy));

                            while map.get(&next).is_none() {
                                next = add_with_wrap(map_size, next, (dx, dy))
                            }

                            match map.get(&next) {
                                Some(Tile::Open) => position = next,
                                Some(Tile::Wall) => break,
                                _ => unreachable!(),
                            }
                        }
                        (position, (dx, dy))
                    }
                },
            );

    password(final_state)
}

pub fn solve(input: &str) -> SolutionPair {
    let (map, instructions) = input.split_once("\r\n\r\n").unwrap();
    let map = parse_map(map);
    let instructions = parse_instructions(instructions);

    for y in 1..=12 {
        for x in 1..=16 {
            match map.get(&(x, y)) {
                Some(Tile::Open) => print!("."),
                Some(Tile::Wall) => print!("#"),
                None => print!(" "),
            }
        }
        println!()
    }

    println!("{:?}", instructions);

    let p1 = part_one(&map, &instructions);
    let p2 = 0;

    (Solution::USize(p1), Solution::U64(p2))
}

#[cfg(test)]
mod tests {
    use crate::etc::Solution;

    #[test]
    fn solve() {
        let input = include_str!("../../input/day22/test.txt");
        let (p1, _p2) = super::solve(input);

        if let Solution::U32(v) = p1 {
            assert_eq!(v, 6032)
        }
    }
}
