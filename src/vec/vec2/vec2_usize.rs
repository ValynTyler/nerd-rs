use std::ops::{Add, Mul};

use super::vec2_isize::Vec2ISize;

#[derive(Debug)]
pub struct NotPositiveError;

#[derive(Debug, Clone, Copy)]
pub struct Vec2USize(pub usize, pub usize);

impl Add for Vec2USize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul::<usize> for Vec2USize {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl TryFrom::<Vec2ISize> for Vec2USize {
    type Error = NotPositiveError;

    fn try_from(value: Vec2ISize) -> Result<Self, Self::Error> {
        match value.0 >= 0 && value.1 >= 0 {
            true => Ok(Self(value.0 as usize, value.1 as usize)),
            false => Err(NotPositiveError),
        }
    }
}
