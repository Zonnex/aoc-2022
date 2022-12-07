use crate::{Solution, SolutionPair};

fn all_unique_bits(masks: &[u32]) -> bool {
    let mut unique = 0;
    for mask in masks {
        if unique | mask == unique {
            return false;
        }
        unique |= mask;
    }
    true
}


fn find_first_distinct_combination(input: &[u32], size: usize) -> usize {
    input
        .windows(size)
        .position(all_unique_bits)
        .expect("Expected to find a position") + size
}


pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day6/real.txt");
    const ASCII_A_LOWERCASE: u8 = 97;

    let mask_vec = input
        .bytes()
        .map(|c| 1_u32 << (c - ASCII_A_LOWERCASE))
        .collect::<Vec<_>>();

    let sol1: usize = find_first_distinct_combination(&mask_vec, 4);
    let sol2: usize = find_first_distinct_combination(&mask_vec, 14);

    (Solution::USize(sol1), Solution::USize(sol2))
}
