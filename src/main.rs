use nerd::vector::Vec2;

extern crate nerd;

fn main() {

    let v = Vec2::new(3., 4.);
    let w = Vec2::new(2., 3.);

    println!("{}", -v + w + 5.);
}