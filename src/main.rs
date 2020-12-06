use std::fs::File;
use std::path::Path;

use raytracing::prelude::*;
use raytracing::{color::Color3d, p3, ray::Ray, vec::Vec3d};  // point::Point3d

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_HEIGHT: usize = 400;
const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

pub fn ray_color(ray: &Ray) -> Color3d {
    let unit_direction: Vec3d = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return  Color3d::new(1.0, 1.0, 1.0) * (1.0 - t) + Color3d::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.ppm");
    let display = path.display();

    // Open a new file for writing
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // write the file header
    p3::write_header(&mut file, IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    // fill in a dummy color
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let mut color = Color3d::new(
                x as f32 / IMAGE_WIDTH as f32,
                y as f32 / IMAGE_HEIGHT as f32,
                0.25,
            );
            color = color * 256.0_f32;
            p3::write_color(&mut file, &color).unwrap();
        }
    }
}
