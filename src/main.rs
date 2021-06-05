use std::path::Path;

use raytracing::{
    camera::{Camera, ASPECT_RATIO},
    circle::Circle,
    color::Color3d,
    hit::{HitList, Hittable},
    material, p3,
    point::Point3d,
    prelude::*,
    ray::Ray,
    vec::Vec3d,
};

// constants

const IMAGE_HEIGHT: usize = 400;
const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f32 * ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 50;
const RECURSION_LIMIT: usize = 5;

// Materials

const DULL_METAL: material::Metal = material::Metal::new(Color3d::new(0.5, 0.5, 0.5), 0.8);
const SHINY_METAL: material::Metal = material::Metal::new(Color3d::new(0.95, 0.95, 0.95), 0.2);
const LIGHT_GREEN: material::Lambertian = material::Lambertian::new(Color3d::new(0., 0.4, 0.));
const BRIGHT_BLUE: material::Lambertian = material::Lambertian::new(Color3d::new(0., 0., 0.97));

pub fn ray_color(ray: &Ray, world: &dyn Hittable, recursion_depth: usize) -> Color3d {
    // don't scatter forever
    if recursion_depth == 0 {
        return Color3d::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f32::INFINITY) {
        // Difuse material - scatter and send off
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color3d = Default::default();

        // scatter on the material, then go to next material.
        hit.material
            .scatter(ray, &hit, &mut attenuation, &mut scattered);
        if attenuation.near_zero() {
            return Color3d::new(0., 0., 0.);
        }
        return attenuation * ray_color(&scattered, world, recursion_depth - 1);
    }

    // Background
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
    hitlist.add(Box::new(Circle::new(
        100.,
        Point3d::new(0., -100.5, -1.),
        &LIGHT_GREEN,
    )));
    hitlist.add(Box::new(Circle::new(
        0.5,
        Point3d::new(0., 0., -1.),
        &BRIGHT_BLUE,
    )));
    hitlist.add(Box::new(Circle::new(
        0.5,
        Point3d::new(-1., 0., -1.),
        &SHINY_METAL,
    )));
    hitlist.add(Box::new(Circle::new(
        0.5,
        Point3d::new(1., 0., -1.),
        &DULL_METAL,
    )));

    // Camera
    let camera = Camera::new();

    // render the scene
    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            // anti-aliasing
            let mut pixel_color: Color3d = Color3d::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let v = (y as f32 + fastrand::f32()) / (IMAGE_HEIGHT - 1) as f32;
                let u = (x as f32 + fastrand::f32()) / (IMAGE_WIDTH - 1) as f32;
                let r = camera.get_ray(u, v);

                let sub_pixel_color = ray_color(&r, &hitlist, RECURSION_LIMIT);
                pixel_color = pixel_color + sub_pixel_color;
            }

            pixel_color = pixel_color / SAMPLES_PER_PIXEL as f32;
            p3.write_color(pixel_color).unwrap();
        }
    }
}
