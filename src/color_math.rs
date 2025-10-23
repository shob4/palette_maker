pub enum Encoding {
    Rgb(i32, i32, i32),
    Hsl(f32, f32, f32),
    Name(String),
    Hsb(i32, i32, i32),
    Hex(i128),
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
            Encoding::Hsb(r, g, b) => Encoding::Rgb(r, g, b),
            Encoding::Hex(h) => {
                let r = (h % 0x00ffff / 0xffff) as i32;
                let g = (h % 0xff0000 / 0xff) as i32;
                let b = (h % 0xffff00) as i32;
                Encoding::Rgb(r, g, b)
            }
        }
    }
}

pub fn complement() {}
pub fn triad() {}
pub fn square() {}
pub fn analogous() {}
pub fn monochromatic() {}
