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
        match (self, other) {
            (Encoding::Rgb(r1, g1, b1), Encoding::Rgb(r2, g2, b2)) => {
                r1 == r2 && g1 == g2 && b1 == b2
            }
            (Encoding::Hsl(h1, s1, l1), Encoding::Hsl(h2, s2, l2)) => {
                h1 == h2 && s1 == s2 && l1 == l2
            }
            (Encoding::Name(n1), Encoding::Name(n2)) => n1 == n2,
            (Encoding::Hsb(h1, s1, b1), Encoding::Hsb(h2, s2, b2)) => {
                h1 == h2 && s1 == s2 && b1 == b2
            }
            (Encoding::Hex(h1), Encoding::Hex(h2)) => h1 == h2,
            _ => false,
        }
    }
}

impl Encoding {
    fn translate_to_rgb(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => Encoding::Rgb(*r, *g, *b),
            Encoding::Hsl(h, s, l) => {
                if *h < 0.0 || *h > 360.0 {
                    panic!("h not a degree");
                }
                if *s < 0.0 || *s > 1.0 {
                    panic!("s not a percentage")
                }
                if *l < 0.0 || *l > 1.0 {
                    panic!("l not a percentage")
                }
                let (r, g, b): (f32, f32, f32);
                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                println!("{c}");
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                println!("{x}");
                let m = l - c / 2.0;
                println!("{m}");

                if *h < 60.0 {
                    (r, g, b) = (c, x, 0.0);
                } else if *h < 120.0 {
                    (r, g, b) = (x, c, 0.0);
                } else if *h < 180.0 {
                    (r, g, b) = (0.0, c, x);
                } else if *h < 240.0 {
                    (r, g, b) = (0.0, x, c);
                } else if *h < 300.0 {
                    (r, g, b) = (x, 0.0, c);
                } else if *h < 360.0 {
                    (r, g, b) = (c, 0.0, x);
                } else {
                    panic!("h out of bounds");
                }
                let (r, g, b) = (
                    ((r + m) * 255.0) as i32,
                    ((g + m) * 255.0) as i32,
                    ((b + m) * 255.0) as i32,
                );
                println!("r: {r}");
                println!("g: {g}");
                println!("b: {b}");
                Encoding::Rgb(r, g, b)
            }
            Encoding::Name(_) => Encoding::Rgb(0, 0, 0),
            Encoding::Hsb(h, s, b) => {
                if *h < 0.0 || *h > 360.0 {
                    panic!("h not a degree");
                }
                if *s < 0.0 || *s > 1.0 {
                    panic!("s not a percentage")
                }
                if *b < 0.0 || *b > 1.0 {
                    panic!("b not a percentage")
                }
                let (red, green, blue): (f32, f32, f32);
                let c = b * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = b - c;

                if *h < 60.0 {
                    (red, green, blue) = (c, x, 0.0);
                } else if *h < 120.0 {
                    (red, green, blue) = (x, c, 0.0);
                } else if *h < 180.0 {
                    (red, green, blue) = (0.0, c, x);
                } else if *h < 240.0 {
                    (red, green, blue) = (0.0, x, c);
                } else if *h < 300.0 {
                    (red, green, blue) = (x, 0.0, c);
                } else if *h < 360.0 {
                    (red, green, blue) = (c, 0.0, x);
                } else {
                    panic!("h out of bounds");
                }
                let (red, green, blue) = (
                    ((red + m) * 255.0) as i32,
                    ((green + m) * 255.0) as i32,
                    ((blue + m) * 255.0) as i32,
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
        match self {
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

    #[test]
    fn rgb_to_rgb() {
        let test = Encoding::Rgb(245, 73, 39);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hex_to_rgb() {
        let test = Encoding::Hex(0xf54927);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsl_to_rgb() {
        let test = Encoding::Hsl(10.0, 0.91, 0.56);
        println!("h: 10.0, s: 0.91, l: 0.56");
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsb_to_rgb() {
        let test = Encoding::Hsb(10.0, 0.841, 0.961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }
}
