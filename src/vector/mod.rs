use std::ops;

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
    fn normalize(&mut self) -> Self;
}

#[derive(Debug, Copy, Clone)]
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

    fn normalize(&mut self) -> Self {
        let f = 1.0 / self.len();
        self.x = self.x * f;
        self.y = self.y * f;    // TODO: MACRO
        *self
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

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Vector for Vec3 {
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

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(
            -self.x,
            -self.y,
            -self.z,
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
