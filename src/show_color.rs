use anyhow::{anyhow, bail, Result};
use color_space::ToRgb;
use crossterm::{queue, style::*, tty::IsTty};
use std::fmt::Debug;
use std::io::{stdout, Write};

use crate::{colors::ColorSpace, html_colors::HtmlColors};

pub fn show_color(color_space: ColorSpace, values: &[f64]) -> Result<()> {
    let required_args = if color_space == ColorSpace::Cmyk {
        4
    } else {
        3
    };
    if values.len() != required_args {
        bail!(
            "The color space {:?} requires {} arguments, but {} were provided",
            color_space,
            required_args,
            values.len()
        );
    }
    match color_space {
        ColorSpace::Rgb => show(color_space::Rgb {
            r: values[0],
            g: values[1],
            b: values[2],
        }),
        ColorSpace::Cmy => show(color_space::Cmy {
            c: values[0],
            m: values[1],
            y: values[2],
        }),
        ColorSpace::Cmyk => show(color_space::Cmyk {
            c: values[0],
            m: values[1],
            y: values[2],
            k: values[3],
        }),
        ColorSpace::Hsv => show(color_space::Hsv {
            h: values[0],
            s: values[1],
            v: values[2],
        }),
        ColorSpace::Hsl => show(color_space::Hsl {
            h: values[0],
            s: values[1],
            l: values[2],
        }),
        ColorSpace::Lch => show(color_space::Lch {
            l: values[0],
            c: values[1],
            h: values[2],
        }),
        ColorSpace::Luv => show(color_space::Luv {
            l: values[0],
            u: values[1],
            v: values[2],
        }),
        ColorSpace::Lab => show(color_space::Lab {
            l: values[0],
            a: values[1],
            b: values[2],
        }),
        ColorSpace::HunterLab => show(color_space::HunterLab {
            l: values[0],
            a: values[1],
            b: values[2],
        }),
        ColorSpace::Xyz => show(color_space::Xyz {
            x: values[0],
            y: values[1],
            z: values[2],
        }),
        ColorSpace::Yxy => show(color_space::Yxy {
            y1: values[0],
            x: values[1],
            y2: values[2],
        }),
    }?;
    Ok(())
}

pub fn show_hex_or_html_color(color: &str) -> Result<()> {
    let html_colors = HtmlColors::new();
    if let Some(rgb) = html_colors.get(color) {
        show(rgb)?;
    } else {
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

        show(color_space::Rgb { r, g, b })?;
    }
    Ok(())
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

fn show(color: impl ToRgb + Debug) -> Result<()> {
    let color_dbg = format!("{:?}", color);
    let rgb = color.to_rgb();
    let rgb_dbg = format!("{:?}", rgb);
    let msg = if color_dbg == rgb_dbg {
        color_dbg
    } else {
        color_dbg + " = " + &rgb_dbg
    };

    show_impl(rgb, msg)
}

fn show_impl(rgb: color_space::Rgb, msg: String) -> Result<()> {
    let crossterm_color = Color::Rgb {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    let mut stdout = stdout();

    if stdout.is_tty() {
        queue!(stdout, Print(msg), Print("\n"))?;
        queue!(
            stdout,
            SetForegroundColor(crossterm_color),
            Print(" ▄▄▄▄▄▄▄▄▄▄\n ██████████\n ██████████\n ██████████\n ██████████\n ▀▀▀▀▀▀▀▀▀▀\n"),
            ResetColor,
        )?;
    } else {
        queue!(stdout, Print(rgb_to_json(rgb)))?;
    }

    Ok(())
}

fn rgb_to_json(rgb: color_space::Rgb) -> String {
    format!(r#"{{"r":{},"g":{},"b":{}}}"#, rgb.r, rgb.g, rgb.b)
}
