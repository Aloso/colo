//! Calculate text contrast and relative luminance of colors.
//!
//! - https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef
//! - https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
//!

use super::space::Rgb;

/// Visible contrast between two colors, which is
/// a value between 1 (no contrast) and 21 (high contrast).
pub(crate) fn contrast(l1: f64, l2: f64) -> f64 {
    let higher = f64::max(l1, l2);
    let lower = f64::min(l1, l2);

    (higher + 0.05) / (lower + 0.05)
}

/// The relative brightness of any point in a colorspace,
/// normalized to 0 for darkest black and 1 for lightest white
#[allow(non_snake_case)]
pub(crate) fn relative_luminance(color: Rgb) -> f64 {
    let r = clamp_rgb(color.r) / 255.0;
    let g = clamp_rgb(color.g) / 255.0;
    let b = clamp_rgb(color.b) / 255.0;

    let R = get_RGB(r);
    let G = get_RGB(g);
    let B = get_RGB(b);

    0.2126 * R + 0.7152 * G + 0.0722 * B
}

fn clamp_rgb(n: f64) -> f64 {
    f64::max(0.0, f64::min(255.0, n))
}

#[allow(non_snake_case)]
fn get_RGB(n: f64) -> f64 {
    if n <= 0.03928 {
        n / 12.92
    } else {
        ((n + 0.055) / 1.055).powf(2.4)
    }
}
