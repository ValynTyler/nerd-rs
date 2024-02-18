use nerd::matrix::Matrix4;

extern crate nerd;

fn main() {
    let mat: Matrix4 = Matrix4::identity();
    println!("{}", mat);
}