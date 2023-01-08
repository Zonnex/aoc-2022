use std::ops::RangeInclusive;

use crate::{Solution, SolutionPair};

fn parse_range(r: &str) -> RangeInclusive<i32> {
    let (a, b) = r.split_once('-').unwrap();
    let from = a.parse().unwrap();
    let to = b.parse().unwrap();
    from..=to
}

fn check_full_overlap(l: &RangeInclusive<i32>, r: &RangeInclusive<i32>) -> bool {
    l.contains(r.start()) && l.contains(r.end()) 
    || r.contains(l.start()) && r.contains(l.end())
}

fn check_any_overlap(l: &RangeInclusive<i32>, r: &RangeInclusive<i32>) -> bool {
    l.contains(r.start()) || l.contains(r.end()) 
    || r.contains(l.start()) || r.contains(l.end())
}

pub fn solve(input: &str) -> SolutionPair {
    let pairs = input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            (parse_range(elf1), parse_range(elf2))
        })
        .collect::<Vec<_>>();

    let p1 = pairs.iter()
        .filter(|(r1,r2)| check_full_overlap(r1, r2))
        .count();
    
    let p2 = pairs.iter()
        .filter(|(r1,r2)| check_any_overlap(r1, r2))
        .count();

    (Solution::USize(p1), Solution::USize(p2))
}
