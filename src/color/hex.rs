use crate::color::space;
use anyhow::{bail, Context, Result};

/// Parses a hex color (e.g. `#FF7700`).
///
/// Supported are colors with 1 to 8 digits per channel (e.g. `#F70`,
/// `#FFFFFFFF_77777777_00000000`). Underscores and leading `#` signs are removed.
pub fn parse(color: &str) -> Result<space::Rgb> {
    let color: String = color
        .trim_start_matches('#')
        .chars()
        .filter(|&c| c != '_')
        .collect();

    if color.is_empty() {
        bail!("color is empty");
    }
    if color.len() % 3 != 0 {
        bail!("{:?} doesn't have a length divisible by 3", color);
    }
    if color.len() > 24 {
        bail!(
            "{:?} is too long, only hexadecimal numbers of up to 24 digits are supported",
            color,
        );
    }

    let len = color.len() / 3;
    let (r, gb) = color.split_at(len);
    let (g, b) = gb.split_at(len);

    let r = hex_to_f64(r)?;
    let g = hex_to_f64(g)?;
    let b = hex_to_f64(b)?;

    scale_down(r, g, b, len)
}

/// Converts a hexadecimal string to a float
fn hex_to_f64(src: &str) -> Result<f64> {
    Ok(u32::from_str_radix(src, 16)
        .with_context(|| format!("{:?} could not be parsed as a hexadecimal number", src))?
        as f64)
}

/// Scales the number down to 2 hexadecimal places and converts it to a `Rgb` color.
/// The original length is specified as `len`.
fn scale_down(r: f64, g: f64, b: f64, len: usize) -> Result<space::Rgb> {
    let up = match len {
        1 => 0xF,
        2 => 0xFF,
        3 => 0xFFF,
        4 => 0xFFFF,
        5 => 0xFFFFF,
        6 => 0xFFFFFF,
        7 => 0xFFFFFFF,
        8 => 0xFFFFFFFFu32,
        _ => bail!("length {} is greater than 8", len),
    } as f64;
    let factor = up / 255.0;
    Ok(space::Rgb::new(r / factor, g / factor, b / factor))
}

/// Converts an RGB color to hexadecimal notation
pub fn from_rgb(rgb: space::Rgb) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        rgb.r.round() as u8,
        rgb.g.round() as u8,
        rgb.b.round() as u8
    )
}

#[cfg(test)]
mod tests {
    use super::{from_rgb, parse, space::Rgb};

    #[test]
    fn test_from_rgb() {
        let rgb = Rgb {
            r: 15.0,
            g: 0.0,
            b: 255.0,
        };
        assert_eq!(from_rgb(rgb), String::from("#0F00FF"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("000").unwrap(), Rgb::new(0.0, 0.0, 0.0));
        assert_eq!(parse("FFF").unwrap(), Rgb::new(255.0, 255.0, 255.0));
        assert_eq!(
            parse("123456").unwrap(),
            Rgb::new(0x12 as f64, 0x34 as f64, 0x56 as f64),
        );
        assert_eq!(
            parse("22222222_44444444_66666666").unwrap(),
            Rgb::new(0x22 as f64, 0x44 as f64, 0x66 as f64),
        );
    }

    #[test]
    fn test_parse_and_to_hex() {
        assert_eq!(from_rgb(parse("224466").unwrap()), "#224466");
        assert_eq!(from_rgb(parse("246").unwrap()), "#224466");
        assert_eq!(from_rgb(parse("222_444_666").unwrap()), "#224466");
        assert_eq!(from_rgb(parse("2222_4444_6666").unwrap()), "#224466");
        assert_eq!(from_rgb(parse("222222_444444_666666").unwrap()), "#224466");
        assert_eq!(
            from_rgb(parse("12345678_3456789A_56789ABC").unwrap()),
            "#123456"
        );
    }
}
