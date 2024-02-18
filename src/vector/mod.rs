pub mod vec2;
use std::ops;

pub use vec2::Vec2;

macro_rules! vector {
    ($x:expr) => ($x);
    ($x:expr, $y:expr) => (Vector2 {
        x: $x,
        y: $y,
    });
    ($x:expr, $y:expr, $z:expr) => (Vector3 {
        x: $x,
        y: $y,
        z: $z,
    });
    ($x:expr, $y:expr, $z:expr, $w:expr) => (Vector4 {
        x: $x,
        y: $y,
        z: $z,
        w: $w,
    });
}

macro_rules! square_magnitude {
    // Base case:
    ($x:expr) => ($x * $x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        (square_magnitude!($x) + square_magnitude!($($y),+))
    )
}

pub trait Vector {
    fn len(&self) -> f64;
}

#[derive(Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        vector!(x, y)
    }
}

impl Vector for Vector2 {
    fn len(&self) -> f64 {
        f64::sqrt(square_magnitude!(self.x, self.y))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        vector!(x, y, z)
    }
}

impl Vector for Vector3 {
    fn len(&self) -> f64 {
        f64::sqrt(square_magnitude!(self.x, self.y, self.z))
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}
