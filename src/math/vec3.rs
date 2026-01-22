use std::ops::{Add, Mul, Sub, Div};
use std::ops::{AddAssign, MulAssign, SubAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3 {
    pub v: [i32; 3]
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            v: [x, y, z],
        }
    }

    pub fn x(&self) -> i32 {
        self.v[0]
    }

    pub fn y(&self) -> i32 {
        self.v[1]
    }
    pub fn z(&self) -> i32 {
        self.v[2]
    }

    pub fn unit_vector(self) -> Self {
        self / 3
    }
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

// ----------------- Vec3 + i32 -----------------
impl Add<i32> for Vec3 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
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

// ----------------- Vec3 += i32 -----------------
impl AddAssign<i32> for Vec3 {
    fn add_assign(&mut self, rhs: i32) {
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

// ----------------- Vec3 - i32 -----------------
impl Sub<i32> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            v: [
                self.v[0] - rhs,
                self.v[1] - rhs,
                self.v[2] - rhs,
            ],
        }
    }
}

impl SubAssign<i32> for Vec3 {
    fn sub_assign(&mut self, rhs: i32) {
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

// ----------------- Vec3 * i32 -----------------
impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            v: [
                self.v[0] * rhs,
                self.v[1] * rhs,
                self.v[2] * rhs,
            ],
        }
    }
}

impl MulAssign<i32> for Vec3 {
    fn mul_assign(&mut self, rhs: i32) {
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

// ----------------- Vec3 / i32 -----------------
impl Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            v: [
                self.v[0] / rhs,
                self.v[1] / rhs,
                self.v[2] / rhs,
            ],
        }
    }
}

impl DivAssign<i32> for Vec3 {
    fn div_assign(&mut self, rhs: i32) {
        for i in 0..3 {
            self.v[i] /= rhs;
        }
    }
}
