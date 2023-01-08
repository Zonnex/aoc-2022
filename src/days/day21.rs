use std::{collections::HashMap};

use itertools::Itertools;

use crate::{Solution, SolutionPair};

#[derive(Debug)]
enum Job<'a> {
    Number(i64),
    Math(&'a str, &'a str, &'a str),
}

fn calc(map: &HashMap<&str, Job>, current: &str) -> i64 {
    let monkey = map.get(current).unwrap();
    match monkey {
        Job::Number(v) => *v,
        Job::Math(l, o, r) => {
            let l = calc(map, l);
            let r = calc(map, r);
            match *o {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => l / r,
                _ => unreachable!(),
            }
        }
    }
}

fn try_humn_value(map: &mut HashMap<&str, Job>, value: i64) -> i64 {
    let x = map.get_mut("humn").unwrap();
    *x = Job::Number(value);

    if let Job::Math(l, _, r) = map.get("root").unwrap() {
        let l = calc(map, l);
        let r = calc(map, r);
        return l - r;
    }
    unreachable!()
}

fn part_two(mut map: HashMap<&str, Job>) -> i64 {
    let mut lower_bound: i64 = 0;
    let mut upper_bound: i64 = 0;
    let mut guess = 1;
    loop {
        let result = try_humn_value(&mut map, guess);
        if result == 0 {
            return guess;
        }

        if result < 0 {
            lower_bound = guess;
        } else {
            upper_bound = guess;
        }

        if lower_bound == 0 || upper_bound == 0 {
            guess *= 2;
        } else {
            guess = (lower_bound + upper_bound) / 2;
        }
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let monkey_map = input
        .lines()
        .map(|l| {
            let (m, r) = l.split_once(": ").unwrap();
            match r.parse::<i64>() {
                Ok(v) => (m, Job::Number(v)),
                Err(_) => {
                    let (l, o, r) = r.split(' ').collect_tuple().unwrap();
                    (m, Job::Math(l, o, r))
                }
            }
        })
        .collect::<HashMap<_, _>>();

    let p1 = calc(&monkey_map, "root");
    let p2 = part_two(monkey_map);

    (Solution::I64(p1), Solution::I64(p2))
}
