use nerd::{matrix::{Matrix4, Matrix}, vector::{Vector3, Vector, Vector4}};

extern crate nerd;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

fn main() {

    let v = Vector3::new(3., 0., 0.);
    let w = Vector4::new(-1., 1., 0., 0.);

    println!("{:?}", Matrix4::look_at(Vector3::ZERO, v, Vector3::UP) * w);
}