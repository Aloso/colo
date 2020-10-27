mod convert;

pub mod hex;
pub mod html;
pub mod json;

/// Module containing all color spaces
pub mod spaces {
    pub use color_space::{Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy};
}

use spaces::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorSpace {
    Rgb,
    Cmy,
    Cmyk,
    Hsv,
    Hsl,
    Lch,
    Luv,
    Lab,
    HunterLab,
    Xyz,
    Yxy,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Rgb(Rgb),
    Cmy(Cmy),
    Cmyk(Cmyk),
    Hsv(Hsv),
    Hsl(Hsl),
    Lch(Lch),
    Luv(Luv),
    Lab(Lab),
    HunterLab(HunterLab),
    Xyz(Xyz),
    Yxy(Yxy),
}

impl Color {
    pub fn new(color_space: ColorSpace, components: &[f64]) -> Result<Self, convert::ParseError> {
        std::convert::TryFrom::try_from((color_space, components))
    }
}
