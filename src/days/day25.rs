use std::{iter::Sum, ops::Add};

use crate::{Solution, SolutionPair};

#[derive(Default)]
struct Fuel {
    value: i64,
}

impl Add for Fuel {
    type Output = Fuel;

    fn add(self, rhs: Self) -> Self::Output {
        Fuel {
            value: self.value + rhs.value,
        }
    }
}

impl<'a> Add for &'a Fuel {
    type Output = Fuel;

    fn add(self, other: &'a Fuel) -> Fuel {
        Fuel {
            value: self.value + other.value,
        }
    }
}

impl Sum for Fuel {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fuel::default(), |acc, x| acc + x)
    }
}

impl Fuel {
    fn from_snafu(snafu: &str) -> Fuel {
        let value = snafu
            .bytes()
            .rev()
            .enumerate()
            .map(|(i, b)| {
                let bit = 5_i64.pow(i as u32);
                let v: i64 = match b {
                    b'=' => -2,
                    b'-' => -1,
                    b'0' => 0,
                    b'1' => 1,
                    b'2' => 2,
                    _ => unreachable!(),
                };
                v * bit
            })
            .sum();

        Fuel { value }
    }

    fn to_snafu(&self) -> String {
        let mut n = self.value;
        let l = (n as f64).log(5.0).ceil();
        let mut digits = Vec::with_capacity(l as usize);

        let to_snafu_digit = |r| match r {
            3 => '=',
            4 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        };

        while n != 0 {
            let r = n % 5;
            let digit = to_snafu_digit(r);
            digits.push(digit);
            n = (n + 2) / 5;
        }

        digits.iter().rev().collect()
    }
}

pub fn solve(input: &str) -> SolutionPair {
    let p1 = input.lines().map(Fuel::from_snafu).sum::<Fuel>().to_snafu();
    let p2: u64 = 0;

    (Solution::Str(p1), Solution::U64(p2))
}

#[cfg(test)]
mod tests {
    use crate::days::day25::Fuel;

    #[test]
    fn from_snafu_test() {
        assert_eq!(1, Fuel::from_snafu("1").value);
        assert_eq!(2, Fuel::from_snafu("2").value);
        assert_eq!(3, Fuel::from_snafu("1=").value);
        assert_eq!(4, Fuel::from_snafu("1-").value);
        assert_eq!(5, Fuel::from_snafu("10").value);
        assert_eq!(6, Fuel::from_snafu("11").value);
        assert_eq!(7, Fuel::from_snafu("12").value);
        assert_eq!(8, Fuel::from_snafu("2=").value);
        assert_eq!(9, Fuel::from_snafu("2-").value);
        assert_eq!(10, Fuel::from_snafu("20").value);
        assert_eq!(15, Fuel::from_snafu("1=0").value);
        assert_eq!(20, Fuel::from_snafu("1-0").value);
        assert_eq!(2022, Fuel::from_snafu("1=11-2").value);
        assert_eq!(12345, Fuel::from_snafu("1-0---0").value);
        assert_eq!(314159265, Fuel::from_snafu("1121-1110-1=0").value);
    }

    #[test]
    fn to_snafu_test() {
        assert_eq!(Fuel { value: 1 }.to_snafu(), "1");
        assert_eq!(Fuel { value: 2 }.to_snafu(), "2");
        assert_eq!(Fuel { value: 3 }.to_snafu(), "1=");
        assert_eq!(Fuel { value: 4 }.to_snafu(), "1-");
        assert_eq!(Fuel { value: 5 }.to_snafu(), "10");
        assert_eq!(Fuel { value: 6 }.to_snafu(), "11");
        assert_eq!(Fuel { value: 7 }.to_snafu(), "12");
        assert_eq!(Fuel { value: 8 }.to_snafu(), "2=");
        assert_eq!(Fuel { value: 9 }.to_snafu(), "2-");
        assert_eq!(Fuel { value: 10 }.to_snafu(), "20");
        assert_eq!(Fuel { value: 15 }.to_snafu(), "1=0");
        assert_eq!(Fuel { value: 20 }.to_snafu(), "1-0");
        assert_eq!(Fuel { value: 2022 }.to_snafu(), "1=11-2");
        assert_eq!(Fuel { value: 12345 }.to_snafu(), "1-0---0");
        assert_eq!(Fuel { value: 314159265 }.to_snafu(), "1121-1110-1=0");
    }

    #[test]
    fn solve() {
        let input = include_str!("../../input/day25/test.txt");
        let (p1, p2) = super::solve(input);

        println!("{p1}, {p2}");
    }
}
