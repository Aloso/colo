use std::{convert::TryFrom, error::Error, fmt};

use super::space::*;
use super::{Color, ColorSpace};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParseError {
    NumberOfComponents {
        expected: usize,
        got: usize,
    },
    Negative {
        component: &'static str,
        got: f64,
    },
    OutOfRange {
        component: &'static str,
        min: f64,
        max: f64,
        got: f64,
    },
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ParseError::NumberOfComponents { expected, got } => write!(
                f,
                "Wrong number of color components (expected {}, got {})",
                expected, got
            ),
            ParseError::Negative { component, got } => write!(
                f,
                "Color component {:?} can't be negative (got {})",
                component, got
            ),
            ParseError::OutOfRange {
                component,
                min,
                max,
                got,
            } => write!(
                f,
                "Color component {:?} out of range (expected {} to {}, got {})",
                component, min, max, got
            ),
        }
    }
}

impl TryFrom<(ColorSpace, &[f64])> for Color {
    type Error = ParseError;

    fn try_from((space, values): (ColorSpace, &[f64])) -> Result<Self, Self::Error> {
        let required_args = if space == ColorSpace::Cmyk { 4 } else { 3 };

        if values.len() != required_args {
            return Err(ParseError::NumberOfComponents {
                expected: required_args,
                got: values.len(),
            });
        }

        match space {
            ColorSpace::Rgb => Color::try_from(Rgb {
                r: values[0],
                g: values[1],
                b: values[2],
            }),
            ColorSpace::Cmy => Color::try_from(Cmy {
                c: values[0],
                m: values[1],
                y: values[2],
            }),
            ColorSpace::Cmyk => Color::try_from(Cmyk {
                c: values[0],
                m: values[1],
                y: values[2],
                k: values[3],
            }),
            ColorSpace::Hsv => Color::try_from(Hsv {
                h: values[0],
                s: values[1],
                v: values[2],
            }),
            ColorSpace::Hsl => Color::try_from(Hsl {
                h: values[0],
                s: values[1],
                l: values[2],
            }),
            ColorSpace::Lch => Color::try_from(Lch {
                l: values[0],
                c: values[1],
                h: values[2],
            }),
            ColorSpace::Luv => Color::try_from(Luv {
                l: values[0],
                u: values[1],
                v: values[2],
            }),
            ColorSpace::Lab => Color::try_from(Lab {
                l: values[0],
                a: values[1],
                b: values[2],
            }),
            ColorSpace::HunterLab => Color::try_from(HunterLab {
                l: values[0],
                a: values[1],
                b: values[2],
            }),
            ColorSpace::Xyz => Color::try_from(Xyz {
                x: values[0],
                y: values[1],
                z: values[2],
            }),
            ColorSpace::Yxy => Color::try_from(Yxy {
                y1: values[0],
                x: values[1],
                y2: values[2],
            }),
        }
    }
}

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
    h: 0.0 to 360.0;
    s: 0.0 to 1.0;
    v: 0.0 to 1.0;
}
try_from_color! { Hsl ->
    h: 0.0 to 360.0;
    s: 0.0 to 1.0;
    l: 0.0 to 1.0;
}
try_from_color! { Lch ->
    l: 0.0 to 100.0;
    c: 0.0 to 100.0;
    h: 0.0 to 360.0;
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
try_from_color! { Xyz -> }
try_from_color! { Yxy -> }

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
