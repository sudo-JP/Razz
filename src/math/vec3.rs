use std::ops::{Add, Mul, Sub, Div};
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub v: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            v: [x, y, z],
        }
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }
    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn unit_vector(self) -> Self {
        let mag = (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt();
        self / mag
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    let mut total: f64 = 0.; 
    total += v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z();
    total
}

// ----------------- Vec3 + Vec3 -----------------
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
            ],
        }
    }
}

// ----------------- Vec3 + f64 -----------------
impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            v: [
                self.v[0] + rhs,
                self.v[1] + rhs,
                self.v[2] + rhs,
            ],
        }
    }
}

// ----------------- Vec3 += Vec3 -----------------
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.v[i] += rhs.v[i];
        }
    }
}

// ----------------- Vec3 += f64 -----------------
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.v[i] += rhs;
        }
    }
}

// ----------------- Vec3 - Vec3 -----------------
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.v[i] -= rhs.v[i];
        }
    }
}

// ----------------- Vec3 - f64 -----------------
impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            v: [
                self.v[0] - rhs,
                self.v[1] - rhs,
                self.v[2] - rhs,
            ],
        }
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.v[i] -= rhs;
        }
    }
}

// ----------------- Vec3 * Vec3 -----------------
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] * rhs.v[0],
                self.v[1] * rhs.v[1],
                self.v[2] * rhs.v[2],
            ],
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.v[i] *= rhs.v[i];
        }
    }
}

// ----------------- Vec3 * f64 -----------------
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            v: [
                self.v[0] * rhs,
                self.v[1] * rhs,
                self.v[2] * rhs,
            ],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.v[i] *= rhs;
        }
    }
}

// ----------------- Vec3 / Vec3 -----------------
impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            v: [
                self.v[0] / rhs.v[0],
                self.v[1] / rhs.v[1],
                self.v[2] / rhs.v[2],
            ],
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.v[i] /= rhs.v[i];
        }
    }
}

// ----------------- Vec3 / f64 -----------------
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            v: [
                self.v[0] / rhs,
                self.v[1] / rhs,
                self.v[2] / rhs,
            ],
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.v[i] /= rhs;
        }
    }
}
