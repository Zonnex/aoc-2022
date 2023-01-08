use itertools::Itertools;

use crate::{Solution, SolutionPair};

type Position = (i64, i64);

#[derive(Debug)]
struct Sensor {
    position: Position,
    closest_beacon: Position,
    distance: i64,
}

fn manhattan_distance((x1, y1): Position, (x2, y2): Position) -> i64 {
    (y2 - y1).abs() + (x2 - x1).abs()
}

impl Sensor {
    fn parse(line: &str) -> Option<Sensor> {
        let (sensor, beacon) = line.split_once(": ")?;
        let position = Sensor::parse_sensor(sensor)?;
        let beacon = Sensor::parse_beacon(beacon)?;

        Some(Sensor {
            position,
            closest_beacon: beacon,
            distance: manhattan_distance(position, beacon),
        })
    }

    fn parse_sensor(sensor: &str) -> Option<Position> {
        let (x, y) = sensor.trim_start_matches("Sensor at ").split_once(", ")?;
        let x = x.trim_start_matches("x=").parse().unwrap();
        let y = y.trim_start_matches("y=").parse().unwrap();

        Some((x, y))
    }

    fn parse_beacon(beacon: &str) -> Option<Position> {
        let (x, y) = beacon
            .trim_start_matches("closest beacon is at ")
            .split_once(", ")?;
        let x = x.trim_start_matches("x=").parse().unwrap();
        let y = y.trim_start_matches("y=").parse().unwrap();

        Some((x, y))
    }

    fn y_line_coverage(&self, y: i64) -> Option<(i64, i64)> {
        let dy = (self.position.1 - y).abs();
        if dy <= self.distance {
            let x = self.position.0;
            let dx = self.distance - dy;
            return Some(((x - dx), (x + dx)))
        }
        None
    }

    fn is_inside_range(&self, point: Position) -> bool {
        if self.closest_beacon == point {
            return false;
        }
        self.distance >= manhattan_distance(self.position, point)
    }
}


fn part_two(sensors: &[Sensor], size: i64) -> i64 {
    sensors
        .iter()
        .find_map(|s| {
            let (x, y) = s.position;
            let x_min = (x - s.distance - 1).max(0);
            let x_max = x.min(size);

            // check positive quadrant
            (x_min..=x_max).zip(y..=size).find_map(|(x, y)| {
                sensors
                    .iter()
                    .all(|s| !s.is_inside_range((x, y)))
                    .then_some(x * 4000000 + y)
            })
        })
        .unwrap()
}

fn merge_range(mut acc: Vec<(i64, i64)>, (x1, x2): Position) -> Vec<(i64, i64)> {
    if acc.is_empty() {
        acc.push((x1, x2));
        return acc;
    }

    let (r1, r2) = acc.last().unwrap();
    let r2 = *r2;

    if r2 > x1 {
        *acc.last_mut().unwrap() = (*r1, r2.max(x2));
    } else {
        acc.push((x1, x2));
    }
    acc
}

pub fn solve(input: &str) -> SolutionPair {
    let input = input;
    let y = 2_000_000;
    let size = 4_000_000;

    let sensors = input.lines().filter_map(Sensor::parse).collect::<Vec<_>>();

    let p1 = sensors
        .iter()
        .filter_map(|s| s.y_line_coverage(y))
        .sorted()
        .fold(Vec::new(), merge_range)
        .iter()
        .fold(0, |acc, (x1, x2)| acc + (x2 - x1));

    let p2 = part_two(&sensors, size);

    (Solution::I64(p1), Solution::I64(p2))
}
