use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
enum State {
    Wall,
    Rest,
}

type Map = HashMap<(usize, usize), State>;
type Position = (usize, usize);

fn parse_position(p: &str) -> Position {
    let (x1, y1) = p.split_once(',').unwrap();

    (x1.parse().unwrap(), y1.parse().unwrap())
}

fn add((x, y): Position, (dx, dy): (isize, isize)) -> Position {
    let (x, y) = ((x as isize + dx), (y as isize + dy));
    (x as usize, y as usize)
}

fn insert_walls(grid: &mut Map, line: &str) {
    line.split(" -> ").tuple_windows().for_each(|(from, to)| {
        let (x1, y1) = parse_position(from);
        let (x2, y2) = parse_position(to);

        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                grid.insert((x, y), State::Wall);
            }
        }
    });
}

fn map_anchors(grid: &Map) -> (usize, usize, usize, usize) {
    grid.keys().fold(
        (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
        |(min_x, min_y, max_x, max_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
        },
    )
}

fn move_sand_to_rest(map: &Map, start: Position, depth: usize) -> Position {
    let mut current = start;
    let mut rest = None;
    let candidates = [(0isize, 1isize), (-1, 1), (1, 1)];

    while current.1 < depth && rest.is_none() {
        let next = candidates
            .iter()
            .find_map(|&d| {
                let next = add(current, d);
                match map.get(&next) {
                    None => Some(next),
                    _ => None,
                }
            });

        match next {
            Some(position) => current = position,
            None => rest = Some(current),
        }
    }

    rest.unwrap_or(current)
}

fn part_one(mut map: Map, inlet: Position, depth: usize) -> (usize, Map) {
    loop {
        let pos = move_sand_to_rest(&map, inlet, depth);
        if pos.1 == depth {
            break;
        }
        map.insert(pos, State::Rest);
    }

    let rests = map
        .values()
        .filter(|&s| *s == State::Rest)
        .count();

    (rests, map)
}

fn part_two(mut map: Map, inlet: Position, depth: usize) -> (usize, Map) {
    loop {
        let pos = move_sand_to_rest(&map, inlet, depth);
        match pos {
            (500, 0) => break,
            _ => {
                map.insert(pos, State::Rest);
            }
        }
    }

    let rests = map
        .values()
        .filter(|&s| *s == State::Rest)
        .count() + 1;

    (rests, map)
}

pub fn solve(input: &str) -> SolutionPair {
    let inlet = (500, 0);

    let mut map: Map = HashMap::new();
    input.lines().for_each(|line| insert_walls(&mut map, line));
    let (_, _, _, map_depth) = map_anchors(&map);

    let (p1, map) = part_one(map, inlet, map_depth);
    let (p2, _map) = part_two(map, inlet, map_depth + 1);

    (Solution::USize(p1), Solution::USize(p2))
}
