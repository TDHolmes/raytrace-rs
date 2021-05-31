use super::{point::Point3d, ray::Ray, vec::Vec3d};

pub const ASPECT_RATIO: f32 = 16. / 9.;

pub struct Camera {
    origin: Point3d,
    lower_left_corner: Point3d,
    horizontal: Vec3d,
    vertical: Vec3d,
}

impl Camera {
    pub fn new() -> Camera {
        // Camera
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3d::new(0., 0., 0.);
        let horizontal = Vec3d::new(viewport_width, 0., 0.);
        let vertical = Vec3d::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3d::new(0., 0., focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
