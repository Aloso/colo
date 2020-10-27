mod convert;
mod display;

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

    pub fn divide(self) -> (ColorSpace, Vec<f64>) {
        match self {
            Color::Rgb(color) => (ColorSpace::Rgb, vec![color.r, color.g, color.b]),
            Color::Cmy(color) => (ColorSpace::Cmy, vec![color.c, color.m, color.y]),
            Color::Cmyk(color) => (ColorSpace::Cmyk, vec![color.c, color.m, color.y, color.k]),
            Color::Hsv(color) => (ColorSpace::Hsv, vec![color.h, color.s, color.v]),
            Color::Hsl(color) => (ColorSpace::Hsl, vec![color.h, color.s, color.l]),
            Color::Lch(color) => (ColorSpace::Lch, vec![color.l, color.c, color.h]),
            Color::Luv(color) => (ColorSpace::Luv, vec![color.l, color.u, color.v]),
            Color::Lab(color) => (ColorSpace::Lab, vec![color.l, color.a, color.b]),
            Color::HunterLab(color) => (ColorSpace::HunterLab, vec![color.l, color.a, color.b]),
            Color::Xyz(color) => (ColorSpace::Xyz, vec![color.x, color.y, color.z]),
            Color::Yxy(color) => (ColorSpace::Yxy, vec![color.y1, color.x, color.y2]),
        }
    }
}
