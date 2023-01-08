use itertools::Itertools;
use crate::{Solution, SolutionPair};

pub fn solve(input: &str) -> SolutionPair {
    let elf_calories = input
        .split("\r\n\r\n")
        .map(|elf| 
            elf.lines()
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .sum::<usize>()
        )
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let p1 = elf_calories[0];
    let p2 = elf_calories[0..3].iter().sum();

    (Solution::USize(p1), Solution::USize(p2))
}
