use nerd::{matrix::Mat4, vector::Vec3};

extern crate nerd;

fn main() {
    let vec: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let mat: Mat4 = Mat4::from_translation(vec);
    println!("{}", mat);
}