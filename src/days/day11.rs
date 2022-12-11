use std::{collections::VecDeque, iter, str::Lines};

use itertools::Itertools;

use crate::{Solution, SolutionPair};

#[derive(Clone, Copy, Debug)]
enum Operation {
    Plus(u64),
    Multiply(u64),
    Squared(),
}

#[derive(Clone, Copy, Debug)]
struct Test {
    divider: u64,
    true_monkey: usize,
    false_monkey: usize,
}

#[derive(Clone, Debug)]
struct Monkey {
    inspect_count: i64,
    inventory: VecDeque<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn receive_item(&mut self, item: u64) {
        self.inventory.push_back(item);
    }

    fn throw_next<F: Fn(u64) -> u64>(&mut self, adjust_worry: F) -> Option<(u64, usize)> {
        self.inventory.pop_front().map(|worry| {
            self.inspect_count += 1;
            let worry = match self.operation {
                Operation::Plus(v) => worry + v,
                Operation::Multiply(v) => worry * v,
                Operation::Squared() => worry * worry,
            };
            let worry = adjust_worry(worry);
            match worry % self.test.divider == 0 {
                true => (worry, self.test.true_monkey),
                false => (worry, self.test.false_monkey),
            }
        })
    }
}

fn parse_inventory(line: &str) -> VecDeque<u64> {
    let (_, right) = line.split_once(": ").unwrap();

    right
        .split(", ")
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    let (_, right) = line.split_once("new = old ").unwrap();
    let (operation, value) = right.split_once(' ').unwrap();
    if value == "old" {
        return Operation::Squared();
    }
    let value = value.parse().unwrap();
    match operation {
        "+" => Operation::Plus(value),
        "*" => Operation::Multiply(value),
        _ => unreachable!(),
    }
}

fn parse_test(mut take: iter::Take<Lines>) -> Test {
    let (_, divider) = take.next().unwrap().split_once("by ").unwrap();
    let (_, true_monkey) = take.next().unwrap().split_once("monkey ").unwrap();
    let (_, false_monkey) = take.next().unwrap().split_once("monkey ").unwrap();

    Test {
        divider: divider.parse().unwrap(),
        true_monkey: true_monkey.parse().unwrap(),
        false_monkey: false_monkey.parse().unwrap(),
    }
}

fn parse_monkey(monkey: &str) -> Monkey {
    let (_, lines) = monkey.split_once(":\r\n").unwrap();
    let mut lines = lines.lines();

    let inventory = parse_inventory(lines.next().unwrap());
    let operation = parse_operation(lines.next().unwrap());
    let test = parse_test(lines.take(3));
    Monkey {
        inspect_count: 0,
        inventory,
        operation,
        test,
    }
}

fn play_round<F: Fn(u64) -> u64>(monkies: &mut [Monkey], adjust_worry: F) {
    for i in 0..monkies.len() {
        let mut monkey = monkies[i].clone();
        while let Some((item, target)) = monkey.throw_next(&adjust_worry) {
            let target = monkies.get_mut(target).unwrap();
            target.receive_item(item);
        }
        monkies[i] = monkey;
    }
}

fn simulation<F: Fn(u64) -> u64>(mut monkies: Vec<Monkey>, rounds: i64, adjust_worry: F) -> i64 {
    for _ in 0..rounds {
        play_round(&mut monkies, &adjust_worry);
    }
    monkies
        .iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day11/real.txt");

    let monkies: Vec<Monkey> = input.split("\r\n\r\n").map(parse_monkey).collect();

    let modulus: u64 = monkies.iter().map(|m| m.test.divider).product();
    let p1 = simulation(monkies.clone(), 20, |w| w / 3);
    let p2 = simulation(monkies, 10000, |w| w % modulus);

    (Solution::I64(p1), Solution::I64(p2))
}
