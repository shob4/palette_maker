use crate::color_spaces::*;
use crate::named_colors::NAMED_COLORS;
use crate::{color_math::three_node_distance_rgb, error::PaletteError};
use std::cmp::{max, min};

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
    fn translate_to_rgb(&self) -> Result<Encoding, PaletteError> {
        match self {
            Encoding::Rgb(r, g, b) => Ok(Encoding::Rgb(*r, *g, *b)),

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
                Ok(Encoding::Rgb(r, g, b))
            }

            // -----------------------
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "failed to get rgb from {name}".to_string(),
                        ));
                    }
                };
                Ok(Encoding::Rgb(r, g, b))
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
                Ok(Encoding::Rgb(r, g, b))
            }

            // -----------------------
            Encoding::Hex(h) => {
                let r = ((h >> 16) & 0xFF) as u8;
                let g = ((h >> 8) & 0xFF) as u8;
                let b = (h & 0xFF) as u8;
                Ok(Encoding::Rgb(r, g, b))
            }
        }
    }

    // -----------------------

    fn translate_to_hsl(&self) -> Result<Encoding, PaletteError> {
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
                    return Err(PaletteError::UntranslatableEncoding(format!(
                        "c_max ({c_max}) does not match r ({r}), g ({g}), or b ({b})"
                    )));
                };

                let s = if delta == 0.0 {
                    0.0
                } else {
                    delta / (1.0 - (2.0 * (l / 1000.0) - 1.0).abs())
                };

                Ok(Encoding::Hsl(
                    h.round() as u16,
                    s.round() as u16,
                    l.round() as u16,
                ))
            }
            Encoding::Hsl(h, s, l) => Ok(Encoding::Hsl(*h, *s, *l)),
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "failed to get rgb from {name}".to_string(),
                        ));
                    }
                };
                match Encoding::Rgb(r, g, b).translate_to_hsl() {
                    Ok(hsl) => Ok(hsl),
                    Err(e) => Err(e),
                }
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

                Ok(Encoding::Hsl(h, s, l))
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
                    return Err(PaletteError::UntranslatableEncoding(
                        "c_max ({c_max}) does not match r ({r}), g ({g}), or b ({b})".to_string(),
                    ));
                };

                let s = if delta == 0.0 {
                    0.0
                } else {
                    delta / (1.0 - (2.0 * (l / 1000.0) - 1.0).abs())
                };

                Ok(Encoding::Hsl(
                    h.round() as u16,
                    s.round() as u16,
                    l.round() as u16,
                ))
            }
        }
    }

    // -----------------------

    fn translate_to_hsb(&self) -> Result<Encoding, PaletteError> {
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

                Ok(Encoding::Hsb(
                    h.round() as u16,
                    s.round() as u16,
                    b.round() as u16,
                ))
            }
            Encoding::Hsl(h, s, l) => {
                assert!(*h <= 360);
                assert!(*s <= 1000);
                assert!(*l <= 1000);
                let h = *h;
                let old_s = *s;
                // TODO check multiplication
                let b = (*l as u32 + old_s as u32 * min(*l, 1000 - *l) as u32) as u16;
                let s = if b == 0 { 0 } else { 2 * (1000 - *l / b) };
                Ok(Encoding::Hsb(h, s, b))
            }
            Encoding::Name(name) => {
                let (r, g, b) = match NAMED_COLORS.get(name.as_str()) {
                    Some(rgb) => *rgb,
                    None => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "failed to get rgb from {name}".to_string(),
                        ));
                    }
                };
                Encoding::Rgb(r, g, b).translate_to_hsb()
            }
            Encoding::Hsb(h, s, b) => Ok(Encoding::Hsb(*h, *s, *b)),
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

                Ok(Encoding::Hsb(
                    h.round() as u16,
                    s.round() as u16,
                    b.round() as u16,
                ))
            }
        }
    }

    // -----------------------

    fn rgb_to_hex(&self) -> Result<Encoding, PaletteError> {
        match self {
            Encoding::Rgb(r, g, b) => Ok(Encoding::Hex(
                (*r as u32) << 16 | (*g as u32) << 8 | (*b as u32),
            )),
            _ => {
                return Err(PaletteError::UntranslatableEncoding(format!(
                    "rgb to hex was given the wrong encoding type: {:?}",
                    self
                )));
            }
        }
    }

    // -----------------------

    fn translate_to_name(&self) -> Result<Encoding, PaletteError> {
        match self {
            Encoding::Name(name) => Ok(Encoding::Name(name.to_string())),
            _ => {
                let rgb = self.get_rgb()?;
                let mut name: Encoding = Encoding::Rgb(0, 0, 0);
                let mut min_distance = u32::MAX;
                for (key, (r, g, b)) in NAMED_COLORS.iter() {
                    let start = Rgb::new(*r, *g, *b);
                    let goal = Rgb::new(rgb.r, rgb.g, rgb.b);
                    if start == goal {
                        name = Encoding::Name(String::from(*key));
                        break;
                    } else {
                        let new_distance = three_node_distance_rgb(goal, start);
                        if new_distance < min_distance {
                            min_distance = new_distance;
                            name = Encoding::Name(String::from(*key));
                        }
                    }
                }
                Ok(name)
            }
        }
    }

    // -----------------------

    pub fn get_rgb(&self) -> Result<Rgb, PaletteError> {
        match self {
            Encoding::Rgb(r, g, b) => Ok(Rgb::new(*r, *g, *b)),
            _ => {
                let rgb = self.translate_to_rgb()?;
                match rgb {
                    Encoding::Rgb(r, g, b) => Ok(Rgb::new(r, g, b)),
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "could not translate to rgb".to_string(),
                        ));
                    }
                }
            }
        }
    }

    // -----------------------

    pub fn get_hsl(&self) -> Result<Hsl, PaletteError> {
        match self {
            Encoding::Hsl(h, s, l) => Ok(Hsl::new(*h, *s, *l)),
            _ => {
                let hsl = self.translate_to_hsl()?;
                match hsl {
                    Encoding::Hsl(h, s, l) => Ok(Hsl::new(h, s, l)),
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "could not translate to hsl".to_string(),
                        ));
                    }
                }
            }
        }
    }

    // -----------------------

    pub fn get_hsb(&self) -> Result<Hsb, PaletteError> {
        match self {
            Encoding::Hsb(h, s, b) => Ok(Hsb::new(*h, *s, *b)),
            _ => {
                let hsb = self.translate_to_hsb()?;
                match hsb {
                    Encoding::Hsb(h, s, b) => Ok(Hsb::new(h, s, b)),
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "could not translate to hsb".to_string(),
                        ));
                    }
                }
            }
        }
    }

    // -----------------------

    pub fn get_hex(&self) -> Result<Hex, PaletteError> {
        match self {
            Encoding::Hex(h) => Ok(Hex::new(*h)),
            Encoding::Rgb(_, _, _) => {
                let hex = self.rgb_to_hex()?;
                match hex {
                    Encoding::Hex(h) => Ok(Hex::new(h)),
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "1 could not translate rgb to hex".to_string(),
                        ));
                    }
                }
            }
            _ => {
                let rgb = self.translate_to_rgb()?;
                match rgb {
                    Encoding::Rgb(_, _, _) => {
                        let hex = rgb.rgb_to_hex()?;
                        match hex {
                            Encoding::Hex(h) => Ok(Hex::new(h)),
                            _ => {
                                return Err(PaletteError::UntranslatableEncoding(
                                    "2 could not translate rgb to hex".to_string(),
                                ));
                            }
                        }
                    }
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "could not translate to rgb while trying to get hex".to_string(),
                        ));
                    }
                }
            }
        }
    }

    // -----------------------

    pub fn get_name(&self) -> Result<String, PaletteError> {
        match self {
            Encoding::Name(n) => Ok(String::from(n)),
            _ => {
                let name = &self.translate_to_name()?;
                match name {
                    Encoding::Name(n) => Ok(String::from(n)),
                    _ => {
                        return Err(PaletteError::UntranslatableEncoding(
                            "could not translate to name".to_string(),
                        ));
                    }
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
            // don't know how it translates, so can't test off by ones
            (Encoding::Hsl(0, 531, 582), String::from("Indian Red")),
            (Encoding::Hsb(0, 551, 804), String::from("Indian Red")),
        ]);

        for (encoding, name) in tests {
            println!("input: {:?}, desired result: {:?}", encoding, name);
            let result = match encoding.get_name() {
                Ok(name) => name,
                Err(e) => panic!("{e}"),
            };
            assert_eq!(result, name);
        }
    }

    #[test]
    fn test_get_rgb() {
        let tests: HashMap<Encoding, Rgb> = HashMap::from([
            (Encoding::Hsl(0, 531, 582), Rgb::new(205, 92, 92)),
            (
                Encoding::Name(String::from("Indian Red")),
                Rgb::new(205, 92, 92),
            ),
            (Encoding::Hsb(0, 551, 804), Rgb::new(205, 92, 92)),
        ]);

        for (encoding, rgb) in tests {
            println!("input: {:?}, desired result: {:?}", encoding, rgb);
            let result = match encoding.get_rgb() {
                Ok(rgb) => rgb,
                Err(e) => panic!("{e}"),
            };
            assert_eq!(result, rgb);
        }
    }
}
