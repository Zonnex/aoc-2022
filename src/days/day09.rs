use crate::{Solution, SolutionPair};
use std::collections::HashSet;

type Pair = (i32, i32);

fn move_head((x, y): &Pair, (dx, dy): &Pair) -> (i32, i32) {
    (x + dx, y + dy)
}

fn move_tail((hx, hy): &Pair, (tx, ty): &Pair) -> Pair {
    let (dx, dy) = ((hx - tx), (hy - ty));
    if dx.abs() < 2 && dy.abs() < 2 {
        return (*tx, *ty);
    }
    (tx + dx.signum(), ty + dy.signum())
}

fn parse_direction(dir: &str) -> Pair {
    match dir {
        "U" => ( 1,  0),
        "D" => (-1,  0),
        "L" => ( 0, -1),
        "R" => ( 0,  1),
        _ => unreachable!(),
    }
}

fn simulate_knots(movements: &[(i32, i32)], knots: usize) -> usize {
    let mut knots = vec![(0, 0); knots];

    let mut tail_positions = HashSet::with_capacity(movements.len());
    tail_positions.insert((0, 0));

    for movement in movements {
        knots[0] = move_head(&knots[0], movement);

        for i in 1..knots.len() {
            let head = knots[i - 1];
            let tail = knots[i];
            knots[i] = move_tail(&head, &tail);
        }
        let tail = knots.last().unwrap();
        tail_positions.insert(*tail);
    }
    tail_positions.len()
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day9/real.txt");

    let movements = input
        .lines()
        .flat_map(|l| {
            let (dir, steps) = l.split_once(' ').unwrap();
            let steps = steps.parse().unwrap();
            (0..steps).map(|_| parse_direction(dir))
        })
        .collect::<Vec<_>>();

    let p1: usize = simulate_knots(&movements, 2);
    let p2: usize = simulate_knots(&movements, 10);

    (Solution::USize(p1), Solution::USize(p2))
}
