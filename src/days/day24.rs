use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{
    utils::vector_2d::{Vector2D, E, N, S, W},
    Solution, SolutionPair,
};

type Bounds = (usize, usize);

#[derive(PartialEq)]
enum Entity {
    Blizzard(Vector2D),
    Entrance,
    Exit,
    Wall,
}
struct Canyon {
    bounds: Bounds,
    entities: HashMap<Vector2D, Entity>,
}

#[derive(Debug, Eq)]
struct HeapData {
    time: usize,
    distance: usize,
    point: Vector2D,
}

impl Ord for HeapData {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then(other.distance.cmp(&self.distance))
    }
}

impl PartialOrd for HeapData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapData {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.distance == other.distance
    }
}

impl Canyon {
    fn position_at_offset(&self, position: Vector2D, offsets: Vector2D) -> Vector2D {
        let (width, height) = self.bounds;
        let (width, height) = (width as isize, height as isize);

        Vector2D {
            x: (position.x - 1 + offsets.x).rem_euclid(width - 2) + 1,
            y: (position.y - 1 + offsets.y).rem_euclid(height - 2) + 1,
        }
    }
    fn positions_at_offset(&self, p: Vector2D, offset: usize) -> [Vector2D; 4] {
        [
            self.position_at_offset(p, N * offset),
            self.position_at_offset(p, E * offset),
            self.position_at_offset(p, W * offset),
            self.position_at_offset(p, S * offset),
        ]
    }

    fn is_in_bounds(&self, v: Vector2D) -> bool {
        let (width, height) = self.bounds;
        let (x, y) = (v.x, v.y);

        x.is_positive() && y.is_positive() && (x as usize).lt(&width) && (y as usize).lt(&height)
    }

    fn can_move_to(&self, position: Vector2D, time: usize) -> bool {
        match self.entities.get(&position) {
            Some(Entity::Exit) => true,
            Some(Entity::Entrance) => true,
            Some(Entity::Wall) => false,
            _ => {
                if !self.is_in_bounds(position) {
                    return false;
                }

                let blizzards = self
                    .positions_at_offset(position, time)
                    .iter()
                    .enumerate()
                    .map(|(i, p)| match i {
                        0 => (S, p),
                        1 => (W, p),
                        2 => (E, p),
                        3 => (N, p),
                        _ => unreachable!(),
                    })
                    .filter(|(v, p)| match self.entities.get(p) {
                        Some(Entity::Blizzard(d)) => *v == *d,
                        _ => false,
                    })
                    .count();

                blizzards == 0
            }
        }
    }

    fn find_entrance_exit(&self) -> (Vector2D, Vector2D) {
        let mut entrance_key = None;
        let mut exit_key = None;
        let mut iter = self.entities.iter();
        while entrance_key.is_none() || exit_key.is_none() {
            match iter.next() {
                Some((k, v)) => match v {
                    Entity::Entrance => entrance_key = Some(*k),
                    Entity::Exit => exit_key = Some(*k),
                    _ => (),
                },
                None => break,
            }
        }
        (entrance_key.unwrap(), exit_key.unwrap())
    }

    fn find_path(&self, start: Vector2D, end: Vector2D, start_time: usize) -> usize {
        let mut queue = BinaryHeap::new();

        queue.push(HeapData {
            time: start_time,
            distance: start.distance_to(end),
            point: start,
        });

        let mut visited = HashSet::new();

        while let Some(data) = queue.pop() {
            if data.point == end {
                return data.time;
            } else {
                let time = data.time + 1;
                if self.can_move_to(data.point, time) {
                    queue.push(HeapData {
                        time,
                        distance: data.point.distance_to(end),
                        point: data.point,
                    });
                }
                for p in [N, E, W, S]
                    .iter()
                    .map(|d| data.point + *d)
                    .filter(|p| visited.insert((*p, time)))
                    .filter(|p| self.can_move_to(*p, time))
                {
                    queue.push(HeapData {
                        time,
                        distance: p.distance_to(end),
                        point: p,
                    });
                }
            }
        }
        0
    }
}

fn parse_input(input: &str) -> Canyon {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let bounds = (width, height);

    let mut entities = input
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, l)| {
            l.bytes().enumerate().map(move |(x, b)| {
                let x = x as isize;
                let y = y as isize;
                (Vector2D { x, y }, b)
            })
        })
        .filter_map(|(p, b)| match b {
            b'#' => Some((p, Entity::Wall)),
            b'^' => Some((p, Entity::Blizzard(N))),
            b'>' => Some((p, Entity::Blizzard(E))),
            b'<' => Some((p, Entity::Blizzard(W))),
            b'v' => Some((p, Entity::Blizzard(S))),
            _ => None,
        })
        .collect::<HashMap<_, _>>();

    let x = 1;
    let y = height as isize - 1;
    entities.insert(Vector2D { x, y }, Entity::Entrance);

    let x = width as isize - 2;
    let y = 0;
    entities.insert(Vector2D { x, y }, Entity::Exit);

    Canyon { entities, bounds }
}

pub fn solve(input: &str) -> SolutionPair {
    let map = parse_input(input);

    let (start, end) = map.find_entrance_exit();
    let start_to_end = map.find_path(start, end, 0);
    let end_to_start = map.find_path(end, start, start_to_end);
    let and_back_again = map.find_path(start, end, end_to_start);

    (
        Solution::USize(start_to_end),
        Solution::USize(and_back_again),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = include_str!("../../input/day24/test.txt");
        _ = super::solve(input);
    }
}
