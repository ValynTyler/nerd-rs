extern crate nerd;

use nerd::complex::*;

fn main() {
    let c = Complex {
        real: 0.2,
        imag: 0.1,
    };

    // c.println();

    let mut a = 0;
    println!("{}", nerd::mandelbrot::is_part_of(c, 4.0, 100, &mut a));
}