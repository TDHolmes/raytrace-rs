use std::path::Path;

use raytracing::{
    circle::Circle,
    color::Color3d,
    hit::{HitList, Hittable},
    p3,
    point::Point3d,
    prelude::*,
    ray::Ray,
    vec::Vec3d,
};

const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_HEIGHT: usize = 400;
const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as usize;

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color3d {
    if let Some(hit) = world.hit(ray, 0., f32::INFINITY) {
        return 0.5 * (hit.normal + Color3d::new(1., 1., 1.));
    }

    let unit_direction: Vec3d = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color3d::new(1.0, 1.0, 1.0) + t * Color3d::new(0.5, 0.7, 1.0);
}

fn main() {
    // Create our image file
    let path = Path::new("hello.ppm");
    let mut p3 = p3::P3File::new(path, IMAGE_WIDTH, IMAGE_HEIGHT, 255);
    p3.write_header().unwrap();

    // World
    let mut hitlist = HitList::new();
    hitlist.add(Box::new(Circle::new(100., Point3d::new(0., -100.5, -1.))));
    hitlist.add(Box::new(Circle::new(0.5, Point3d::new(0., 0., -1.))));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3d::new(0., 0., 0.);
    let horizontal = Vec3d::new(viewport_width, 0., 0.);
    let vertical = Vec3d::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3d::new(0., 0., focal_length);

    // render the scene
    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let u = (x as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (y as f32) / (IMAGE_HEIGHT - 1) as f32;
            let r: Ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let pixel_color = ray_color(&r, &hitlist);
            p3.write_color(&pixel_color).unwrap();
        }
    }
}
