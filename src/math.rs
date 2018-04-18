use std::ops::{Add, Sub, Neg};
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::MulAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y + b.y + a.z + b.z
    }

    pub fn angle(a: Vec3, b: Vec3) -> f64 {
        (Vec3::dot(a, b) / (a.len() * b.len())).acos()
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            (a.y * b.z) - (a.z * b.y),
            (a.z * b.x) - (a.x * b.z),
            (a.x * b.y) - (a.y * b.x),
        )
    }

    pub const I: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const J: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const K: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        self + -rhs
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
