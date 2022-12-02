use crate::{Solution, SolutionPair};

const WIN: usize = 6;
const DRAW: usize = 3;
const LOSS: usize = 0;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSOR: usize = 3;


enum Choice {
    Rock = 0,
    Paper = 1,
    Scissor = 2
}

enum Outcome {
    Win = 0,
    Draw = 1,
    Loss = 2,
}

fn parse_choice(c: char) -> Choice {
    match c {
        'X' | 'A' => Choice::Rock,
        'Y' | 'B' => Choice::Paper,
        'Z' | 'C' => Choice::Scissor,
        _ => panic!("parse error")
    }
}

fn score_round_p1(rules: &[Vec<usize>], round: (Choice, Choice)) -> usize {
    let (r, c) = round;
    rules[r as usize][c as usize]
}

fn parse_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Win,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Loss,
        _ => panic!("parse error")
    }
}

fn score_round_p2(rules: &[Vec<usize>], round: (Choice, Outcome)) -> usize {
    let (r, o) = round;
    rules[r as usize][o as usize]
}

pub fn solve() -> SolutionPair {
    let input = include_str!("../../input/day2/real.txt");
    let p1_rules = vec![
        vec![ROCK + DRAW, PAPER + WIN,  SCISSOR + LOSS],    // opponent picks rock
        vec![ROCK + LOSS, PAPER + DRAW, SCISSOR + WIN],     // opponent picks paper
        vec![ROCK + WIN,  PAPER + LOSS, SCISSOR + DRAW],    // opponent picks scissor
    ];

    let p2_rules = vec![
        vec![SCISSOR + LOSS, ROCK + DRAW, PAPER + WIN],     // opponent picks rock
        vec![ROCK + LOSS, PAPER + DRAW, SCISSOR + WIN],     // opponent picks paper
        vec![PAPER + LOSS,  SCISSOR + DRAW, ROCK + WIN],    // opponent picks scissor
    ];

    let rounds = input.lines()
        .into_iter()
        .map(|l| {
            let bytes = l.as_bytes();
            let opponent_choice = bytes[0] as char;
            let my_choice = bytes[2] as char;
            (opponent_choice, my_choice)
        })
        .collect::<Vec<_>>();
    
    // Your solution here...
    let p1 = rounds.iter()
        .map(|p| {
            let choices = (parse_choice(p.0), parse_choice(p.1));
            score_round_p1(&p1_rules, choices)
        })
        .sum();

    let sol2 = rounds.iter()
        .map(|p| {
            let choices = (parse_choice(p.0), parse_outcome(p.1));
            score_round_p2(&p2_rules, choices)
        })
        .sum();

    (Solution::USize(p1), Solution::USize(sol2))
}
