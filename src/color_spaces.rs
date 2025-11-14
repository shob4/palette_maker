use crate::encoding::Encoding;

#[derive(Hash, Eq, Debug)]
pub struct Hsl {
    pub h: u16,
    pub s: u16,
    pub l: u16,
}

impl PartialEq for Hsl {
    fn eq(&self, other: &Hsl) -> bool {
        self.h == other.h && self.s == other.s && self.l == other.l
    }
}

impl Hsl {
    pub fn encode(&self) -> Encoding {
        Encoding::Hsl(self.h, self.s, self.l)
    }

    pub fn new(h: u16, s: u16, l: u16) -> Hsl {
        Hsl { h: h, s: s, l: l }
    }
}

// -----------------------

#[derive(Hash, Eq, Debug)]
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
    pub fn encode(&self) -> Encoding {
        Encoding::Rgb(self.r, self.g, self.b)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r: r, g: g, b: b }
    }
}
