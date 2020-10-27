use anyhow::Result;
use color_space::ToRgb;
use crossterm::style::{self, Print, ResetColor, SetForegroundColor};
use crossterm::{queue, tty::IsTty};
use std::fmt::Debug;
use std::io::{stdout, Write};

use crate::color::{hex, html, json, spaces, Color};

pub fn show(color: Color) -> Result<()> {
    let printed = color.to_string();
    match color {
        Color::Rgb(c) => show_generic(c, printed, Some(hex::from_rgb(c))),
        Color::Cmy(c) => show_generic(c, printed, None),
        Color::Cmyk(c) => show_generic(c, printed, None),
        Color::Hsv(c) => show_generic(c, printed, None),
        Color::Hsl(c) => show_generic(c, printed, None),
        Color::Lch(c) => show_generic(c, printed, None),
        Color::Luv(c) => show_generic(c, printed, None),
        Color::Lab(c) => show_generic(c, printed, None),
        Color::HunterLab(c) => show_generic(c, printed, None),
        Color::Xyz(c) => show_generic(c, printed, None),
        Color::Yxy(c) => show_generic(c, printed, None),
    }
}

pub fn show_hex_or_html(color: &str) -> Result<()> {
    let rgb = html::get_single(color).map_or_else(|| hex::parse(color), Ok)?;
    show_generic(rgb, hex::from_rgb(rgb), None)?;
    Ok(())
}

fn show_generic(
    color: impl ToRgb + Debug,
    mut input: String,
    converted: Option<String>,
) -> Result<()> {
    let rgb = color.to_rgb();

    input.push_str(" ~ ");
    input.push_str(&converted.unwrap_or_else(|| Color::Rgb(rgb).to_string()));

    show_impl(rgb, input)
}

fn show_impl(rgb: spaces::Rgb, msg: String) -> Result<()> {
    let crossterm_color = style::Color::Rgb {
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
            Print(&make_square(4)),
            ResetColor,
        )?;
    } else {
        queue!(stdout, Print(json::from_rgb(rgb)))?;
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
