use std::fs::File;
use std::io::prelude::*;

use super::color::{Color, Color3d};

/// Writes the P3 header to the P3 file
pub fn write_header(file: &mut File, width: usize, height: usize) -> std::io::Result<()> {
    Ok(write!(file, "P3\n{} {}\n255\n", width, height)?)
}

/// Writes the color given to the P3 file
pub fn write_color(file: &mut File, color: &Color3d) -> std::io::Result<()> {
    Ok(write!(
        file,
        "{} {} {}\n",
        color.red() as usize,
        color.green() as usize,
        color.blue() as usize
    )?)
}
