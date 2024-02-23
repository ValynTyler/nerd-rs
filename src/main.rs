use nerd::{matrix::{Mat4, Matrix}, vector::{Vec3, Vector, Vector4}};

extern crate nerd;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

fn main() {

    let v = Vec3::new(1., 0., 0.);
    let w = Vec3::new(0., 1., 0.);

    println!("{:?}", v);
    println!("{:?}", w);
    println!("{:?}", w.cross(v));
}