use std::ops::{RangeInclusive};

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

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day4/real.txt");

    let sol1 = input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let (r1, r2) = (parse_range(elf1), parse_range(elf2));
            
            check_full_overlap(&r1, &r2)
        })
        .count();

    let sol2 = input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let (r1, r2) = (parse_range(elf1), parse_range(elf2));
            
            check_any_overlap(&r1, &r2)
        })
        .count();

    (Solution::USize(sol1), Solution::USize(sol2))
}
