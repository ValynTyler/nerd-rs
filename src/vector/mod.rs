use std::ops;

#[macro_export]
macro_rules! vector {
    ($x:expr) => ($x);
    ($x:expr, $y:expr) => (Vector2 {
        x: $x,
        y: $y,
    });
    ($x:expr, $y:expr, $z:expr) => (Vec3 {
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
    fn len(&self) -> f32;
}

#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        vector!(x, y)
    }
}

impl Vector for Vector2 {
    fn len(&self) -> f32 {
        f32::sqrt(square_magnitude!(self.x, self.y))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        vector!(x, y, z)
    }
}

impl Vector for Vec3 {
    fn len(&self) -> f32 {
        f32::sqrt(square_magnitude!(self.x, self.y, self.z))
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

#[derive(Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        vector!(x, y, z, w)
    }
}
