use crate::{Solution, SolutionPair};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Add,
};

#[derive(Eq, Hash, PartialEq, Copy, Clone, PartialOrd, Debug, Ord)]
struct Point(isize, isize);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, Point(dx, dy): Point) -> Self::Output {
        let Point(x, y) = self;

        Point(x + dx, y + dy)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point::add(*self, *rhs)
    }
}

const NW: Point = Point(-1, 1);
const N: Point = Point(0, 1);
const NE: Point = Point(1, 1);
const E: Point = Point(1, 0);
const SE: Point = Point(1, -1);
const S: Point = Point(0, -1);
const SW: Point = Point(-1, -1);
const W: Point = Point(-1, 0);

type DirCheck = [Point; 3];

const N_DIR: DirCheck = [N, NE, NW];
const S_DIR: DirCheck = [S, SE, SW];
const W_DIR: DirCheck = [W, NW, SW];
const E_DIR: DirCheck = [E, NE, SE];

const INITIAL_DIR_ORDER: [DirCheck; 4] = [N_DIR, S_DIR, W_DIR, E_DIR];
const SURROUNDING_TILES: [Point; 8] = [NW, N, NE, W, E, SW, S, SE];

#[derive(Clone, Copy, Debug)]
struct Elf {
    position: Point,
}

impl Elf {
    fn new(position: Point) -> Elf {
        Elf { position }
    }

    fn set_position(&mut self, point: Point) {
        self.position = point
    }
}

#[derive(Clone)]
struct Elves {
    elves: Vec<Elf>,
}

impl Elves {
    fn positions(&self) -> HashSet<Point> {
        self.elves.iter().map(|e| e.position).collect()
    }
}

fn propose_direction(
    index: usize,
    elf: &Elf,
    positions: &HashSet<Point>,
    directions: &VecDeque<DirCheck>,
) -> Option<(usize, Point)> {
    directions.iter().find_map(|direction| {
        let test = direction
            .iter()
            .map(|p| *p + elf.position)
            .find(|p| positions.contains(p));

        match test {
            Some(_) => None,
            None => Some((index, elf.position + direction[0])),
        }
    })
}

fn simulate_round(mut elves: Elves, dirs: &VecDeque<[Point; 3]>) -> (Elves, bool) {
    let positions = elves.positions();

    let needs_to_move = positions
        .iter()
        .flat_map(|p| SURROUNDING_TILES.iter().map(|dz| *dz + *p))
        .collect::<HashSet<_>>();

    let mut to_move = elves
        .elves
        .iter()
        .enumerate()
        .filter(|(_, elf)| needs_to_move.contains(&elf.position))
        .filter_map(|(index, elf)| propose_direction(index, elf, &positions, dirs))
        .collect::<Vec<_>>();

    let mut counts: HashMap<Point, usize> = HashMap::with_capacity(to_move.len());
    for (_, dest) in to_move.iter() {
        *counts.entry(*dest).or_insert(0) += 1;
    }

    to_move.retain(|(_, p)| counts[p] == 1);

    for (index, dest) in to_move.iter() {
        elves.elves[*index].set_position(*dest);
    }
    (elves, !to_move.is_empty())
}

fn part_one(mut elves: Elves, mut dirs: VecDeque<DirCheck>) -> usize {
    for _ in 1..=10 {
        (elves, _) = simulate_round(elves, &dirs);
        dirs.rotate_left(1);
    }

    let (x1, x2, y1, y2) = elves.positions().iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(x1, x2, y1, y2), &Point(x, y)| (x1.min(x), x2.max(x), y1.min(y), y2.max(y)),
    );

    let b = x1.abs_diff(x2) + 1;
    let h = y1.abs_diff(y2) + 1;
    (b * h) - elves.elves.len()
}

fn part_two(mut elves: Elves, mut dirs: VecDeque<DirCheck>) -> usize {
    let mut did_move;
    for n in 1.. {
        (elves, did_move) = simulate_round(elves, &dirs);
        if !did_move {
            return n;
        }
        dirs.rotate_left(1);
    }
    unreachable!()
}

fn parse_input(input: &str) -> Elves {
    let elves = input
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| Elf::new(Point(x as isize, y as isize)))
        })
        .collect::<Vec<_>>();

    Elves { elves }
}

pub fn solve(input: &str) -> SolutionPair {
    let dirs = VecDeque::from_iter(INITIAL_DIR_ORDER);
    let elves = parse_input(input);

    let p1 = part_one(elves.clone(), dirs.clone());
    let p2 = part_two(elves, dirs);

    (Solution::USize(p1), Solution::USize(p2))
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = include_str!("../../input/day23/test.txt");
        super::solve(input);
    }
}
