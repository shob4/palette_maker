use crate::{
    color_spaces::{Color, Hsl, Rgb},
    error::PaletteError,
};
use rand::prelude::*;

// TODO

pub fn complement(hsl: &Hsl) -> Hsl {
    let new_h = (hsl.h + 180).rem_euclid(360);
    Hsl::new(new_h, hsl.s, hsl.l)
}

pub fn triad(hsl: &Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h as i32 - 120).rem_euclid(360);
    let right = (hsl.h + 120).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn square(hsl: &Hsl) -> (Hsl, Hsl, Hsl) {
    let left = (hsl.h as i32 - 90).rem_euclid(360);
    let middle = (hsl.h + 180).rem_euclid(360);
    let right = (hsl.h + 90).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let middle = Hsl::new(middle, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, middle, right)
}

pub fn analogous(hsl: Hsl) -> (Hsl, Hsl) {
    let left = (hsl.h as i32 - 30).rem_euclid(360);
    let right = (hsl.h + 30).rem_euclid(360);

    let left = Hsl::new(left as u16, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);

    (left, right)
}

pub fn monochromatic(hsl: &Hsl) -> Result<Vec<Color>, PaletteError> {
    let mut monochrome: Vec<Color> = Vec::new();

    for l in (50..hsl.l).step_by(50) {
        monochrome.push(Color::new(Hsl::new(hsl.h, hsl.s, l).encode())?);
    }

    for l in ((hsl.l + 50)..=1000).step_by(50) {
        monochrome.push(Color::new(Hsl::new(hsl.h, hsl.s, l).encode())?);
    }

    Ok(monochrome)
}

pub fn gradient(hsl: Hsl, hsl2: Hsl, num: u32) -> Vec<Hsl> {
    assert!(hsl.h <= 360 && hsl2.h <= 360);
    assert!(hsl.s <= 1000 && hsl2.s <= 1000);
    assert!(hsl.l <= 1000 && hsl2.l <= 1000);
    assert!(num > 1);

    let num = num as i32;
    let mut gradient: Vec<Hsl> = Vec::with_capacity(num as usize);

    let hue = if hsl.s == 0 {
        hsl2.h as i32
    } else {
        hsl.h as i32
    };

    let mut h = hue;
    let mut s = hsl.s as i32;
    let mut l = hsl.l as i32;

    let end_h = hsl2.h as i32;
    let s_interval = (hsl2.s as i32 - s) / num;
    let l_interval = (hsl2.l as i32 - l) / num;
    let h_difference = end_h - hue;

    let h_interval = if h_difference > 180 {
        (360 - h_difference) / num
    } else if h_difference < -180 {
        (360 + h_difference) / num
    } else {
        h_difference / num
    };

    for _ in 0..num {
        h = (h + h_interval).rem_euclid(360);
        s = s + s_interval;
        l = l + l_interval;

        gradient.push(Hsl::new(h as u16, s as u16, l as u16));
    }

    gradient
}

pub fn three_node_distance_rgb(rgb1: Rgb, rgb2: Rgb) -> u32 {
    let r = (rgb1.r as i32 - rgb2.r as i32).pow(2);

    let g = (rgb1.g as i32 - rgb2.g as i32).pow(2);

    let b = (rgb1.b as i32 - rgb2.b as i32).pow(2);

    let distance = r + g + b;
    distance as u32
}

pub fn n_color_average_complement(nodes: &Vec<Color>) -> Result<Color, PaletteError> {
    let mut complements: Vec<Hsl> = Vec::new();
    for node in nodes {
        complements.push(complement(&node.hsl));
    }
    let rgb = match complements.pop() {
        Some(val) => val.encode().get_rgb()?,
        None => {
            return Err(PaletteError::InvalidFormat(
                "no colors to complement".to_string(),
            ));
        }
    };
    let mut r = rgb.r as u32;
    let mut g = rgb.g as u32;
    let mut b = rgb.b as u32;
    for complement in complements {
        let rgb = complement.encode().get_rgb()?;
        r = (r + rgb.r as u32) / 2;
        g = (r + rgb.g as u32) / 2;
        b = (r + rgb.b as u32) / 2;
    }

    Ok(Color::new(Rgb::new(r as u8, g as u8, b as u8).encode())?)
}

