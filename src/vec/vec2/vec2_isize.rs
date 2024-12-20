use std::ops::{Add, Mul};

use super::vec2_usize::Vec2USize;

#[derive(Debug, Clone, Copy)]
pub struct Vec2ISize(pub isize, pub isize);

impl Add for Vec2ISize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul::<isize> for Vec2ISize {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl From::<Vec2USize> for Vec2ISize {
    fn from(value: Vec2USize) -> Self {
        Self(value.0 as isize, value.1 as isize)
    }
}
