use crate::complex::{self, Complex};

/// Returns wheter or not complex number C(x, y) is part of the
/// Mandelbrot set.
/// 'iters' indicates the number of iterations the algorithm evaluates,
/// which determines the fractal detail of the set.
#[allow(dead_code)]
pub fn is_part_of(constant: complex::Complex, thresh: f32, max_iters: i32, iters: &mut i32) -> bool {
    let mut z = Complex {
        real: 0.0,
        imag: 0.0,
    };

    for i in 0..max_iters {
        if z.square_magnitude() > thresh {
            *iters = i;
            return false;
        }
        z = z * z + constant;
    }
    return true;
}
