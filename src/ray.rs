use std::fmt::{Debug, Formatter, Result};

use super::{point::Point3d, vec::Vec3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Debug for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Ray<{:?} -> {:?}>", self.origin, self.direction)
    }
}

impl Ray {
    pub fn new(origin: Point3d, direction: Vec3d) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, time: f32) -> Point3d {
        self.origin + self.direction * time
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray::new(Point3d::new(0., 0., 0.), Vec3d::new(1., 0., 0.))
    }
}
