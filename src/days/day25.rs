use crate::{Solution, SolutionPair};

pub fn solve(_input: &str) -> SolutionPair {
    // Your solution here...
    let p1: u64 = 0;
    let p2: u64 = 0;

    (Solution::U64(p1), Solution::U64(p2))
}

/*

  Decimal          SNAFU
        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       15            1=0
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0

*/

#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let input = include_str!("../../input/day25/test.txt");
        _ = super::solve(input);
    }
}