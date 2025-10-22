pub enum Encoding {
    Rgb(u32, u32, u32),
    Hsl(u32, u32, u32),
    Name(String),
    Hsb(u32, u32, u32),
    Hex(u128),
}

impl Encoding {
    fn translate_to_rgb(&self) -> Encoding {
        match *self {
            Encoding::Rgb(r, g, b) => Encoding::Rgb(r, g, b),
            Encoding::Hsl(r, g, b) => Encoding::Rgb(r, g, b),
            Encoding::Name(_) => Encoding::Rgb(0, 0, 0),
            Encoding::Hsb(r, g, b) => Encoding::Rgb(r, g, b),
            Encoding::Hex(_) => Encoding::Rgb(0, 0, 0),
        }
    }
}

pub fn complement() {}
pub fn triad() {}
pub fn square() {}
pub fn analogous() {}
pub fn monochromatic() {}
