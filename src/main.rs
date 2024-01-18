use nerd::{vector::Vec2, matrix::Mat2x2};

extern crate nerd;

fn main() {

    let vec = Vec2::new(1.0, 1.0);
    let mat = Mat2x2::new(
        [
            [0., -1.],
            [1., 0.],
        ]
    );


    println!("{}", mat * vec);

}