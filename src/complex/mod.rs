use std::ops;

#[derive(Copy, Clone)]
pub struct Complex {
    pub real: f32,
    pub imag: f32,
}

#[allow(dead_code)]
impl Complex {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    pub fn print(&self) {
        print!("{}+{}i", self.real, self.imag)
    }

    pub fn println(&self) {
        println!("{}+{}i", self.real, self.imag)
    }

    pub fn square_magnitude(&self) -> f32 {
        return self.real * self.real + self.imag * self.imag;
    }

    pub fn magnitude(&self) -> f32 {
        return self.square_magnitude().sqrt();
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, _rhs: Complex) -> Complex {
        return Complex {
            real: self.real + _rhs.real,
            imag: self.imag + _rhs.imag,
        };
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, _rhs: Complex) -> Complex {
        return Complex {
            real: self.real - _rhs.real,
            imag: self.imag - _rhs.imag,
        };
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        // a, b ∈ C
        //
        // A = a + xi
        // B = b + yi
        // C = c + zi
        //
        // A * B = C
        //       = (a + xi) * (b + yi)
        //       = (a*b) + (a*yi) + (xi*b) + (xi*yi)
        //       = (a*b) + (a*yi) + (xi*b) + i^2(x*y)
        //       = (a*b) + (a*yi) + (xi*b) - (x*y)
        //          ---     ----     ----     ---
        //           r       im       im       r
        //
        // => { c  = (a*b) - (x*y)
        //    { zi = (a*yi) + (xi*b)
        //
        return Complex {
            real: (self.real * rhs.real) - (self.imag * rhs.imag),
            imag: (self.real * rhs.imag) + (self.imag * rhs.real),
        };
    }
}

// TODO:
// [x] Multiplication
// [ ] Division
// [ ] interop with real numbers
