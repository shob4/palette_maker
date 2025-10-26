use std::cmp::{max, min};

#[derive(Debug)]
pub enum Encoding {
    Rgb(i32, i32, i32),
    Hsl(f32, f32, f32),
    Name(String),
    Hsb(f32, f32, f32),
    Hex(i128),
}

impl PartialEq for Encoding {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Encoding::Rgb(r, g, b) => match *other {
                Encoding::Rgb(red, green, blue) => r == red && g == green && b == blue,
                _ => false,
            },
            Encoding::Hsl(h, s, l) => match *other {
                Encoding::Hsl(hue, saturation, light) => h == hue && s == saturation && l == light,
                _ => false,
            },
            Encoding::Name(n) => match *other {
                Encoding::Name(name) => n == name,
                _ => false,
            },
            Encoding::Hsb(h, s, b) => match *other {
                Encoding::Hsb(hue, saturation, brightness) => {
                    h == hue && s == saturation && b == brightness
                }
                _ => false,
            },
            Encoding::Hex(h) => match *other {
                Encoding::Hex(hex) => h == hex,
                _ => false,
            },
            _ => false,
        }
    }
}

impl Encoding {
    fn translate_to_rgb(&self) -> Encoding {
        match *self {
            Encoding::Rgb(r, g, b) => Encoding::Rgb(r, g, b),
            Encoding::Hsl(h, s, l) => {
                let (mut r, mut g, mut b): (i32, i32, i32);
                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let c = c as i32;
                let x = x as i32;
                if h < 60.0 {
                    (r, g, b) = (c, x, 0 as i32);
                } else if h < 120.0 {
                    (r, g, b) = (x, c, 0 as i32);
                } else if h < 180.0 {
                    (r, g, b) = (0 as i32, c, x);
                } else if h < 240.0 {
                    (r, g, b) = (0 as i32, x, c);
                } else if h < 300.0 {
                    (r, g, b) = (x, 0 as i32, c);
                } else if h < 360.0 {
                    (r, g, b) = (c, 0 as i32, x);
                } else {
                    panic!("h out of bounds");
                }
                (r, g, b) = (
                    (r + m as i32) * 255,
                    (g + m as i32) * 255,
                    (b + m as i32) * 255,
                );
                Encoding::Rgb(r, g, b)
            }
            Encoding::Name(_) => Encoding::Rgb(0, 0, 0),
            Encoding::Hsb(h, s, b) => {
                let (mut red, mut green, mut blue): (i32, i32, i32);
                let c = b * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = b - c;

                let c = c as i32;
                let x = x as i32;
                if h < 60.0 {
                    (red, green, blue) = (c, x, 0 as i32);
                } else if h < 120.0 {
                    (red, green, blue) = (x, c, 0 as i32);
                } else if h < 180.0 {
                    (red, green, blue) = (0 as i32, c, x);
                } else if h < 240.0 {
                    (red, green, blue) = (0 as i32, x, c);
                } else if h < 300.0 {
                    (red, green, blue) = (x, 0 as i32, c);
                } else if h < 360.0 {
                    (red, green, blue) = (c, 0 as i32, x);
                } else {
                    panic!("h out of bounds");
                }
                (red, green, blue) = (
                    (red + m as i32) * 255,
                    (green + m as i32) * 255,
                    (blue + m as i32) * 255,
                );
                Encoding::Rgb(red, green, blue)
            }
            Encoding::Hex(h) => {
                let r = (h % 0x00ffff / 0xffff) as i32;
                let g = (h % 0xff0000 / 0xff) as i32;
                let b = (h % 0xffff00) as i32;
                Encoding::Rgb(r, g, b)
            }
        }
    }

    fn rgb_to_hsl(&self) {}
    fn rgb_to_hsb(&self) -> Result<(f32, f32, f32), String> {
        match *self {
            Encoding::Rgb(r, g, b) => {
                let red = r / 255;
                let green = g / 255;
                let blue = b / 255;
                let bigger = max(red, blue);
                let smaller = min(red, blue);
                let c_max = max(bigger, green);
                let c_min = min(smaller, green);
                let delta = c_max as f32 - c_min as f32;
                let red = red as f32;
                let green = green as f32;
                let blue = blue as f32;
                let h = if (c_max as f32) == red {
                    60.0 * ((green - blue) / delta % 6.0)
                } else if (c_max as f32) == green {
                    60.0 * ((blue - red) / delta + 2.0)
                } else if (c_max as f32) == blue {
                    60.0 * ((red - green) / delta + 4.0)
                } else {
                    0.0
                };

                let s = if c_max == 0 {
                    0.0
                } else {
                    delta / (c_max as f32)
                };
                let b = c_max as f32;
                return Ok((h, s, b));
            }
            // TODO change to error type
            _ => return Err("Incorrect Encoding type".to_string()),
        }
    }
    fn rgb_to_hex(&self) {}
    fn rgb_to_name(&self) {}
}

pub fn complement() {}
pub fn triad() {}
pub fn square() {}
pub fn analogous() {}
pub fn monochromatic() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex_to_rgb() {
        let test = Encoding::Hex(0xf54927);
        let result = test.translate_to_rgb();
        assert_eq!(result, test);
    }
}
