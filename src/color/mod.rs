use color_space::{FromRgb, ToRgb};
use std::fmt;

use space::*;

pub use format::ColorFormat;
pub use parse::{parse, ParseError};
pub use space::ColorSpace;

mod convert;
mod parse;

pub mod ansi;
pub mod format;
pub mod hex;
pub mod html;
pub mod space;

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
    pub fn new(color_space: ColorSpace, components: &[f64]) -> Result<Self, ParseError> {
        std::convert::TryFrom::try_from((color_space, components))
    }

    /// Return the color space, without the color components
    pub fn get_color_space(&self) -> ColorSpace {
        match self {
            Color::Rgb(_) => ColorSpace::Rgb,
            Color::Cmy(_) => ColorSpace::Cmy,
            Color::Cmyk(_) => ColorSpace::Cmyk,
            Color::Hsv(_) => ColorSpace::Hsv,
            Color::Hsl(_) => ColorSpace::Hsl,
            Color::Lch(_) => ColorSpace::Lch,
            Color::Luv(_) => ColorSpace::Luv,
            Color::Lab(_) => ColorSpace::Lab,
            Color::HunterLab(_) => ColorSpace::HunterLab,
            Color::Xyz(_) => ColorSpace::Xyz,
            Color::Yxy(_) => ColorSpace::Yxy,
        }
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
        let current_space = self.get_color_space();
        if current_space == color_space {
            return *self;
        }
        let rgb = self.to_rgb();
        match color_space {
            ColorSpace::Rgb => Color::Rgb(rgb),
            ColorSpace::Cmy => Color::Cmy(Cmy::from_rgb(&rgb)),
            ColorSpace::Cmyk => {
                /// TODO: Use `Cmyk::from_rgb` from the color_space crate, as
                /// soon as that function works correctly
                fn cmyk_from_rgb(rgb: &Rgb) -> Cmyk {
                    let cmy = Cmy::from_rgb(rgb);
                    let k = cmy.c.min(cmy.m.min(cmy.y.min(1.0)));
                    match (k - 1.0).abs() < 1e-3 {
                        true => Cmyk::new(0.0, 0.0, 0.0, k),
                        false => Cmyk::new(
                            (cmy.c - k) / (1.0 - k),
                            (cmy.m - k) / (1.0 - k),
                            (cmy.y - k) / (1.0 - k),
                            k,
                        ),
                    }
                }

                Color::Cmyk(cmyk_from_rgb(&rgb))
            }
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

    pub fn to_term_color(&self) -> colored::Color {
        let Rgb { r, g, b } = Self::clamp_rgb(self.to_rgb());
        colored::Color::TrueColor {
            r: r.round() as u8,
            g: g.round() as u8,
            b: b.round() as u8,
        }
    }

    pub fn text_color(&self) -> TextColor {
        let lab = match self.to_color_space(ColorSpace::Lab) {
            Color::Lab(lab) => lab,
            _ => unreachable!("Conversion to Lab unsuccessful"),
        };

        let lab_distance = (lab.a.abs().powi(2) + lab.b.abs().powi(2)).sqrt();
        let colorfulness = lab_distance.min(100.0) / 12.0; // empirically determined values

        if lab.l < (60.0 - colorfulness) {
            TextColor::White
        } else {
            TextColor::Black
        }
    }

    fn clamp_rgb(rgb: Rgb) -> Rgb {
        Rgb {
            r: rgb.r.min(255.0).max(0.0),
            g: rgb.g.min(255.0).max(0.0),
            b: rgb.b.min(255.0).max(0.0),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextColor {
    Black,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (color_space, parts) = self.divide();
        write!(f, "{}(", color_space)?;
        let last = parts.len() - 1;
        for (i, part) in parts.into_iter().enumerate() {
            write!(f, "{}", (part * 1000.0).round() / 1000.0)?;
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
        let hsv = Color::Hsv(space::Hsv::new(350.0125, 0.9, 0.5002));

        assert_eq!(rgb.to_string().as_str(), "rgb(255, 0, 127.5)");
        assert_eq!(hsv.to_string().as_str(), "hsv(350.013, 0.9, 0.5)");
    }
}
