use std::{fmt, ops};

use crate::vector::{Vector3, Vector, Vector4};

pub trait Matrix {
    fn identity() -> Self;
    fn flip(self) -> Self;
    fn get(&self, i: usize, j: usize) -> f32;
    fn set(&mut self, i: usize, j: usize, value: f32);
    fn as_ptr(&self) -> *const f32;
}

pub struct Matrix3(pub [f32; 9]);

impl Matrix3 {
    pub fn rotation_x(rotation: f32) -> Self {
        let theta = rotation;
        Self([
            1.0, 0.0,              0.0,
            0.0, f32::cos(theta), -f32::sin(theta),
            0.0, f32::sin(theta),  f32::cos(theta),
        ])
    }

    pub fn rotation_y(rotation: f32) -> Self {
        let theta = rotation;
        Self([
             f32::cos(theta), 0.0, f32::sin(theta),
             0.0,             1.0, 0.0,            
            -f32::sin(theta), 0.0, f32::cos(theta),
        ])
    }

    pub fn rotation_axis(axis: Vector3, angle: f32) -> Self {
        let r = axis;
        let theta = angle;
        
        let sin = theta.sin();
        let cos = theta.cos();

        Matrix3([
            cos + r.x*r.x*(1.0 - cos),        r.x*r.y*(1.0 - cos) - r.z*sin,     r.x*r.z*(1.0 - cos) + r.y*sin,
            r.y*r.x*(1.0 - cos) + r.z*sin,    cos + r.y*r.y*(1.0 - cos),         r.y*r.z*(1.0 - cos) - r.x*sin,
            r.z*r.x*(1.0 - cos) - r.y*sin,    r.z*r.y*(1.0 - cos) + r.x*sin,    cos + r.z*r.z*(1.0 - cos),
        ])
    }
}

impl Matrix for Matrix3 {
    fn identity() -> Self {
        Matrix3([
            1., 0., 0.,
            0., 1., 0.,
            0., 0., 1.,
        ])
    }

    fn flip(self) -> Self {
        todo!()
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        self.0[(i-1) * 3 + (j-1)]
    }

    fn set(&mut self, i: usize, j: usize, value: f32) {
        self.0[(i-1) * 3 + (j-1)] = value;
    }

    fn as_ptr(&self) -> *const f32 {
        todo!()
    }
}

impl ops::Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let mut vec = Vector3::ZERO;

        for i in 1..4 {
            let mut sum: f32 = 0.0;
            for k in 1..4 {
                sum += self.get(i, k) * rhs.get(k);  // TODO: Turn into a MACRO
            }
            vec.set(i, sum);
        }

        vec
    }
}

impl ops::Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, rhs: Matrix3) -> Self::Output {
        let mut mat = Matrix3::identity();

        for i in 1..4 {
            for j in 1..4 {
                let mut sum: f32 = 0.0;
                for k in 1..4 {
                    sum += self.get(i, k) * rhs.get(k, j);  // TODO: Turn into a MACRO
                }
                mat.set(i, j, sum);
            }
        }

        mat
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..3 {
            write!(
                f,
                "[{} {} {}]\n",
                self.0[0 + i*3],
                self.0[1 + i*3],
                self.0[2 + i*3],
            )
            .unwrap();
        }
        write!(f, "")
    }
}

pub struct Matrix4(pub [f32; 16]);

impl Matrix for Matrix4 {
    fn identity() -> Self {
        Matrix4([
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        ])
    }

    fn flip(self) -> Self {
        Matrix4([
            self.get(1, 1), self.get(2, 1), self.get(3, 1), self.get(4, 1),
            self.get(1, 2), self.get(2, 2), self.get(3, 2), self.get(4, 2),
            self.get(1, 3), self.get(2, 3), self.get(3, 3), self.get(4, 3),
            self.get(1, 4), self.get(2, 4), self.get(3, 4), self.get(4, 4),
        ])
    }

    fn get(&self, i: usize, j: usize) -> f32 {
        self.0[(i-1) * 4 + (j-1)]
    }

    fn set(&mut self, i: usize, j: usize, value: f32) {
        self.0[(i-1) * 4 + (j-1)] = value;
    }

    fn as_ptr(&self) -> *const f32 {
        &self.0[0]
    } 
}

