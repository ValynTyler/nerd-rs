<<<<<<< HEAD
pub mod mat2;
pub mod mat3;
pub mod mat4;
=======
use std::{fmt, ops};

use crate::vector::Vec2;

pub struct Mat2x2 {
    elements: [[f64; 2]; 2]
}

impl Mat2x2 {
    pub fn new(values: [[f64; 2]; 2]) -> Mat2x2 {
        Mat2x2 {
            elements: values
        }
    }

    pub fn get(&self, lin: usize, col: usize) -> f64 {
        self.elements[lin][col]
    }

    pub fn set(&mut self, lin: usize, col: usize, value: f64) {
        self.elements[lin][col] = value;
    }
}

impl ops::Mul<Vec2> for Mat2x2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        // rhs.x * col1 + rhs.y * col2
        Vec2 {
            x: rhs.x * self.elements[0][0] + rhs.y * self.elements[0][1],
            y: rhs.x * self.elements[1][0] + rhs.y * self.elements[1][1],
        }
    }
}

impl fmt::Display for Mat2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            [{} {}]
            [{} {}]
            ",
            self.elements[0][0],
            self.elements[0][1],
            self.elements[1][0],
            self.elements[1][1],
        )
    }
}
>>>>>>> b6e070e0cf69a445d6f0f91486b3f488848125a9
