use crate::{Solution, SolutionPair};

fn all_unique_bits(chars: &[u8]) -> bool {
    for i in 1..chars.len() {
        for j in 0..i {
            if chars[i] == chars[j] {
                return false;
            };
        }
    }
    true
}

fn find_first_distinct_combination(input: &str, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .position(all_unique_bits)
        .expect("Expected to find a position") + size
}


pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day6/real.txt");

    let sol1: usize = find_first_distinct_combination(input, 4);
    let sol2: usize = find_first_distinct_combination(input, 14);

    (Solution::USize(sol1), Solution::USize(sol2))
}
