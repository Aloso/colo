use crate::color::space;

pub fn from_rgb(rgb: space::Rgb) -> String {
    format!(r#"{{"r":{},"g":{},"b":{}}}"#, rgb.r, rgb.g, rgb.b)
}
