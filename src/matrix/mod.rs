use std::{fmt, ops};

use crate::vector::{Vec3, Vector4};

pub trait Matrix {
    fn identity() -> Self;
    fn as_ptr(&self) -> *const f32;
}

pub struct Mat4(pub [f32; 16]);

impl Matrix for Mat4 {
    fn identity() -> Self {
        Mat4([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    fn as_ptr(&self) -> *const f32 {
        &self.0[0]
    } 
}

impl Mat4 {
    pub fn look_at(_position: Vec3, _forward: Vec3, _up: Vec3) -> Self {
        Mat4::identity()
    }

    pub fn from_translation(translation: Vec3) -> Self {
        let v = translation;
        Mat4::identity() * Vector4::new(v.x, v.y, v.z, 1.0)
    }
}

impl ops::Mul<Vector4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        
    }
}

impl fmt::Display for Mat4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            write!(
                f,
                "[{} {} {} {}]\n",
                self.0[0 + i*4],
                self.0[1 + i*4],
                self.0[2 + i*4],
                self.0[3 + i*4]
            )
            .unwrap();
        }
        write!(f, "")
    }
}
