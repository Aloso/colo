use std::{fmt, str::FromStr};

pub use color_space::{Cmy, Cmyk, Hsl, Hsv, HunterLab, Lab, Lch, Luv, Rgb, Xyz, Yxy};
use color_space::{FromRgb, ToRgb};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gray {
    pub l: f64,
}

impl Gray {
    pub fn new(l: f64) -> Self {
        Self { l }
    }
}

impl ToRgb for Gray {
    fn to_rgb(&self) -> Rgb {
        Hsl::new(0.0, 0.0, self.l).into()
    }
}

impl FromRgb for Gray {
    fn from_rgb(rgb: &Rgb) -> Self {
        let hsl = Hsl::from_rgb(rgb);
        Self { l: hsl.l }
    }
}

/// A C-like enum listing all supported color spaces
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
    Gray,
}

impl ColorSpace {
    /// Returns the number of color components (between 1 and 4).
    pub fn num_components(&self) -> usize {
        match self {
            ColorSpace::Cmyk => 4,
            ColorSpace::Gray => 1,
            _ => 3,
        }
    }
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ColorSpace::Rgb => "rgb",
            ColorSpace::Cmy => "cmy",
            ColorSpace::Cmyk => "cmyk",
            ColorSpace::Hsv => "hsv",
            ColorSpace::Hsl => "hsl",
            ColorSpace::Lch => "lch",
            ColorSpace::Luv => "luv",
            ColorSpace::Lab => "lab",
            ColorSpace::HunterLab => "hunterlab",
            ColorSpace::Xyz => "xyz",
            ColorSpace::Yxy => "yxy",
            ColorSpace::Gray => "gry",
        })
    }
}

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[error("The string is not a valid color space")]
pub struct ColorSpaceParseError;

impl FromStr for ColorSpace {
    type Err = ColorSpaceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower;
        let lower = if s.contains(|c: char| c.is_ascii_uppercase()) {
            lower = s.to_ascii_lowercase();
            &lower
        } else {
            s
        };
        Ok(match lower {
            "rgb" => ColorSpace::Rgb,
            "cmy" => ColorSpace::Cmy,
            "cmyk" => ColorSpace::Cmyk,
            "hsv" => ColorSpace::Hsv,
            "hsl" => ColorSpace::Hsl,
            "lch" => ColorSpace::Lch,
            "luv" => ColorSpace::Luv,
            "lab" => ColorSpace::Lab,
            "hunterlab" => ColorSpace::HunterLab,
            "xyz" => ColorSpace::Xyz,
            "yxy" => ColorSpace::Yxy,
            "gry" => ColorSpace::Gray,
            _ => return Err(ColorSpaceParseError),
        })
    }
}
