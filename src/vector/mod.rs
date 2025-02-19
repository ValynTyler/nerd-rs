pub mod vector2;
pub mod vector3;
pub mod vector4;

pub trait Vector<T> {
    fn norm(&self) -> T;
    fn unit(&self) -> Self;
}
