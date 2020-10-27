use std::fmt;

use super::{Color, ColorSpace};

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
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
