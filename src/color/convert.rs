//! Module for converting a color space and color components to a `Color`.

use std::convert::TryFrom;

use super::{space::*, Color, ColorSpace, ParseError};

impl TryFrom<(ColorSpace, &[f64])> for Color {
    type Error = ParseError;

    fn try_from((space, vals): (ColorSpace, &[f64])) -> Result<Self, Self::Error> {
        let required_args = space.num_components();

        if vals.len() != required_args {
            return Err(ParseError::NumberOfComponents {
                expected: required_args,
                got: vals.len(),
            });
        }

        // Create the color and check if the values are in the valid range
        match space {
            ColorSpace::Rgb => Color::try_from(Rgb::new(vals[0], vals[1], vals[2])),
            ColorSpace::Cmy => Color::try_from(Cmy::new(vals[0], vals[1], vals[2])),
            ColorSpace::Cmyk => Color::try_from(Cmyk::new(vals[0], vals[1], vals[2], vals[3])),
            ColorSpace::Hsv => Color::try_from(Hsv::new(vals[0], vals[1], vals[2])),
            ColorSpace::Hsl => Color::try_from(Hsl::new(vals[0], vals[1], vals[2])),
            ColorSpace::Lch => Color::try_from(Lch::new(vals[0], vals[1], vals[2])),
            ColorSpace::Luv => Color::try_from(Luv::new(vals[0], vals[1], vals[2])),
            ColorSpace::Lab => Color::try_from(Lab::new(vals[0], vals[1], vals[2])),
            ColorSpace::HunterLab => Color::try_from(HunterLab::new(vals[0], vals[1], vals[2])),
            ColorSpace::Xyz => Color::try_from(Xyz::new(vals[0], vals[1], vals[2])),
            ColorSpace::Yxy => Color::try_from(Yxy::new(vals[0], vals[1], vals[2])),
            ColorSpace::Gray => Color::try_from(Gray::new(vals[0])),
        }
    }
}

pub(crate) fn color_from_components_unchecked(space: ColorSpace, vals: &[f64]) -> Color {
    let required_args = space.num_components();

    if vals.len() != required_args {
        Err::<(), _>(ParseError::NumberOfComponents {
            expected: required_args,
            got: vals.len(),
        })
        .unwrap();
    }

    // Create the color and check if the values are in the valid range
    match space {
        ColorSpace::Rgb => Color::Rgb(Rgb::new(vals[0], vals[1], vals[2])),
        ColorSpace::Cmy => Color::Cmy(Cmy::new(vals[0], vals[1], vals[2])),
        ColorSpace::Cmyk => Color::Cmyk(Cmyk::new(vals[0], vals[1], vals[2], vals[3])),
        ColorSpace::Hsv => Color::Hsv(Hsv::new(vals[0], vals[1], vals[2])),
        ColorSpace::Hsl => Color::Hsl(Hsl::new(vals[0], vals[1], vals[2])),
        ColorSpace::Lch => Color::Lch(Lch::new(vals[0], vals[1], vals[2])),
        ColorSpace::Luv => Color::Luv(Luv::new(vals[0], vals[1], vals[2])),
        ColorSpace::Lab => Color::Lab(Lab::new(vals[0], vals[1], vals[2])),
        ColorSpace::HunterLab => Color::HunterLab(HunterLab::new(vals[0], vals[1], vals[2])),
        ColorSpace::Xyz => Color::Xyz(Xyz::new(vals[0], vals[1], vals[2])),
        ColorSpace::Yxy => Color::Yxy(Yxy::new(vals[0], vals[1], vals[2])),
        ColorSpace::Gray => Color::Gray(Gray::new(vals[0])),
    }
}

/// Implements `TryFrom<$ty>` for `Color`. The conversion fails if any
/// color component isn't in the valid range.
macro_rules! try_from_color {
    ($ty:ident -> $( $component:ident : $min:literal to $max:literal );* $(;)?) => {
        impl TryFrom<$ty> for Color {
            type Error = ParseError;

            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                $(
                    min_max(stringify!($component), $min, $max, value.$component)?;
                )*
                Ok(Self::$ty(value))
            }
        }
    };
}

try_from_color! { Rgb ->
    r: 0.0 to 255.0;
    g: 0.0 to 255.0;
    b: 0.0 to 255.0;
}
try_from_color! { Cmy ->
    c: 0.0 to 1.0;
    m: 0.0 to 1.0;
    y: 0.0 to 1.0;
}
try_from_color! { Cmyk ->
    c: 0.0 to 1.0;
    m: 0.0 to 1.0;
    y: 0.0 to 1.0;
    k: 0.0 to 1.0;
}
try_from_color! { Hsv ->
    h: -360.0 to 360.0;
    s: 0.0 to 1.0;
    v: 0.0 to 1.0;
}
try_from_color! { Hsl ->
    h: -360.0 to 360.0;
    s: 0.0 to 1.0;
    l: 0.0 to 1.0;
}
try_from_color! { Lch ->
    l: 0.0 to 100.0;
    c: 0.0 to 100.0;
    h: -360.0 to 360.0;
}
try_from_color! { Luv ->
    l: 0.0 to 100.0;
    u: -134.0 to 220.0;
    v: -140.0 to 122.0;
}
try_from_color! { Lab ->
    l: 0.0 to 100.0;
}
try_from_color! { HunterLab ->
    l: 0.0 to 100.0;
}
try_from_color! { Xyz ->
}
try_from_color! { Yxy ->
}
try_from_color! { Gray ->
    l: 0.0 to 1.0;
}

/// Checks that the value is in the specified range. If it isn't, an error is
/// returned.
fn min_max(component: &'static str, min: f64, max: f64, got: f64) -> Result<(), ParseError> {
    if got < min || got > max {
        if min == 0.0 && got < min {
            Err(ParseError::Negative { component, got })
        } else {
            Err(ParseError::OutOfRange {
                component,
                min,
                max,
                got,
            })
        }
    } else {
        Ok(())
    }
}

macro_rules! from_color_for {
    ($t:ty) => {
        impl From<Color> for $t {
            fn from(c: Color) -> Self {
                match c {
                    Color::Rgb(c) => c.into(),
                    Color::Cmy(c) => c.into(),
                    Color::Cmyk(c) => c.into(),
                    Color::Hsv(c) => c.into(),
                    Color::Hsl(c) => c.into(),
                    Color::Lch(c) => c.into(),
                    Color::Luv(c) => c.into(),
                    Color::Lab(c) => c.into(),
                    Color::HunterLab(c) => c.into(),
                    Color::Xyz(c) => c.into(),
                    Color::Yxy(c) => c.into(),
                    Color::Gray(c) => c.into(),
                }
            }
        }
    };
}

from_color_for!(Rgb);
from_color_for!(Cmy);
from_color_for!(Cmyk);
from_color_for!(Hsv);
from_color_for!(Hsl);
from_color_for!(Lch);
from_color_for!(Luv);
from_color_for!(Lab);
from_color_for!(HunterLab);
from_color_for!(Xyz);
from_color_for!(Yxy);
from_color_for!(Gray);
