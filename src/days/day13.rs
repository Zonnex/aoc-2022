use std::cmp::{max, Ordering};

use itertools::Itertools;
use serde_json::{Value, json};

use crate::{Solution, SolutionPair};

fn parse_signals(input: &str) -> Vec<Value> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            serde_json::from_str::<Value>(line).ok()
        })
        .collect::<Vec<_>>()
}

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let x = x.as_u64().unwrap();
            let y = &y.as_u64().unwrap();
            x.cmp(y)
        },
        (Value::Array(left), Value::Array(right)) => {
            for i in 0..max(left.len(), right.len()) {
                match (left.get(i), right.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match compare(x, y) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    },
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => {
            compare(a, &Value::Array(vec![b.clone()]))
        },
        (Value::Number(_), Value::Array(_)) => {
            compare(&Value::Array(vec![a.clone()]), b)
        },
        _ => unreachable!(),
    }
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day13/real.txt");

    let mut signals = parse_signals(input);

    let p1 = signals
        .iter()
        .tuples()
        .positions(|(a, b)| compare(a, b) != Ordering::Greater)
        .map(|i| i + 1)
        .sum();

    let beacons = [json!([[2]]), json!([[6]])];

    signals.extend(beacons.iter().cloned());
    signals.sort_by(compare);
    
    let p2 = signals
        .iter()
        .positions(|b| beacons.contains(b))
        .map(|i| i + 1)
        .product();

    (Solution::USize(p1), Solution::USize(p2))
}
