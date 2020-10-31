mod convert;

pub mod hex;
pub mod html;
pub mod json;
pub mod space;

use color_space::{FromRgb, ToRgb};
pub use space::ColorSpace;

use space::*;
use std::fmt;

/// A color enum that unifies the color types specific to a color space.
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
    /// Constructs a color from the color space and the color components.
    pub fn new(color_space: ColorSpace, components: &[f64]) -> Result<Self, convert::ParseError> {
        std::convert::TryFrom::try_from((color_space, components))
    }

    /// Return the color space and the color components separately.
    pub fn divide(&self) -> (ColorSpace, Vec<f64>) {
        match *self {
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

    /// Converts the color to a different color space. It is in the same color
    /// space, this is a no-op.
    pub fn to_color_space(&self, color_space: ColorSpace) -> Self {
        let (current_space, _) = self.divide();
        if current_space == color_space {
            return *self;
        }
        let rgb = self.to_rgb();
        match color_space {
            ColorSpace::Rgb => Color::Rgb(rgb),
            ColorSpace::Cmy => Color::Cmy(Cmy::from_rgb(&rgb)),
            ColorSpace::Cmyk => Color::Cmyk(Cmyk::from_rgb(&rgb)),
            ColorSpace::Hsv => Color::Hsv(Hsv::from_rgb(&rgb)),
            ColorSpace::Hsl => Color::Hsl(Hsl::from_rgb(&rgb)),
            ColorSpace::Lch => Color::Lch(Lch::from_rgb(&rgb)),
            ColorSpace::Luv => Color::Luv(Luv::from_rgb(&rgb)),
            ColorSpace::Lab => Color::Lab(Lab::from_rgb(&rgb)),
            ColorSpace::HunterLab => Color::HunterLab(HunterLab::from_rgb(&rgb)),
            ColorSpace::Xyz => Color::Xyz(Xyz::from_rgb(&rgb)),
            ColorSpace::Yxy => Color::Yxy(Yxy::from_rgb(&rgb)),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (color_space, parts) = self.divide();
        write!(f, "{}(", color_space)?;
        let last = parts.len() - 1;
        for (i, part) in parts.into_iter().enumerate() {
            write!(f, "{}", (part * 100.0).round() / 100.0)?;
            if i != last {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

impl ToRgb for Color {
    fn to_rgb(&self) -> Rgb {
        match *self {
            Color::Rgb(color) => color,
            Color::Cmy(color) => color.to_rgb(),
            Color::Cmyk(color) => color.to_rgb(),
            Color::Hsv(color) => color.to_rgb(),
            Color::Hsl(color) => color.to_rgb(),
            Color::Lch(color) => color.to_rgb(),
            Color::Luv(color) => color.to_rgb(),
            Color::Lab(color) => color.to_rgb(),
            Color::HunterLab(color) => color.to_rgb(),
            Color::Xyz(color) => color.to_rgb(),
            Color::Yxy(color) => color.to_rgb(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{space, Color};

    #[test]
    fn test_color_display() {
        let rgb = Color::Rgb(space::Rgb::new(255.0, 0.0, 127.5));
        let hsv = Color::Hsv(space::Hsv::new(350.125, 0.9, 0.502));

        assert_eq!(rgb.to_string().as_str(), "rgb(255, 0, 127.5)");
        assert_eq!(hsv.to_string().as_str(), "hsv(350.13, 0.9, 0.5)");
    }
}
