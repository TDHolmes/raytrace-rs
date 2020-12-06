//! Implements a simple 3d vector to represent a point. also used for color.
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3d(pub f32, pub f32, pub f32);

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d(x, y, z)
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Vec3d) -> f32 {
        return self.0 * v.0
             + self.1 * v.1
             + self.2 * v.2;
    }

    pub fn cross(&self, v: &Vec3d) -> Vec3d {
        return Vec3d::new(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0
        );
    }

    pub fn unit_vector(self) -> Vec3d {
        return self / self.length();
    }
}

// --- Operators

impl ops::Add<Vec3d> for Vec3d {
    type Output = Self;

    fn add(self, rhs: Vec3d) -> Self {
        Vec3d::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3d> for Vec3d {
    type Output = Self;

    fn sub(self, rhs: Vec3d) -> Self {
        Vec3d::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f32> for Vec3d {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec3d::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Div<f32> for Vec3d {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Vec3d::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
