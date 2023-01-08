use crate::{Solution, SolutionPair};

fn add(u: usize, i: isize) -> usize {
    ((u as isize) + i) as usize
}

fn try_get_neighbor(grid: &[Vec<u32>], x: usize, y: usize, dx: isize, dy: isize) -> Option<&u32> {
    grid.get(add(x, dx)).and_then(|x| x.get(add(y, dy)))
}

fn check_visible(grid: &[Vec<u32>], x: usize, y: usize) -> (bool, u32) {
    let tree_height = grid[x][y];

    if x == 0 || y == 0 {
        return (true, 0);
    }

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let (mut invisible, mut score) = (true, 1);

    for (dx, dy) in dirs {
        let (mut x, mut y, mut i, mut visible) = (x, y, 0, true);

        while let Some(&neighbor) = try_get_neighbor(grid, x, y, dx, dy) {
            i += 1;
            if tree_height <= neighbor {
                visible = false;
                break;
            }
            x = add(x, dx);
            y = add(y, dy);
        }
        if visible {
            invisible = false;
        }
        score *= i;
    }
    (!invisible, score)
}

pub fn solve(input: &str) -> SolutionPair {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| char::to_digit(c, 10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut tree_count = 0;
    let mut max_score = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            let (visible, score) = check_visible(&grid, x, y);
            if visible {
                tree_count += 1;
                max_score = max_score.max(score);
            }
        }
    }

    (Solution::I32(tree_count), Solution::U32(max_score))
}
