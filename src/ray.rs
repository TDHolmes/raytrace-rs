use super::{point::Point3d, vec::Vec3d};

pub struct Ray {
    pub origin: Point3d,
    pub direction: Vec3d,
}

impl Ray {
    pub fn at(&self, time: usize) -> Point3d {
        self.origin + self.direction * time as f32
    }
}
