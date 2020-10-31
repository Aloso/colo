use crate::color::Color;

macro_rules! build_string {
    ($color:expr, $i1:ident, $i2:ident, $i3:ident) => {
        format!(
            concat!(
                "{{\"",
                stringify!($i1),
                "\":{},\"",
                stringify!($i2),
                "\":{},\"",
                stringify!($i3),
                "\":{}}}"
            ),
            $color.$i1, $color.$i2, $color.$i3
        )
    };
    ($color:expr, $i1:ident, $i2:ident, $i3:ident, $i4:ident) => {
        format!(
            concat!(
                "{{\"",
                stringify!($i1),
                "\":{},\"",
                stringify!($i2),
                "\":{},\"",
                stringify!($i3),
                "\":{},\"",
                stringify!($i4),
                "\":{}}}"
            ),
            $color.$i1, $color.$i2, $color.$i3, $color.$i4
        )
    };
}

/// Create a JSON string from the color.
pub fn from_color(color: Color) -> String {
    match color {
        Color::Rgb(color) => build_string!(color, r, g, b),
        Color::Cmy(color) => build_string!(color, c, m, y),
        Color::Cmyk(color) => build_string!(color, c, m, y, k),
        Color::Hsv(color) => build_string!(color, h, s, v),
        Color::Hsl(color) => build_string!(color, h, s, l),
        Color::Lch(color) => build_string!(color, l, c, h),
        Color::Luv(color) => build_string!(color, l, u, v),
        Color::Lab(color) => build_string!(color, l, a, b),
        Color::HunterLab(color) => build_string!(color, l, a, b),
        Color::Xyz(color) => build_string!(color, x, y, z),
        Color::Yxy(color) => build_string!(color, y1, x, y2),
    }
}

#[cfg(test)]
mod tests {
    use space::ColorSpace;

    use crate::color::{space, Color};

    #[test]
    fn test_json_generation() {
        let rgb = Color::Rgb(space::Rgb::new(255.0, 0.0, 127.5));
        let hsv = rgb.to_color_space(ColorSpace::Hsv);

        assert_eq!(
            super::from_color(rgb).as_str(),
            r#"{"r":255,"g":0,"b":127.5}"#,
        );
        assert_eq!(super::from_color(hsv).as_str(), r#"{"h":330,"s":1,"v":1}"#);
    }
}
