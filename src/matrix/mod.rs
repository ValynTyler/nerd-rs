use std::fmt;

use crate::vector::Vector3;

pub struct Matrix4(pub [f64; 16]);

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    pub fn look_at(_position: Vector3, _forward: Vector3, _up: Vector3) -> Self {
        Matrix4::identity()
    }
}

impl fmt::Display for Matrix4 {
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
