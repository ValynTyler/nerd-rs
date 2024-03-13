use super::square_magnitude;
use super::vector;

pub use crate::vector::Vector;

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
        if self.len() == 0.0 {
            return *self;
        }

        let f = 1.0 / self.len();
        self.x = self.x * f;
        self.y = self.y * f;    // TODO: MACRO
        *self
    }
}
