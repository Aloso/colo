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