impl Matrix4 {
    pub fn look_at(eye: Vector3, center: Vector3, up: Vector3) -> Matrix4 {
        Matrix4::look_at_dir(eye, center - eye, up)
    }

    pub fn look_at_dir(eye: Vector3, dir: Vector3, up: Vector3) -> Self {
        let mut f = dir;
        let f = f.normalize();
        let r = f.cross(up).normalize();
        let u = r.cross(f);

        Matrix4([
            r.x.clone(), u.x.clone(), -f.x.clone(), 0.0,
            r.y.clone(), u.y.clone(), -f.y.clone(), 0.0,
            r.z.clone(), u.z.clone(), -f.z.clone(), 0.0,
            -eye.dot(r), -eye.dot(u), eye.dot(f), 1.0,
        ]).flip()
    }

    pub fn from_translation(translation: Vector3) -> Self {
        let v = translation;
        Matrix4([
            1.0, 0.0, 0.0, v.x,
            0.0, 1.0, 0.0, v.y,
            0.0, 0.0, 1.0, v.z,
            0.0, 0.0, 0.0,  1.0,
        ])
    }

    pub fn rotation_x(rotation: f32) -> Self {
        let theta = rotation;
        Matrix4([
            1.0, 0.0,              0.0,             0.0,
            0.0, f32::cos(theta), -f32::sin(theta), 0.0,
            0.0, f32::sin(theta),  f32::cos(theta), 0.0,
            0.0, 0.0,              0.0,             1.0,
        ])
    }

    pub fn rotation_y(rotation: f32) -> Self {
        let theta = rotation;
        Matrix4([
             f32::cos(theta), 0.0, f32::sin(theta), 0.0,
             0.0,             1.0, 0.0,             0.0,
            -f32::sin(theta), 0.0, f32::cos(theta), 0.0,
             0.0,             0.0, 0.0,             1.0,
        ])
    }

    pub fn rotation_axis(axis: Vector3, angle: f32) -> Self {
        let mut r = axis;
        let r = r.normalize();
        let theta = angle;
        
        let sin = theta.sin();
        let cos = theta.cos();

        Matrix4([
            cos + r.x*r.x*(1.0 - cos),        r.x*r.y*(1.0 - cos) - r.z*sin,     r.x*r.z*(1.0 - cos) + r.y*sin, 0.0,
            r.y*r.x*(1.0 - cos) + r.z*sin,    cos + r.y*r.y*(1.0 - cos),         r.y*r.z*(1.0 - cos) - r.x*sin, 0.0,
            r.z*r.x*(1.0 - cos) - r.y*sin,    r.z*r.y*(1.0 - cos) + r.x*sin,    cos + r.z*r.z*(1.0 - cos),      0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Matrix4 {
        Matrix4([
            2./(right-left),  0.,               0.,              -(right+left) / (right-left),
            0.,               2./(top-bottom),  0.,              -(top+bottom) / (top-bottom),
            0.,               0.,               2./(far-near),   -(far+near)   / (far-near),
            0.,               0.,               0.,               1.,
        ])
    }
}

impl ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4::new(
            self.get(1, 1) * rhs.x + self.get(1, 2) * rhs.y + self.get(1, 3) * rhs.z + self.get(1, 4) * rhs.w,
            self.get(2, 1) * rhs.x + self.get(2, 2) * rhs.y + self.get(2, 3) * rhs.z + self.get(2, 4) * rhs.w,
            self.get(3, 1) * rhs.x + self.get(3, 2) * rhs.y + self.get(3, 3) * rhs.z + self.get(3, 4) * rhs.w,
            self.get(4, 1) * rhs.x + self.get(4, 2) * rhs.y + self.get(4, 3) * rhs.z + self.get(4, 4) * rhs.w,
        )
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut mat = Matrix4::identity();

        for i in 1..5 {
            for j in 1..5 {
                let mut sum: f32 = 0.0;
                for k in 1..5 {
                    sum += self.get(i, k) * rhs.get(k, j);  // TODO: Turn into a MACRO
                }
                mat.set(i, j, sum);
            }
        }

        mat
    }
}

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..4 {
            write!(
                f,
                "[{} {} {} {}]\n",
                self.0[0 + i*4],
                self.0[1 + i*4],
                self.0[2 + i*4],
                self.0[3 + i*4]
            )
            .unwrap();
        }
        write!(f, "")
    }
}
