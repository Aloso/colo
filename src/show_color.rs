use anyhow::Result;
use color_space::ToRgb;
use crossterm::style::{self, Print, ResetColor, SetForegroundColor};
use crossterm::{queue, tty::IsTty};
use std::io::{stdout, Write};

use crate::color::{hex, html, json, space, Color, ColorSpace};

pub fn show(color: Color, out: ColorSpace, size: u32, text: Option<String>) -> Result<()> {
    let rgb = color.to_rgb();
    let input = color.to_string();
    let converted = color.to_color_space(out);
    let json = json::from_color(converted);
    let converted = converted.to_string();

    let second_str = if input != converted {
        converted
    } else {
        hex::from_rgb(rgb)
    };

    show_impl(rgb, input + " ~ " + &second_str, json, size, text)
}

pub fn show_hex_or_html(
    color: &str,
    out: ColorSpace,
    size: u32,
    text: Option<String>,
) -> Result<()> {
    let rgb = html::get(color).map_or_else(|| hex::parse(color), Ok)?;
    let input = hex::from_rgb(rgb);
    let converted = Color::Rgb(rgb).to_color_space(out);
    let json = json::from_color(converted);
    let converted = converted.to_string();

    show_impl(rgb, input + " ~ " + &converted, json, size, text)
}

fn show_impl(
    rgb: space::Rgb,
    msg: String,
    json: String,
    size: u32,
    text: Option<String>,
) -> Result<()> {
    let crossterm_color = style::Color::Rgb {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    let mut stdout = stdout();

    if !stdout.is_tty() {
        queue!(stdout, Print(json))?;
    } else if let Some(text) = text {
        queue!(
            stdout,
            SetForegroundColor(crossterm_color),
            Print(text),
            Print("\n"),
            ResetColor
        )?;
    } else {
        queue!(stdout, Print(msg), Print("\n"))?;
        let square = make_square(size);
        queue!(
            stdout,
            SetForegroundColor(crossterm_color),
            Print(&square),
            ResetColor,
        )?;
    }

    Ok(())
}

/// Generates an ASCII square with the given size
fn make_square(size: u32) -> String {
    if size == 0 {
        return String::new();
    }

    let mut s = String::from(" ");
    for _ in 0..size {
        s.push('▄');
        s.push('▄');
    }
    s.push('\n');
    s.push(' ');
    for _ in 0..size - 1 {
        for _ in 0..size {
            s.push('█');
            s.push('█');
        }
        s.push('\n');
        s.push(' ');
    }
    for _ in 0..size {
        s.push('▀');
        s.push('▀');
    }
    s.push('\n');
    s
}
