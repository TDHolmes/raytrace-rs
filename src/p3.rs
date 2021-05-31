use std::fs::File;
use std::io::prelude::*;

use super::color::{Color, Color3d};

pub struct P3File {
    file: File,
    width: usize,
    height: usize,
    max_value: usize,
}

impl P3File {
    pub fn new(path: &std::path::Path, width: usize, height: usize, max_value: usize) -> P3File {
        let file = match File::create(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        P3File {
            file,
            width,
            height,
            max_value,
        }
    }

    /// Writes the P3 header to the P3 file
    pub fn write_header(&mut self) -> std::io::Result<()> {
        Ok(write!(
            self.file,
            "P3\n{} {}\n{}\n",
            self.width, self.height, self.max_value
        )?)
    }

    /// Writes the color given to the P3 file
    pub fn write_color(&mut self, mut color: Color3d) -> std::io::Result<()> {
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        color.0 = f32::sqrt(color.0);
        color.1 = f32::sqrt(color.1);
        color.2 = f32::sqrt(color.2);

        Ok(write!(
            self.file,
            "{} {} {}\n",
            (color.red() * 255.) as usize,
            (color.green() * 255.) as usize,
            (color.blue() * 255.) as usize
        )?)
    }
}
