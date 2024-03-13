mod vector2;
pub use crate::vector::vector2::*;

mod vector3;
pub use crate::vector::vector3::*;

mod vector4;
pub use crate::vector::vector4::*;

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
pub (crate) use vector;

macro_rules! square_magnitude {
    // Base case:
    ($x:expr) => ($x * $x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        (square_magnitude!($x) + square_magnitude!($($y),+))
    )
}
pub (crate) use square_magnitude;

pub trait Vector {
    fn len(&self) -> f32;
    fn normalize(&mut self) -> Self;
}
