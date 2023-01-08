use itertools::Itertools;

use crate::{Solution, SolutionPair};

fn value(c: u8) -> usize {
    match c {
        b'a'..=b'z' => c as usize - b'a' as usize + 1,
        b'A'..=b'Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn find_match(line: &[u8]) -> Option<u8> {
    let half = line.len() / 2;
    for c1 in &line[..half] {
        for c2 in &line[half..] {
            if c1 == c2 {
                return Some(*c1)
            }
        }
    }
    None
}

fn find_overlap(elf1: &[u8], elf2: &[u8], elf3: &[u8]) -> Option<u8> {
    for c1 in elf1 {
        for c2 in elf2 {
            if c1 == c2 {
                for c3 in elf3 {
                    if c1 == c3 {
                        return Some(*c1)
                    }
                }
            }
        }
    }
    None
}

pub fn solve(input: &str) -> SolutionPair {
    let lines = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let p1: usize = lines
        .iter()
        .filter_map(|line| find_match(line))
        .map(value)
        .sum();

    let p2: usize = lines
        .iter()
        .tuples()
        .filter_map(|(elf1,elf2,elf3)| find_overlap(elf1, elf2, elf3))
        .map(value)
        .sum();

    (Solution::USize(p1), Solution::USize(p2))
}
