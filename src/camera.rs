use super::{point::Point3d, ray::Ray, vec::Vec3d};

#[derive(Debug, PartialEq)]
pub struct Degrees(f32);

#[derive(Debug, PartialEq)]
pub struct Radians(f32);

const DEGREES_TO_RADIANS: f32 = std::f32::consts::PI / 180.;

impl Degrees {
    pub fn new(val: f32) -> Degrees {
        Degrees { 0: val }
    }
}

impl Radians {
    pub fn new(val: f32) -> Radians {
        Radians { 0: val }
    }
}

impl From<Radians> for Degrees {
    fn from(val: Radians) -> Self {
        Degrees::new(val.0 / DEGREES_TO_RADIANS)
    }
}

impl From<Degrees> for Radians {
    fn from(val: Degrees) -> Self {
        Radians::new(val.0 * DEGREES_TO_RADIANS)
    }
}

impl std::ops::Deref for Degrees {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for Radians {
    type Target = f32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Camera {
    origin: Point3d,
    lower_left_corner: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
}

impl Camera {
    pub fn new(
        lookfrom: Point3d,
        lookat: Point3d,
        vup: Vec3d,
        vert_fov: Degrees,
        aspect_ratio: f32,
    ) -> Camera {
        // Camera
        let theta: Radians = vert_fov.into();
        let h = (*theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3d::unit_vector(lookfrom - lookat);
        let u = Vec3d::unit_vector(Vec3d::cross(&vup, &w));
        let v = Vec3d::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rads_to_degrees() {
        let rad_one_pi = Radians::new(std::f32::consts::PI);
        let deg: Degrees = rad_one_pi.into();

        assert_eq!(*deg, 180.);
    }

    #[test]
    fn degrees_to_rads() {
        let deg_180 = Degrees::new(180.);
        let rad: Radians = deg_180.into();

        assert_eq!(*rad, std::f32::consts::PI);
    }
}
