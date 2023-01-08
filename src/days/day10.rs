use std::ops::Index;

use crate::{Solution, SolutionPair};

fn parse_line(line: &str) -> i32 {
    match line {
        "noop" => 0,
        _ => {
            let (_, v) = line.split_once(' ').unwrap();

            v.parse().unwrap()
        }
    }
}

fn render_screen(cycles: &[i32]) -> String {
    let mut pixels = String::with_capacity(240);
    (0..240).for_each(|cycle| {
        if cycle % 40 == 0 {
            pixels.push('\n');
        }
        let sprite = cycles[cycle];
        let c = if (sprite - cycle as i32 % 40).abs() < 2 {
                '#'
            } else {
                ' '
            };
        pixels.push(c);
    });
    pixels
}

pub fn solve(input: &str) -> SolutionPair {
    let mut x = 1;
    let mut cycles = Vec::with_capacity(240);

    for instruction in input.lines().map(parse_line) {
        cycles.push(x);
        if instruction != 0 {
            cycles.push(x);
            x += instruction;
        }
    }

    let p1 = (20..=220)
        .step_by(40)
        .map(|i| i as i32 * cycles.index(i - 1))
        .sum();

    let pixels = render_screen(&cycles);

    (Solution::I32(p1), Solution::Str(pixels))
}
