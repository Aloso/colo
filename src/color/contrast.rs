use super::space::Rgb;

/// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef
pub fn contrast(c1: Rgb, c2: Rgb) -> f64 {
    let (l1, l2) = (relative_luminance(c1), relative_luminance(c2));
    let (l1, l2) = (f64::max(l1, l2), f64::min(l1, l2));

    (l1 + 0.05) / (l2 + 0.05)
}

/// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
#[allow(non_snake_case)]
pub fn relative_luminance(color: Rgb) -> f64 {
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
