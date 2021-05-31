//! Implements a simple 3d vector to represent a point. also used for color.
use std::fmt::{Debug, Formatter, Result};
use std::{default, ops};

#[derive(Copy, Clone)]
pub struct Vec3d(pub f32, pub f32, pub f32);

impl Debug for Vec3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<{:0.3}, {:0.3}, {:0.3}>", self.0, self.1, self.2)
    }
}

impl Vec3d {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3d {
        Vec3d(x, y, z)
    }

    pub fn random() -> Vec3d {
        return Vec3d::new(rand::random(), rand::random(), rand::random());
    }

    pub fn random_in_range(min: f32, max: f32) -> Vec3d {
        return Vec3d::new(
            rand::random::<f32>() * max + min,
            rand::random::<f32>() * max + min,
            rand::random::<f32>() * max + min,
        );
    }

    pub fn random_in_unit_sphere() -> Vec3d {
        loop {
            let p = Vec3d::random_in_range(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3d {
        Vec3d::random_in_unit_sphere().unit_vector()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, v: &Vec3d) -> f32 {
        return self.0 * v.0 + self.1 * v.1 + self.2 * v.2;
    }

    pub fn cross(&self, v: &Vec3d) -> Vec3d {
        return Vec3d::new(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        );
    }

    pub fn unit_vector(self) -> Vec3d {
        return self / self.length();
    }

    pub fn near_zero(&self) -> bool {
        let delta: f32 = 1e-8;
        self.0.abs() < delta && self.1.abs() < delta && self.2.abs() < delta
    }

    pub fn reflect(&self, normal: &Vec3d) -> Vec3d {
        return (*self) - 2. * self.dot(normal) * (*normal);
    }
}

impl default::Default for Vec3d {
    fn default() -> Self {
        Vec3d::new(0., 0., 0.)
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

impl ops::Mul<Vec3d> for Vec3d {
    type Output = Self;

    fn mul(self, rhs: Vec3d) -> Self {
        Vec3d::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
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

impl ops::Mul<Vec3d> for f32 {
    type Output = Vec3d;

    fn mul(self, rhs: Vec3d) -> Vec3d {
        rhs * self
    }
}

impl ops::Div<Vec3d> for f32 {
    type Output = Vec3d;

    fn div(self, rhs: Vec3d) -> Vec3d {
        Vec3d::new(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}

impl ops::Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.0, -self.1, -self.2)
    }
}
