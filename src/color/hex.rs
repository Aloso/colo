use crate::color::spaces;
use anyhow::{anyhow, bail, Result};

pub fn parse(color: &str) -> Result<spaces::Rgb> {
    let color = color.trim_start_matches('#');
    let chars: Vec<u8> = color
        .chars()
        .filter(|&c| c != '_')
        .map(|c| {
            c.to_digit(16)
                .map(|n| n as u8)
                .ok_or_else(|| anyhow!("{:?} is not a hexadecimal character", c))
        })
        .collect::<Result<_, _>>()?;

    if chars.is_empty() {
        bail!("string is empty");
    }
    if chars.len() % 3 != 0 {
        bail!(
            "{:?} doesn't have a length divisible by 3",
            format_hex(&chars),
        );
    }
    if chars.len() > 24 {
        bail!(
            "{:?} is too long, only RGB numbers of up to 24 digits are supported",
            format_hex(&chars),
        );
    }

    let len = chars.len() / 3;
    let r = &chars[..len];
    let g = &chars[len..len * 2];
    let b = &chars[len * 2..];

    let r = nibble_slice_to_u32(r) as f64;
    let g = nibble_slice_to_u32(g) as f64;
    let b = nibble_slice_to_u32(b) as f64;

    let (r, g, b) = scale_down(r, g, b, len)?;

    Ok(spaces::Rgb { r, g, b })
}

fn nibble_slice_to_u32(slice: &[u8]) -> u32 {
    let mut result = 0;
    for &n in slice {
        result = (result << 4) + n as u32;
    }
    result
}

fn scale_down(r: f64, g: f64, b: f64, len: usize) -> Result<(f64, f64, f64)> {
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
    Ok((r / factor, g / factor, b / factor))
}

fn format_hex(numbers: &[u8]) -> String {
    numbers
        .iter()
        .map(|&n| std::char::from_digit(n as u32, 16).unwrap())
        .collect::<String>()
}
