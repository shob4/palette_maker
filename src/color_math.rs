use std::cmp::{max, min};

// TODO
// [x] rewrite rgb_to_hsl() for new data types
// [x] write test for rgb_to_hsl()
// [x] write tests for each region of hsl and hsb
// [x] change hex for new data types?
// [] write test for rgb to hex

#[derive(Debug)]
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
    fn rgb_to_hex() {
        let test = Encoding::Rgb(245, 73, 39);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsl_to_rgb() {
        let test = Encoding::Hsl(10, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsl_to_rgb2() {
        let test = Encoding::Hsl(70, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(211, 245, 39));
    }

    #[test]
    fn hsl_to_rgb3() {
        let test = Encoding::Hsl(130, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(39, 245, 73));
    }

    #[test]
    fn hsl_to_rgb4() {
        let test = Encoding::Hsl(190, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(39, 211, 245));
    }

    #[test]
    fn hsl_to_rgb5() {
        let test = Encoding::Hsl(250, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(73, 39, 245));
    }

    #[test]
    fn hsl_to_rgb6() {
        let test = Encoding::Hsl(310, 912, 557);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 39, 211));
    }

    #[test]
    #[should_panic]
    fn hsl_h_too_big() {
        Encoding::Hsl(370, 912, 557).translate_to_rgb();
    }

    #[test]
    #[should_panic]
    fn hsl_s_too_big() {
        Encoding::Hsl(10, 1001, 557).translate_to_rgb();
    }

    #[test]
    #[should_panic]
    fn hsl_l_too_big() {
        Encoding::Hsl(10, 912, 1001).translate_to_rgb();
    }

    #[test]
    fn rgb_to_hsl() {
        let test = Encoding::Rgb(245, 73, 39);
        let result = test.rgb_to_hsl();
        assert_eq!(result, Encoding::Hsl(10, 912, 557));
    }

    #[test]
    fn hsb_to_rgb() {
        let test = Encoding::Hsb(10, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 73, 39));
    }

    #[test]
    fn hsb_to_rgb2() {
        let test = Encoding::Hsb(70, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(211, 245, 39));
    }

    #[test]
    fn hsb_to_rgb3() {
        let test = Encoding::Hsb(130, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(39, 245, 73));
    }

    #[test]
    fn hsb_to_rgb4() {
        let test = Encoding::Hsb(190, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(39, 211, 245));
    }

    #[test]
    fn hsb_to_rgb5() {
        let test = Encoding::Hsb(250, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(73, 39, 245));
    }

    #[test]
    fn hsb_to_rgb6() {
        let test = Encoding::Hsb(310, 841, 961);
        let result = test.translate_to_rgb();
        assert_eq!(result, Encoding::Rgb(245, 39, 211));
    }

    #[test]
    #[should_panic]
    fn hsb_h_too_big() {
        Encoding::Hsb(376, 841, 961).translate_to_rgb();
    }

    #[test]
    #[should_panic]
    fn hsb_s_too_big() {
        Encoding::Hsb(10, 1001, 961).translate_to_rgb();
    }

    #[test]
    #[should_panic]
    fn hsb_b_too_big() {
        Encoding::Hsb(10, 841, 1001).translate_to_rgb();
    }

    #[test]
    fn rgb_to_hsb() {
        let test = Encoding::Rgb(245, 73, 39);
        let result = test.rgb_to_hsb();
        assert_eq!(result, Encoding::Hsb(10, 841, 961));
    }
}
