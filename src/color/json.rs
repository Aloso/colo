use crate::color::spaces;

pub fn from_rgb(rgb: spaces::Rgb) -> String {
    format!(r#"{{"r":{},"g":{},"b":{}}}"#, rgb.r, rgb.g, rgb.b)
}
