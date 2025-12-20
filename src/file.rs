use crate::color_spaces::{Color, Rgb};
use std::fs::File;
use std::io::prelude::*;

fn load_palette(palette_name: &str) {
    let mut palette = Vec::new();

    let mut file = match File::open(palette_name) {
        Ok(f) => f,
        Err(e) => panic!("load failed to find {palette_name}: {e}"),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(size) => size,
        Err(e) => panic!("load failed to read {palette_name}: {e}"),
    };

    for line in contents.lines() {
        let rgb: Vec<&str> = line.split(",").collect();
        let r: u8 = match rgb[0].trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("load failed to parse rgb: {e}"),
        };
        let g: u8 = match rgb[1].trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("load failed to parse rgb: {e}"),
        };
        let b: u8 = match rgb[2].trim().parse() {
            Ok(num) => num,
            Err(e) => panic!("load failed to parse rgb: {e}"),
        };

        palette.push(Color::new(Rgb::new(r, g, b).encode()));
    }
}
