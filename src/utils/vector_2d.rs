#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const N: Vector2 = Vector2 { x: 0, y: 1 };
pub const E: Vector2 = Vector2 { x: 1, y: 0 };
pub const W: Vector2 = Vector2 { x: -1, y: 0 };
pub const S: Vector2 = Vector2 { x: 0, y: -1 };

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

impl Vector2 {

    pub const fn new_usize(x: usize, y: usize) -> Self {
        Vector2 { x: x as isize, y: y as isize }
    }

    pub fn adjacent_points(&self) -> [Vector2; 4] {
        [N, E, W, S].map(|d| *self + d)
    }

    pub fn column(&self) -> isize {
        self.x
    }

    pub fn column_index(&self) -> usize {
        self.x as usize
    }

    pub fn row(&self) -> isize {
        self.y
    }

    pub fn row_index(&self) -> usize {
        self.y as usize
    }

    pub fn manhattan_distance(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }

    pub fn distance_to(&self, other: Vector2) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl TryFrom<(usize, usize)> for Vector2 {
    type Error = &'static str;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        let x = isize::try_from(value.0).map_err(|_| "x value is too large for isize")?;
        let y = isize::try_from(value.1).map_err(|_| "y value is too large for isize")?;
        Ok(Vector2 { x, y })
    }
}


impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&(isize, isize)> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &(isize, isize)) -> Self::Output {
        Vector2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<(isize, isize)> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Vector2 {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<(usize, usize)> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Vector2 {
            x: self.x + rhs.0 as isize,
            y: self.y + rhs.1 as isize,
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: &Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: &Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<usize> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as isize;

        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

