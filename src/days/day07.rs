use std::{collections::{HashMap}, path::PathBuf};

use crate::{Solution, SolutionPair};

const PART_ONE_SIZE: i32 = 100_000;
const PART_TWO_SIZE: i32 = 30_000_000;

pub fn solve(input: &str) -> SolutionPair {
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();
    let mut current_path = PathBuf::new();

    let lines = input.split('$').skip(1);
    for line in lines.map(str::trim) {
        match line.lines().next().unwrap() {
            "ls" => {
                let size: i32 = line
                    .lines()
                    .skip(1)
                    .filter_map(|output| {
                        let (size, _) = output.split_once(' ').unwrap();
                        size.parse::<i32>().ok()
                    })
                    .sum();
                    
                let current_dir = current_path.clone();
                for dir in current_dir.ancestors() {
                    let dir = String::from(dir.to_str().unwrap());
                    dir_sizes.entry(dir).and_modify(|total| *total += size).or_insert(size);
                }
            }
            "cd .." => { 
                current_path.pop();
            },
            dir_command => {
                let (_, dir) = dir_command.split_once(' ').unwrap();
                current_path.push(dir);
            }
        }
    }

    let used_space = dir_sizes.get("/").copied().unwrap();
    let p2_predicate = |s| {
        70_000_000 - PART_TWO_SIZE + s >= used_space
    };

    let p1: i32 = dir_sizes.values().filter(|&&s| s <= PART_ONE_SIZE).sum();
    let p2 = dir_sizes.values().filter(|&&s| p2_predicate(s)).min().copied().unwrap();

    (Solution::I32(p1), Solution::I32(p2))
}
