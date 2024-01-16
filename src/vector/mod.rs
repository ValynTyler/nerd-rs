use std::{ops, fmt};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub const ONE: Vec2 = Vec2::new(1., 1.);
    pub const UP: Vec2 = Vec2::new(1., 0.);
    pub const DOWN: Vec2 = Vec2::new(-1., 0.);

    pub const fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn sq_mag(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }

    pub fn mag(&self) -> f64 {
        f64::sqrt(self.sq_mag())
    }

    pub fn normalized(&self) -> Vec2 {
        let coef: f64 = 1./self.mag();
        *self * coef
    }

    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    pub fn dot(&self, rhs: Vec2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Add<f64> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f64) -> Self::Output {
        self + Vec2::ONE * rhs
    }
}

impl ops::Sub<f64> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Vec2::ONE * rhs
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

// pub struct Vec3 {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// pub struct Vec4 {
//     x: f64,
//     y: f64,
//     z: f64,
//     w: f64
// }