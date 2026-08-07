use crate::color_spaces::{Color, Hex, Hsb, Hsl, Rgb};
use crate::error::PaletteError;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn load_palette(palette_name: &str) -> Result<Vec<Color>, PaletteError> {
    let mut palette = Vec::new();

    let mut file = File::open(palette_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for (line_num, line) in contents.lines().enumerate() {
        let color: Vec<&str> = line.split(" ").collect();

        if color.len() != 6 {
            return Err(PaletteError::InvalidFormat(format!(
                "Line {}: expected 6 values, got {}",
                line_num + 1,
                color.len()
            )));
        }

        let rgb: Vec<&str> = color[0].split(",").collect();

        let r: u8 = rgb[0].trim().parse()?;
        let g: u8 = rgb[1].trim().parse()?;
        let b: u8 = rgb[2].trim().parse()?;
        let rgb = Rgb::new(r, g, b);

        let hsl: Vec<&str> = color[1].split(",").collect();

        let h: u16 = hsl[0].trim().parse()?;
        let s: u16 = hsl[1].trim().parse()?;
        let l: u16 = hsl[2].trim().parse()?;
        let hsl = Hsl::new(h, s, l);

        let hsb: Vec<&str> = color[2].split(",").collect();

        let h: u16 = hsb[0].trim().parse()?;
        let s: u16 = hsb[1].trim().parse()?;
        let b: u16 = hsb[2].trim().parse()?;
        let hsb = Hsb::new(h, s, b);

        let hex: u32 = color[3].trim().parse()?;
        let hex = Hex::new(hex);

        let name: String = color[4].trim().to_string().replace(",", " ");

        let locked: bool = color[5].trim().parse()?;

        let color = Color::new_raw(rgb, hsl, hsb, hex, name, locked);
        palette.push(color);
    }

    if palette.len() < 1 {
        return Err(PaletteError::Display(
            "not enough colors in cache".to_string(),
        ));
    }

    Ok(palette)
}

pub fn save_palette(palette_name: &str, palette: Vec<Color>) -> Result<(), PaletteError> {
    let mut file = File::create(palette_name)?;
    for color in palette {
        file.write(color.color_string().as_bytes())?;
    }

    Ok(())
}

pub fn palette_dir() -> Result<PathBuf, PaletteError> {
    let project_dirs = directories::ProjectDirs::from("", "", "palette-gen")
        .ok_or_else(|| PaletteError::Display("could not resolve data dir".into()))?;
    let dir = project_dirs.data_dir().to_path_buf();
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn list_palette_names() -> Result<Vec<String>, PaletteError> {
    let dir = palette_dir()?;
    let mut names = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            if let Some(name) = entry.file_name().to_str() {
                if name != "cache" {
                    names.push(name.to_string());
                }
            }
        }
    }
    names.sort();
    Ok(names)
}

// --------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_math::generate_palette;
    //use crate::color_spaces::{Color, Hex};

    // #[test]
    // fn test_save_palette() {
    //     let palette = Vec::from([
    //         Color::new(Hex::new(0xf2d7ee).encode()),
    //         Color::new(Hex::new(0xd3bcc0).encode()),
    //         Color::new(Hex::new(0x69306d).encode()),
    //         Color::new(Hex::new(0x0e103d).encode()),
    //         Color::new(Hex::new(0xe83151).encode()),
    //     ]);
    //     let result_palette = match palette.into_iter().collect() {
    //         Ok(vec) => vec,
    //         Err(e) => panic!("{e}"),
    //     };

    //     match save_palette("test", result_palette) {
    //         Ok(_) => return,
    //         Err(e) => panic!("{e}"),
    //     };
    // }

    #[test]
    fn save_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("cache");
        let path_str = path.to_str().unwrap();

        let original = generate_palette(3).unwrap();
        save_palette(path_str, original.clone()).unwrap();
        let loaded = load_palette(path_str).unwrap();

        assert_eq!(original, loaded);
    }
}
