use color_space::ToRgb;

use super::{hex, html, Color, ColorSpace};

pub const PREFERRED_FORMATS: [&[ColorFormat]; 6] = [
    &[
        ColorFormat::Hex,
        ColorFormat::Html,
        ColorFormat::Normal(ColorSpace::Rgb),
    ],
    &[
        ColorFormat::Normal(ColorSpace::Hsl),
        ColorFormat::Normal(ColorSpace::Hsv),
    ],
    &[
        ColorFormat::Normal(ColorSpace::Cmy),
        ColorFormat::Normal(ColorSpace::Cmyk),
    ],
    &[
        ColorFormat::Normal(ColorSpace::Lch),
        ColorFormat::Normal(ColorSpace::Luv),
    ],
    &[
        ColorFormat::Normal(ColorSpace::Lab),
        ColorFormat::Normal(ColorSpace::HunterLab),
    ],
    &[
        ColorFormat::Normal(ColorSpace::Xyz),
        ColorFormat::Normal(ColorSpace::Yxy),
    ],
];

pub const PREFERRED_FORMATS_SHORT: [ColorFormat; 4] = [
    ColorFormat::Hex,
    ColorFormat::Normal(ColorSpace::Rgb),
    ColorFormat::Html,
    ColorFormat::Normal(ColorSpace::Hsl),
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorFormat {
    Normal(ColorSpace),
    Hex,
    Html,
}

impl Default for ColorFormat {
    fn default() -> Self {
        ColorFormat::Hex
    }
}

impl ColorFormat {
    pub fn format(&self, color: Color) -> Option<String> {
        Some(match *self {
            ColorFormat::Normal(space) => {
                let (_, parts) = color.to_color_space(space).divide();
                let a = *parts.get(0).unwrap_or(&0.0);
                let b = *parts.get(1).unwrap_or(&0.0);
                let c = *parts.get(2).unwrap_or(&0.0);
                let d = *parts.get(3).unwrap_or(&0.0);

                match space {
                    ColorSpace::Rgb => format!("rgb({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::Cmy => format!("cmy({}%, {}%, {}%)", p(a), p(b), p(c)),
                    ColorSpace::Cmyk => format!("cmyk({}%, {}%, {}%, {}%)", p(a), p(b), p(c), p(d)),
                    ColorSpace::Hsv => format!("hsv({}, {}%, {}%)", r(a), p(b), p(c)),
                    ColorSpace::Hsl => format!("hsl({}, {}%, {}%)", r(a), p(b), p(c)),
                    ColorSpace::Lch => format!("lch({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::Luv => format!("luv({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::Lab => format!("lab({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::HunterLab => format!("hunterlab({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::Xyz => format!("xyz({}, {}, {})", r(a), r(b), r(c)),
                    ColorSpace::Yxy => format!("yxy({}, {}, {})", r(a), r(b), r(c)),
                }
            }
            ColorFormat::Hex => format!("#{:06x}", hex::rgb_to_u32(color.to_rgb())),
            ColorFormat::Html => {
                let name = html::get_name(color.to_rgb())?;
                name.to_string()
            }
        })
    }
}

/// Round to 1 decimal digit
fn r(num: f64) -> f64 {
    (num * 10.0).round() / 10.0
}

/// Multiply with 100 and round to 1 decimal digit
fn p(num: f64) -> f64 {
    (num * 1000.0).round() / 10.0
}
