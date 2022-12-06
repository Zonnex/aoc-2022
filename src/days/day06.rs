use std::collections::HashSet;

use crate::{Solution, SolutionPair};

fn find_first_distinct_combination(input: &str, size: usize) -> usize {
    let mut hash_set = HashSet::new();

    let (position, _) = input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_, chars)| {
            hash_set.clear();
            chars.iter().all(|c| hash_set.insert(*c))
        })
        .expect("Expected to find a position");
    position + size
}


pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day6/real.txt");

    let sol1: usize = find_first_distinct_combination(input, 4);
    let sol2: usize = find_first_distinct_combination(input, 14);

    (Solution::USize(sol1), Solution::USize(sol2))
}
