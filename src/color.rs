use super::vec;

pub type Color3d = vec::Vec3d;

pub trait Color {
    fn red(&self) -> f32;
    fn green(&self) -> f32;
    fn blue(&self) -> f32;
}

impl Color for Color3d {
    fn red(&self) -> f32 {
        self.0
    }

    fn green(&self) -> f32 {
        self.1
    }

    fn blue(&self) -> f32 {
        self.2
    }
}
