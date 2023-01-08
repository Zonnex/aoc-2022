use std::collections::{HashSet, VecDeque};

use crate::{Solution, SolutionPair};

type Grid = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn add(&self, (dx, dy): (isize, isize)) -> Position {
        let x = (self.x as isize) + dx;
        let y = (self.y as isize) + dy;
        Position {
            x: x as usize,
            y: y as usize,
        }
    }
}

fn search(grid: &Grid, start: Position, end: Position) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    queue.push_back((start, 0));
    while let Some((p, path_length)) = queue.pop_front() {
        if p == end {
            return Some(path_length);
        }
        for dir in directions {
            let p2 = p.add(dir);
            match grid.get(p2.y).and_then(|row| row.get(p2.x)) {
                None => continue,
                Some(&height) => {
                    let current = grid[p.y][p.x];
                    let can_climb = current + 1 >= height;
                    if can_climb && visited.insert(p2) {
                        queue.push_back((p2, path_length + 1));
                    }
                }
            }
        }
    }
    None
}

fn find(grid: &Grid, c: u8) -> Position {
    let height = grid.len();
    let width = grid[0].len();
    
    #[allow(clippy::needless_range_loop)]
    for y in 0..height {
        for x in 0..width {
            let v = grid[y][x];
            if v == c {
                return Position { x, y };
            }
        }
    }
    panic!("Couldn't find {}", c)
}

pub fn solve(input: &str) -> SolutionPair {
    let mut grid: Grid = input.lines().map(|line| line.as_bytes().to_vec()).collect();

    let start = find(&grid, b'S');
    let end = find(&grid, b'E');

    grid[start.y][start.x] = b'a';

    let p1: u32 = search(&grid, start, end).unwrap();

    let candidates = grid
        .iter()
        .enumerate()
        .flat_map(|(y, xs)| {
            xs.iter().enumerate().filter_map(move |(x, &v)| {
                if v == b'a' {
                    Some(Position { x, y })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let p2: u32 = candidates
        .iter()
        .filter_map(|p| search(&grid, *p, end))
        .min()
        .unwrap();

    (Solution::U32(p1), Solution::U32(p2))
}
