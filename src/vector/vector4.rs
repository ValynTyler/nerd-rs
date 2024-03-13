use super::{square_magnitude, vector};

pub use super::Vector;

#[derive(Debug, Copy, Clone)]
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

impl Vector for Vector4 {
    fn len(&self) -> f32 {
        square_magnitude!(self.x, self.y, self.z, self.w)
    }

    fn normalize(&mut self) -> Self {
        let f = 1.0 / self.len();
        self.x = self.x * f;
        self.y = self.y * f;
        self.z = self.z * f;
        self.w = self.w * f;
        *self
    }
}
