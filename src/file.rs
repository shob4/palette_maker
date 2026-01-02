use crate::color_spaces::{Color, Rgb};
use crate::error::PaletteError;
use std::fs::File;
use std::io::prelude::*;

pub fn load_palette(palette_name: &str) -> Result<Vec<Color>, PaletteError> {
    let mut palette = Vec::new();

    let mut file = File::open(palette_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for (line_num, line) in contents.lines().enumerate() {
        let rgb: Vec<&str> = line.split(",").collect();

        if rgb.len() != 3 {
            return Err(PaletteError::InvalidFormat(format!(
                "Line {}: expected 3 values, got {}",
                line_num + 1,
                rgb.len()
            )));
        }

        let r: u8 = rgb[0].trim().parse()?;
        let g: u8 = rgb[1].trim().parse()?;
        let b: u8 = rgb[2].trim().parse()?;

        palette.push(Color::new(Rgb::new(r, g, b).encode())?);
    }

    Ok(palette)
}

pub fn save_palette(palette_name: &str, palette: Vec<Color>) -> Result<(), PaletteError> {
    let mut file = match File::open(palette_name) {
        Ok(file) => file,
        Err(_) => File::create(palette_name)?,
    };
    for color in palette {
        file.write(color.rgb_to_string().as_bytes())?;
    }

    Ok(())
}

// --------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_spaces::{Color, Hex};

    #[test]
    fn test_save_palette() {
        let palette = Vec::from([
            Color::new(Hex::new(0xf2d7ee).encode()),
            Color::new(Hex::new(0xd3bcc0).encode()),
            Color::new(Hex::new(0x69306d).encode()),
            Color::new(Hex::new(0x0e103d).encode()),
            Color::new(Hex::new(0xe83151).encode()),
        ]);
        let result_palette = match palette.into_iter().collect() {
            Ok(vec) => vec,
            Err(e) => panic!("{e}"),
        };

        match save_palette("test", result_palette) {
            Ok(_) => return,
            Err(e) => panic!("{e}"),
        };
    }

    #[test]
    fn test_load_palette() {
        let palette = Vec::from([
            Color::new(Hex::new(0xf2d7ee).encode()),
            Color::new(Hex::new(0xd3bcc0).encode()),
            Color::new(Hex::new(0x69306d).encode()),
            Color::new(Hex::new(0x0e103d).encode()),
            Color::new(Hex::new(0xe83151).encode()),
        ]);
        let result_palette = match palette.into_iter().collect() {
            Ok(vec) => vec,
            Err(e) => panic!("{e}"),
        };

        let test_palette = match load_palette("test") {
            Ok(vec) => vec,
            Err(e) => panic!("{e}"),
        };
        assert_eq!(test_palette, result_palette);
    }
}
