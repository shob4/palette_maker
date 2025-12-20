use crate::color_spaces::{Color, Rgb};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn load_palette(palette_name: &str) -> Result<Vec<Color>, Box<dyn Error> {
    let mut palette = Vec::new();

    let mut file = File::open(palette_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        let rgb: Vec<&str> = line.split(",").collect();

        let r: u8 = rgb[0].trim().parse()?;
        let g: u8 = rgb[1].trim().parse()?;
        let b: u8 = rgb[2].trim().parse()?;

        palette.push(Color::new(Rgb::new(r, g, b).encode()));
    }

    Ok(palette)
}

fn save_palette(palette_name: &str) {}
