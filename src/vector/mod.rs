use core::fmt;
use std::ops;

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
    fn len(&self) -> f32;
    fn normalize(&mut self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Vector2 {
        vector!(x, y)
    }
}

impl Vector for Vector2 {
    fn len(&self) -> f32 {
        f32::sqrt(square_magnitude!(self.x, self.y))
    }

    fn normalize(&mut self) -> Self {
        let f = 1.0 / self.len();
        self.x = self.x * f;
        self.y = self.y * f;    // TODO: MACRO
        *self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3    = Vector3::new(0., 0., 0.,);
    pub const FORWARD: Vector3 = Vector3::new(0., 0., 1.,);
    pub const RIGHT: Vector3   = Vector3::new(1., 0., 0.,);
    pub const UP: Vector3      = Vector3::new(0., 1., 0.,);

    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        vector!(x, y, z)
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn get(&self, i: usize) -> f32 {
        match i {
            1 => self.x,
            2 => self.y,
            3 => self.z,
            _ => panic!(),
        }
    }

    pub fn set(&mut self, i: usize, value: f32) {
        match i {
            1 => self.x = value,
            2 => self.y = value,
            3 => self.z = value,
            _ => panic!(),
        }
    }
}

impl Vector for Vector3 {
    fn len(&self) -> f32 {
        f32::sqrt(square_magnitude!(self.x, self.y, self.z))
    }

    fn normalize(&mut self) -> Self {
        let f = 1.0 / self.len();
        self.x = self.x * f;
        self.y = self.y * f;
        self.z = self.z * f;
        *self
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

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(
            -self.x,
            -self.y,
            -self.z,
        )
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
