use std::collections::VecDeque;

use crate::{Solution, SolutionPair};

fn parse_state(state: &str) -> Vec<Vec<char>> {
    let mut iterator = state.lines().rev();
    let heading = iterator.next().expect("Failed to get header");

    let stack_count = heading
        .trim_end()
        .chars()
        .last()
        .expect("Failed to get last")
        .to_digit(10)
        .expect("Wasn't a digit") as usize;

    let mut stacks = vec![vec![]; stack_count];

    for line in iterator {
        for (stack, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks[stack].push(c);
            }
        }
    }
    stacks
}

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize
}

fn part_one(mut state: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let v = state[instruction.from-1].pop().expect("No item to pop");
            state[instruction.to-1].push(v);
        }
    }

    let chars = state
        .iter()
        .filter_map(|stack| stack.last());

    String::from_iter(chars)
}

fn part_two(mut state: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    let mut temp = VecDeque::new();
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let v = state[instruction.from-1].pop().expect("No item to pop");
            temp.push_front(v);
        }

        while let Some(v) = temp.pop_front() {
            state[instruction.to-1].push(v);

        }
    }

    let chars = state
        .iter()
        .filter_map(|stack| stack.last());

    String::from_iter(chars)
}

pub fn solve(input: &str) -> SolutionPair {
    let (state, instructions) = input
        .split_once("\r\n\r\n")
        .expect("Invalid input for day5");

    let state = parse_state(state);

    let instructions = instructions
        .lines()
        .map(|l| {
            let values = l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();

            Instruction {
                amount: values[0],
                from: values[1],
                to: values[2]
            }
        })
        .collect::<Vec<_>>();

    let p1 = part_one(state.clone(), &instructions);
    let p2 = part_two(state, &instructions);

    (Solution::Str(p1), Solution::Str(p2))
}
