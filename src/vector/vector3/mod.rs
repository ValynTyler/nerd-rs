use super::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vector<T> for Vector3<T> {
    fn norm(&self) -> T {
        todo!()
    }

    fn unit(&self) -> Self {
        todo!()
    }
}
