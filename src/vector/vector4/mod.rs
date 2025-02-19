use super::Vector;

pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T> Vector<T> for Vector4<T> {
    fn norm(&self) -> T {
        todo!()
    }

    fn unit(&self) -> Self {
        todo!()
    }
}
