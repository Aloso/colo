use std::{error::Error, fmt};

use crate::color::space;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseHexError {
    NotHexadecimal { string: String, c: char },
    NoDigits,
    TooManyDigits { string: String, got: u32, max: u32 },
    DigitsNotDivisibleBy3 { string: String, got: u32 },
}

impl Error for ParseHexError {}

impl fmt::Display for ParseHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseHexError::NotHexadecimal { string, c } => {
                write!(f, "{:?} in the string {:?} is not hexadecimal", c, string)
            }
            ParseHexError::NoDigits => write!(f, "No digits found"),
            ParseHexError::TooManyDigits { string, got, max } => write!(
                f,
                "Too many digits found in {:?} (max: {}, got: {})",
                string, max, got
            ),
            ParseHexError::DigitsNotDivisibleBy3 { string, got } => write!(
                f,
                "Number of digits ({}) in {:?} not divisible by 3",
                got, string
            ),
        }
    }
}

/// Parses a hex color (e.g. `#FF7700`).
///
/// Supported are colors with 1 to 8 digits per channel (e.g. `#F70`,
/// `#FFFFFFFF_77777777_00000000`). Underscores and leading `#` signs are
/// removed.
pub fn parse(input: &str) -> Result<space::Rgb, ParseHexError> {
    let color: String = input
        .trim_start_matches('#')
        .chars()
        .filter(|&c| c != '_')
        .map(|c| {
            if c.is_ascii_hexdigit() {
                Ok(c)
            } else {
                Err(ParseHexError::NotHexadecimal {
                    string: input.into(),
                    c,
                })
            }
        })
        .collect::<Result<_, ParseHexError>>()?;

    if color.is_empty() {
        return Err(ParseHexError::NoDigits);
    }
    if color.len() % 3 != 0 {
        return Err(ParseHexError::DigitsNotDivisibleBy3 {
            string: input.into(),
            got: color.len() as u32,
        });
    }
    if color.len() > 24 {
        return Err(ParseHexError::TooManyDigits {
            string: input.into(),
            got: color.len() as u32,
            max: 24,
        });
    }

    let len = color.len() / 3;
    let (r, gb) = color.split_at(len);
    let (g, b) = gb.split_at(len);

    let r = hex_to_f64(r);
    let g = hex_to_f64(g);
    let b = hex_to_f64(b);

    Ok(scale_down(r, g, b, len))
}

/// Converts a hexadecimal string to a float
fn hex_to_f64(src: &str) -> f64 {
    u32::from_str_radix(src, 16).expect("Invalid hexadecimal number") as f64
}

/// Scales the number down to 2 hexadecimal places and converts it to a `Rgb`
/// color. The original length is specified as `len`.
fn scale_down(r: f64, g: f64, b: f64, len: usize) -> space::Rgb {
    let up = match len {
        1 => 0xF,
        2 => 0xFF,
        3 => 0xFFF,
        4 => 0xFFFF,
        5 => 0xFFFFF,
        6 => 0xFFFFFF,
        7 => 0xFFFFFFF,
        8 => 0xFFFFFFFFu32,
        _ => unreachable!("The number has more than 8 hex digits"),
    } as f64;
    let factor = up / 255.0;
    space::Rgb::new(r / factor, g / factor, b / factor)
}

/// Converts an RGB color to hexadecimal notation
pub fn rgb_to_u32(rgb: space::Rgb) -> u32 {
    ((rgb.r.round() as u32) << 16) + ((rgb.g.round() as u32) << 8) + rgb.b.round() as u32
}

#[cfg(test)]
mod tests {
    use super::{parse, rgb_to_u32, space::Rgb};

    fn rgb_to_string(rgb: Rgb) -> String {
        format!("#{:06x}", rgb_to_u32(rgb))
    }

    #[test]
    fn test_from_rgb() {
        let rgb = Rgb {
            r: 15.0,
            g: 0.0,
            b: 255.0,
        };
        assert_eq!(rgb_to_string(rgb), String::from("#0f00FF"));
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
        assert_eq!(rgb_to_string(parse("224466").unwrap()), "#224466");
        assert_eq!(rgb_to_string(parse("246").unwrap()), "#224466");
        assert_eq!(rgb_to_string(parse("222_444_666").unwrap()), "#224466");
        assert_eq!(rgb_to_string(parse("2222_4444_6666").unwrap()), "#224466");
        assert_eq!(
            rgb_to_string(parse("222222_444444_666666").unwrap()),
            "#224466"
        );
        assert_eq!(
            rgb_to_string(parse("12345678_3456789A_56789ABC").unwrap()),
            "#123456"
        );
    }
}
