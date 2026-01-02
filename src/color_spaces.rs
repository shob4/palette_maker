use crate::{encoding::Encoding, error::PaletteError};

// TODO

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
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

// -----------------------

#[derive(Hash, Eq, Debug)]
pub struct Hsb {
    pub h: u16,
    pub s: u16,
    pub b: u16,
}

impl PartialEq for Hsb {
    fn eq(&self, other: &Hsb) -> bool {
        self.h == other.h && self.s == other.s && self.b == other.b
    }
}

impl Hsb {
    pub fn encode(&self) -> Encoding {
        Encoding::Hsb(self.h, self.s, self.b)
    }

    pub fn new(h: u16, s: u16, b: u16) -> Hsb {
        Hsb { h: h, s: s, b: b }
    }
}

// -----------------------

#[derive(Hash, Eq, Debug)]
pub struct Hex {
    pub h: u32,
}

impl PartialEq for Hex {
    fn eq(&self, other: &Hex) -> bool {
        self.h == other.h
    }
}

impl Hex {
    pub fn encode(&self) -> Encoding {
        Encoding::Hex(self.h)
    }

    pub fn new(h: u32) -> Hex {
        Hex { h: h }
    }
}

// -----------------------

#[derive(Hash, Eq, Debug)]
pub struct Color {
    pub rgb: Rgb,
    pub hsl: Hsl,
    pub hsb: Hsb,
    pub hex: Hex,
    pub name: String,
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.rgb == other.rgb
            && self.hsl == other.hsl
            && self.hsb == other.hsb
            && self.hex == other.hex
            && self.name == other.name
    }
}

impl Color {
    pub fn new(code: Encoding) -> Result<Color, PaletteError> {
        let rgb = code.get_rgb()?;
        let hsl = code.get_hsl()?;
        let hsb = code.get_hsb()?;
        let hex = code.get_hex()?;
        let name = code.get_name()?;
        Ok(Color {
            rgb: rgb,
            hsl: hsl,
            hsb: hsb,
            hex: hex,
            name: name,
        })
    }

    pub fn hex_to_string(&self) -> String {
        format!("{}\n", self.hex.h)
    }

    pub fn rgb_to_string(&self) -> String {
        format!("{},{},{}\n", self.rgb.r, self.rgb.g, self.rgb.b)
    }

    pub fn hsl_to_string(&self) -> String {
        format!("{},{},{}\n", self.hsl.h, self.hsl.s, self.hsl.l)
    }

    pub fn hsb_to_string(&self) -> String {
        format!("{},{},{}\n", self.hsb.h, self.hsb.s, self.hsb.b)
    }

    pub fn ratatui_color(&self) -> ratatui::style::Color {
        ratatui::style::Color::Rgb(self.rgb.r, self.rgb.g, self.rgb.b)
    }

    pub fn ratatui_text(&self) -> ratatui::style::Color {
        if self.hsl.l >= 500 {
            ratatui::style::Color::Rgb(255, 255, 255)
        } else {
            ratatui::style::Color::Rgb(0, 0, 0)
        }
    }
}
