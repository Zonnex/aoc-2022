use std::{self, fs};
use itertools::Itertools;
use crate::{Solution, SolutionPair};


pub fn solve() -> SolutionPair {
    // Your solution here...

    // let file = "../../input/day1/test.txt";
    let file = include_str!("../../input/day1/real.txt");

    let elf_calories = fs::read_to_string(file)
        .expect("file not found")
        .split("\r\n\r\n")
        .map(|elf| 
            elf.lines()
                .map(|s| s.parse::<usize>().unwrap_or_default())
                .sum::<usize>()
        )
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let sol1 = elf_calories[0];
    let sol2 = elf_calories[0..3].iter().sum();

    (Solution::USize(sol1), Solution::USize(sol2))
}