pub fn generate_color() -> Result<Color, PaletteError> {
    let mut rng = rand::rng();
    let h = rng.random_range(0..361);
    let s = rng.random_range(0..1001);
    let l = rng.random_range(0..1001);
    Ok(Color::new(Hsl::new(h, s, l).encode())?)
}

pub fn generate_palette(num: usize) -> Result<Vec<Color>, PaletteError> {
    assert!(num > 0);
    let mut new_palette = Vec::with_capacity(num as usize);
    new_palette.push(generate_color()?);
    let mut i = 1;
    if i <= num {
        new_palette.push(Color::new(complement(&new_palette[0].hsl).encode())?);
        i += 1;
    }
    let mut rng = rand::rng();
    while i <= num {
        let method = if num - i < 2 {
            rng.random_range(0..3)
        } else if num - i < 3 {
            rng.random_range(0..4)
        } else {
            rng.random_range(0..5)
        };
        let index = rng.random_range(0..new_palette.len());
        match method {
            0 => new_palette.push(Color::new(complement(&new_palette[index].hsl).encode())?),
            1 => new_palette.push(generate_color()?),
            2 => {
                let new_color = n_color_average_complement(&new_palette)?;
                new_palette.push(new_color);
            }
            3 => {
                let (hsl1, hsl2) = triad(&new_palette[index].hsl);
                new_palette.push(Color::new(hsl1.encode())?);
                new_palette.push(Color::new(hsl2.encode())?);
                i += 1;
            }
            4 => {
                let (hsl1, hsl2, hsl3) = square(&new_palette[index].hsl);
                new_palette.push(Color::new(hsl1.encode())?);
                new_palette.push(Color::new(hsl2.encode())?);
                new_palette.push(Color::new(hsl3.encode())?);
                i += 2;
            }
            _ => {
                return Err(PaletteError::UntranslatableEncoding(
                    "generate palette from base generated an invalid number".to_string(),
                ));
            }
        }
        i += 1;
    }
    Ok(new_palette)
}

pub fn generate_palette_from_base(
    current_palette: &Vec<Color>,
    num: usize,
) -> Result<Vec<Color>, PaletteError> {
    let mut rng = rand::rng();
    let mut temp_palette = Vec::with_capacity(num as usize);
    let mut i = 0;
    while i < num {
        let method = if num - i < 2 {
            rng.random_range(0..3)
        } else if num - i < 3 {
            rng.random_range(0..4)
        } else {
            rng.random_range(0..5)
        };
        let index = rng.random_range(0..current_palette.len());
        match method {
            0 => temp_palette.push(Color::new(
                complement(&current_palette[index].hsl).encode(),
            )?),
            1 => temp_palette.push(generate_color()?),
            2 => {
                let new_color = n_color_average_complement(current_palette)?;
                temp_palette.push(new_color);
            }
            3 => {
                let (hsl1, hsl2) = triad(&current_palette[index].hsl);
                temp_palette.push(Color::new(hsl1.encode())?);
                temp_palette.push(Color::new(hsl2.encode())?);
                i += 1;
            }
            4 => {
                let (hsl1, hsl2, hsl3) = square(&current_palette[index].hsl);
                temp_palette.push(Color::new(hsl1.encode())?);
                temp_palette.push(Color::new(hsl2.encode())?);
                temp_palette.push(Color::new(hsl3.encode())?);
                i += 2;
            }
            _ => {
                return Err(PaletteError::InvalidFormat(
                    "generate palette from base generated an invalid random number".to_string(),
                ));
            }
        }
        i += 1;
    }

    Ok(temp_palette)
}
