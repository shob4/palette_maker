use std::cmp::{max, min};
use std::collections::HashMap;

// TODO
// [x] rewrite rgb_to_hsl() for new data types
// [x] write test for rgb_to_hsl()
// [x] write tests for each region of hsl and hsb
// [x] change hex for new data types?
// [x] write test for rgb to hex
// [x] write values method for Encoding?
// [x] figure out collision between encoding and rgb
// [x] add hsl struct and translations?
// [x] add tests for struct translation
// [x] change tests to loops?
// [] add more cases to tests

#[derive(Hash, Eq, Debug)]
pub enum Encoding {
    Rgb(u8, u8, u8),
    Hsl(u16, u16, u16),
    Name(String),
    Hsb(u16, u16, u16),
    Hex(u32),
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
                assert!(*h <= 360);
                assert!(*s <= 1000);
                assert!(*l <= 1000);

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

    fn rgb_to_hsl(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => {
                let r = ((*r as f32 / 255.0) * 1000.0).round();
                let g = ((*g as f32 / 255.0) * 1000.0).round();
                let b = ((*b as f32 / 255.0) * 1000.0).round();
                let bigger = max(r as i32, b as i32);
                let smaller = min(r as i32, b as i32);
                let c_max = max(bigger, g as i32) as f32;
                let c_min = min(smaller, g as i32) as f32;
                let delta = c_max - c_min;

                let l = (c_max + c_min) / 2.0;

                let h = if delta == 0.0 {
                    0.0
                } else if c_max == r {
                    60.0 * ((g - b) / delta % 6.0)
                } else if c_max == g {
                    60.0 * ((b - r) / delta + 2.0)
                } else if c_max == b {
                    60.0 * ((r - g) / delta + 4.0)
                } else {
                    panic!("c_max ({c_max}) does not match r ({r}), g ({g}), or b ({b})");
                };

                let s = if delta == 0.0 {
                    0.0
                } else {
                    delta / (1.0 - (2.0 * (l / 1000.0) - 1.0).abs())
                };

                Encoding::Hsl(h.round() as u16, s.round() as u16, l.round() as u16)
            }
            _ => panic!("wrong encoding type"),
        }
    }
    fn rgb_to_hsb(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => {
                let red = ((*r as f32 / 255.0) * 1000.0).round();
                let green = ((*g as f32 / 255.0) * 1000.0).round();
                let blue = ((*b as f32 / 255.0) * 1000.0).round();
                let bigger = max(red as i32, blue as i32);
                let smaller = min(red as i32, blue as i32);
                let c_max = max(bigger, green as i32) as f32;
                let c_min = min(smaller, green as i32) as f32;
                let delta = c_max - c_min;

                let h = if c_max == red {
                    60.0 * ((green - blue) / delta % 6.0)
                } else if c_max == green {
                    60.0 * ((blue - red) / delta + 2.0)
                } else if c_max == blue {
                    60.0 * ((red - green) / delta + 4.0)
                } else {
                    0.0
                };

                let s = if c_max == 0.0 {
                    0.0
                } else {
                    (delta / c_max) * 1000.0
                };
                let b = c_max;

                Encoding::Hsb(h.round() as u16, s.round() as u16, b.round() as u16)
            }
            _ => panic!("wrong encoding type"),
        }
    }
    fn rgb_to_hex(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => {
                Encoding::Hex((*r as u32) << 16 | (*g as u32) << 8 | (*b as u32))
            }
            _ => panic!("wrong encoding type"),
        }
    }
    fn rgb_to_name(&self) {}

    fn get_rgb(&self) -> Rgb {
        match self {
            Encoding::Rgb(r, g, b) => Rgb::new(*r, *g, *b),
            _ => {
                let rgb = self.translate_to_rgb();
                match rgb {
                    Encoding::Rgb(r, g, b) => Rgb::new(r, g, b),
                    _ => panic!("could not translate to rgb"),
                }
            }
        }
    }

    fn get_hsl(&self) -> Hsl {
        match self {
            Encoding::Hsl(h, s, l) => Hsl::new(*h, *s, *l),
            _ => {
                let rgb = self.translate_to_rgb();
                let hsl = rgb.rgb_to_hsl();
                match hsl {
                    Encoding::Hsl(h, s, l) => Hsl::new(h, s, l),
                    _ => panic!("could not translate to hsl"),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Hsl {
    h: u16,
    s: u16,
    l: u16,
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Hsl) -> bool {
        self.h == other.h && self.s == other.s && self.l == other.l
    }
}

impl Hsl {
    fn encode(&self) -> Encoding {
        Encoding::Hsl(self.h, self.s, self.l)
    }

    fn new(h: u16, s: u16, l: u16) -> Hsl {
        Hsl { h: h, s: s, l: l }
    }
}

#[derive(Debug)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Rgb) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Rgb {
    fn encode(&self) -> Encoding {
        Encoding::Rgb(self.r, self.g, self.b)
    }

    fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r: r, g: g, b: b }
    }
}

pub fn complement(rgb: Rgb) -> Rgb {
    let r = 255 - rgb.r;
    let g = 255 - rgb.g;
    let b = 255 - rgb.b;
    Rgb::new(r, g, b)
}

pub fn triad(rgb: Rgb) -> (Hsl, Hsl) {
    let rgb = rgb.encode();
    let hsl = rgb.get_hsl();
    let left = hsl.h - 60;
    let right = hsl.h + 60;
    let left = Hsl::new(left, hsl.s, hsl.l);
    let right = Hsl::new(right, hsl.s, hsl.l);
    (left, right)
}
pub fn square(rgb: Rgb) {}
pub fn analogous(rgb: Rgb) {}
pub fn monochromatic(rgb: Rgb) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rgb() {
        let encodings = [
            Encoding::Rgb(245, 73, 39),
            Encoding::Hsl(10, 912, 557),
            Encoding::Hsb(10, 841, 961),
            Encoding::Hex(0xf54927),
        ];
        let desired_result = Rgb::new(245, 73, 39);
        for encoding in encodings {
            println!("{:?}", encoding);
            let result = encoding.get_rgb();
            assert_eq!(result, desired_result);
        }
    }

    #[test]
    fn test_get_hsl() {
        let encodings = [
            Encoding::Rgb(245, 73, 39),
            Encoding::Hsl(10, 912, 557),
            Encoding::Hsb(10, 841, 961),
            Encoding::Hex(0xf54927),
        ];
        let desired_result = Hsl::new(10, 912, 557);
        for encoding in encodings {
            println!("{:?}", encoding);
            let result = encoding.get_hsl();
            assert_eq!(result, desired_result);
        }
    }

    #[test]
    fn test_translate_to_rgb() {
        let tests: HashMap<Encoding, Encoding> = HashMap::from([
            (Encoding::Rgb(245, 73, 39), Encoding::Rgb(245, 73, 39)),
            (Encoding::Hsl(10, 912, 557), Encoding::Rgb(245, 73, 39)),
            (Encoding::Hsb(10, 841, 961), Encoding::Rgb(245, 73, 39)),
            (Encoding::Hex(0xf54927), Encoding::Rgb(245, 73, 39)),
        ]);
        for (encoding, desired_result) in tests {
            println!(
                "encoding: {:?}, desired_result: {:?}",
                encoding, desired_result
            );
            let result = encoding.translate_to_rgb();
            assert_eq!(result, desired_result);
        }
    }
}
