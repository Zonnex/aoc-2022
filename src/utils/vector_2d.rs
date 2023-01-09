#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

pub const N: Vector2D = Vector2D { x: 0, y: 1 };
pub const E: Vector2D = Vector2D { x: 1, y: 0 };
pub const W: Vector2D = Vector2D { x: -1, y: 0 };
pub const S: Vector2D = Vector2D { x: 0, y: -1 };

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2D {
    pub x: isize,
    pub y: isize,
}

impl Vector2D {
    pub const fn new(x: isize, y: isize) -> Self {
        Vector2D { x, y }
    }

    pub fn adjacent_points(&self) -> [Vector2D; 4] {
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

    pub fn distance_to(&self, other: Vector2D) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: &Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&(isize, isize)> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: &(isize, isize)) -> Self::Output {
        Vector2D {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<(isize, isize)> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Vector2D {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Add<(usize, usize)> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: (usize, usize)) -> Self::Output {
        Vector2D {
            x: self.x + rhs.0 as isize,
            y: self.y + rhs.1 as isize,
        }
    }
}

impl AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: &Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: &Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector2D> for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<&Vector2D> for Vector2D {
    fn sub_assign(&mut self, rhs: &Vector2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul<usize> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let rhs = rhs as isize;

        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

