use super::space::*;
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

macro_rules! impl_froms {
    ($t:ty) => {
        impl From<$t> for Gray {
            fn from(c: $t) -> Self {
                Hsl::from(c).into()
            }
        }

        impl From<Gray> for $t {
            fn from(g: Gray) -> Self {
                Hsl::from(g).into()
            }
        }
    };
}

impl From<Hsl> for Gray {
    fn from(hsl: Hsl) -> Self {
        Self { l: hsl.l }
    }
}

impl From<Gray> for Hsl {
    fn from(g: Gray) -> Self {
        Hsl::new(0.0, 0.0, g.l)
    }
}

impl ToRgb for Gray {
    fn to_rgb(&self) -> Rgb {
        Hsl::from(*self).into()
    }
}

impl FromRgb for Gray {
    fn from_rgb(rgb: &Rgb) -> Self {
        Hsl::from(*rgb).into()
    }
}

impl_froms!(Rgb);
impl_froms!(Cmy);
impl_froms!(Cmyk);
impl_froms!(Hsv);
impl_froms!(Lch);
impl_froms!(Luv);
impl_froms!(Lab);
impl_froms!(HunterLab);
impl_froms!(Xyz);
impl_froms!(Yxy);
