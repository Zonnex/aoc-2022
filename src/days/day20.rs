use crate::{Solution, SolutionPair};

fn find_new_index(position_values: &[(usize, isize)], index: usize) -> usize {
    position_values
        .iter()
        .position(|(i, _)| *i == index)
        .unwrap()
}

fn compute_new_index(index: usize, offset: isize, length: usize) -> usize {
    (index as isize + offset).rem_euclid(length as isize) as usize
}

fn mix(original: &[isize], iterations: usize, decrypt_key: isize) -> isize {
    let mut position_values: Vec<(usize, isize)> = original
        .iter()
        .enumerate()
        .map(|(i, v)| (i, v * decrypt_key))
        .collect();

    let length = position_values.len();
    for _ in 0..iterations {
        for index in 0..length {
            let index = find_new_index(&position_values, index);
            let (p, v) = position_values.remove(index);
            let new_index = compute_new_index(index, v, length - 1);
            position_values.insert(new_index, (p, v));
        }
    }
    let values = position_values.iter().map(|(_, v)| *v).collect::<Vec<_>>();
    let zero_index = values.iter().position(|i| *i == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| (zero_index + i) % length)
        .filter_map(|i| values.get(i))
        .sum()
}

pub fn solve(input: &str) -> SolutionPair {
    let values: Vec<isize> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    let p1 = mix(&values, 1, 1);
    let p2 = mix(&values, 10, 811_589_153);

    (Solution::ISize(p1), Solution::ISize(p2))
}
