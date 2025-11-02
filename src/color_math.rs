use std::cmp::{max, min};

#[derive(Debug)]
pub enum Encoding {
    Rgb(u8, u8, u8),
    Hsl(u16, u16, u16),
    Name(String),
    Hsb(u16, u16, u16),
    Hex(i32),
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
                assert!(*h < 360);
                assert!(*s < 1000);
                assert!(*l < 1000);

                let region = *h / 60;
                let h = *h as f32;
                let s = *s as f32 / 1000.0;
                let l = *l as f32 / 1000.0;

                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let (r, g, b) = match region {
                    0 => (c, x, 0.0),
                    1 => (x, c, 0.0),
                    2 => (0.0, c, x),
                    3 => (0.0, x, c),
                    4 => (x, 0.0, c),
                    _ => (c, 0.0, x),
                };
                let (r, g, b) = (
                    ((r + m) * 255.0).round() as u8,
                    ((g + m) * 255.0).round() as u8,
                    ((b + m) * 255.0).round() as u8,
                );
                Encoding::Rgb(r, g, b)
            }
            Encoding::Name(_) => Encoding::Rgb(0, 0, 0),
            Encoding::Hsb(h, s, b) => {
                assert!(*h <= 360);
                assert!(*s <= 1000);
                assert!(*b <= 1000);

                let region = h / 60;
                let h = *h as f32;
                let s = *s as f32 / 1000.0;
                let b = *b as f32 / 1000.0;

                let c = b * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = b - c;

                let (r, g, b) = match region {
                    0 => (c, x, 0.0),
                    1 => (x, c, 0.0),
                    2 => (0.0, c, x),
                    3 => (0.0, x, c),
                    4 => (x, 0.0, c),
                    _ => (c, 0.0, x),
                };

                let (r, g, b) = (
                    ((r + m) * 255.0).round() as u8,
                    ((g + m) * 255.0).round() as u8,
                    ((b + m) * 255.0).round() as u8,
                );
                Encoding::Rgb(r, g, b)
            }
            Encoding::Hex(h) => {
                let r = ((h >> 16) & 0xFF) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;
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

fn int_hsb_to_rgb(h: u16, s: u16, b: u16) -> (u8, u8, u8) {
    assert!(h <= 360);
    assert!(s <= 1000);
    assert!(b <= 1000);

    let region = h / 60;
    let h = h as f32;
    let s = s as f32 / 1000.0;
    let b = b as f32 / 1000.0;

    let c = b * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = b - c;

    let (r, g, b) = match region {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let (r, g, b) = (
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    );
    (r, g, b)
}

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
        let test = Encoding::Hsl(10, 910, 560);
        println!("h: 10.0, s: 0.91, l: 0.56");
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsb_to_rgb() {
        let test = Encoding::Hsb(10, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn test_int_hsb_to_rgb() {
        assert_eq!(int_hsb_to_rgb(10, 841, 961), (245, 73, 39));
    }
}
