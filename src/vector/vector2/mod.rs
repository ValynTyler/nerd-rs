use super::Vector;

pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector<T> for Vector2<T> {
    fn norm(&self) -> T {
        todo!()
    }

    fn unit(&self) -> Self {
        todo!()
    }
}
