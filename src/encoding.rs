use crate::color_math::three_node_distance;
use crate::color_spaces::*;
use crate::named_colors::NAMED_COLORS;
use std::cmp::{max, min};

// TODO

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

// -----------------------

impl Encoding {
    fn translate_to_rgb(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => Encoding::Rgb(*r, *g, *b),

            // -----------------------
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

            // -----------------------
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => panic!("failed to get rgb from {name}"),
                };
                Encoding::Rgb(r, g, b)
            }

            // -----------------------
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

            // -----------------------
            Encoding::Hex(h) => {
                let r = ((h >> 16) & 0xFF) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;
                Encoding::Rgb(r, g, b)
            }
        }
    }

    // -----------------------

    fn translate_to_hsl(&self) -> Encoding {
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
            Encoding::Hsl(h, s, l) => Encoding::Hsl(*h, *s, *l),
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => panic!("failed to get rgb from {name}"),
                };
                Encoding::Rgb(r, g, b).translate_to_hsl()
            }
            Encoding::Hsb(h, s, b) => {
                assert!(*h <= 360);
                assert!(*s <= 1000);
                assert!(*b <= 1000);
                let h = *h;
                let s = *s;
                let b = *b;

                let l = b * (1000 - (s / 2));
                let s = if l == 0 || l == 1 {
                    0
                } else {
                    (b - l) / (min(l, 1000 - l))
                };

                Encoding::Hsl(h, s, l)
            }
            Encoding::Hex(h) => {
                let r = ((h >> 16) & 0xFF) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;

                let r = ((r as f32 / 255.0) * 1000.0).round();
                let g = ((g as f32 / 255.0) * 1000.0).round();
                let b = ((b as f32 / 255.0) * 1000.0).round();
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
        }
    }

    // -----------------------

    fn translate_to_hsb(&self) -> Encoding {
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
            Encoding::Hsl(h, s, l) => {
                assert!(*h <= 360);
                assert!(*s <= 1000);
                assert!(*l <= 1000);
                let h = *h;
                let old_s = *s;
                let b = *l + old_s * min(*l, 1000 - *l);
                let s = if b == 0 { 0 } else { 2 * (1000 - *l / b) };
                Encoding::Hsb(h, s, b)
            }
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => panic!("failed to get rgb from {name}"),
                };
                Encoding::Rgb(r, g, b).translate_to_hsb()
            }
            Encoding::Hsb(h, s, b) => Encoding::Hsb(*h, *s, *b),
            Encoding::Hex(hex) => {
                let r = ((hex >> 16) & 0xFF) as u8;
                let g = ((hex >> 8) & 0xFF) as u8;
                let b = (hex & 0xFF) as u8;

                let red = ((r as f32 / 255.0) * 1000.0).round();
                let green = ((g as f32 / 255.0) * 1000.0).round();
                let blue = ((b as f32 / 255.0) * 1000.0).round();

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
        }
    }

    // -----------------------

    fn rgb_to_hex(&self) -> Encoding {
        match self {
            Encoding::Rgb(r, g, b) => {
                Encoding::Hex((*r as u32) << 16 | (*g as u32) << 8 | (*b as u32))
            }
            _ => panic!("wrong encoding type"),
        }
    }

    // -----------------------

    fn translate_to_name(&self) -> Encoding {
        match self {
            Encoding::Name(name) => Encoding::Name(name.to_string()),
            _ => {
                let rgb = self.get_rgb();
                let mut name: Encoding = Encoding::Rgb(0, 0, 0);
                let mut min_distance = 999999999;
                for (key, (r, g, b)) in NAMED_COLORS.iter() {
                    let start = Rgb::new(*r, *g, *b);
                    let goal = Rgb::new(rgb.r, rgb.g, rgb.b);
                    if start == goal {
                        name = Encoding::Name(String::from(*key))
                    } else {
                        let new_distance = three_node_distance(goal, start);
                        if new_distance < min_distance {
                            min_distance = new_distance;
                            name = Encoding::Name(String::from(*key));
                        }
                    }
                }
                name
            }
        }
    }

    // -----------------------

    pub fn get_rgb(&self) -> Rgb {
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

    // -----------------------

    pub fn get_hsl(&self) -> Hsl {
        match self {
            Encoding::Hsl(h, s, l) => Hsl::new(*h, *s, *l),
            _ => {
                let hsl = self.translate_to_hsl();
                match hsl {
                    Encoding::Hsl(h, s, l) => Hsl::new(h, s, l),
                    _ => panic!("could not translate to hsl"),
                }
            }
        }
    }

    // -----------------------

    pub fn get_hsb(&self) -> Hsb {
        match self {
            Encoding::Hsb(h, s, b) => Hsb::new(*h, *s, *b),
            _ => {
                let hsb = self.translate_to_hsb();
                match hsb {
                    Encoding::Hsl(h, s, b) => Hsb::new(h, s, b),
                    _ => panic!("could not translate to hsb"),
                }
            }
        }
    }

    // -----------------------

    pub fn get_hex(&self) -> Hex {
        match self {
            Encoding::Hex(h) => Hex::new(*h),
            Encoding::Rgb(_, _, _) => {
                let hex = self.rgb_to_hex();
                match hex {
                    Encoding::Hex(h) => Hex::new(h),
                    _ => panic!("could not translate rgb to hex"),
                }
            }
            _ => {
                let rgb = self.translate_to_rgb();
                match rgb {
                    Encoding::Rgb(_, _, _) => {
                        let hex = self.rgb_to_hex();
                        match hex {
                            Encoding::Hex(h) => Hex::new(h),
                            _ => panic!("could not translate rgb to hex"),
                        }
                    }
                    _ => panic!("could not translate to rgb"),
                }
            }
        }
    }

    // -----------------------

    pub fn get_name(&self) -> String {
        match self {
            Encoding::Name(n) => String::from(n),
            _ => {
                let name = &self.translate_to_name();
                match name {
                    Encoding::Name(n) => String::from(n),
                    _ => panic!("could not translate to name"),
                }
            }
        }
    }
}

// -----------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    // use crate::color_spaces::*;

    #[test]
    fn test_get_name() {
        let tests: HashMap<Encoding, String> = HashMap::from([
            (Encoding::Rgb(205, 92, 92), String::from("Indian Red")),
            (Encoding::Rgb(205, 91, 93), String::from("Indian Red")),
            (Encoding::Hsl(0, 53, 58), String::from("Indian Red")),
        ]);

        for (encoding, name) in tests {
            println!("input: {:?}, desired result: {:?}", encoding, name);
            assert_eq!(encoding.get_name(), name);
        }
    }
}
