use anyhow::Result;
use color_space::ToRgb;
use crossterm::style::{self, Print, ResetColor, SetForegroundColor};
use crossterm::{queue, tty::IsTty};
use std::fmt::Debug;
use std::io::{stdout, Write};

use crate::color::{hex, html, json, spaces, Color};

pub fn show(color: Color) -> Result<()> {
    match color {
        Color::Rgb(c) => show_generic(c, None),
        Color::Cmy(c) => show_generic(c, None),
        Color::Cmyk(c) => show_generic(c, None),
        Color::Hsv(c) => show_generic(c, None),
        Color::Hsl(c) => show_generic(c, None),
        Color::Lch(c) => show_generic(c, None),
        Color::Luv(c) => show_generic(c, None),
        Color::Lab(c) => show_generic(c, None),
        Color::HunterLab(c) => show_generic(c, None),
        Color::Xyz(c) => show_generic(c, None),
        Color::Yxy(c) => show_generic(c, None),
    }
}

pub fn show_hex_or_html(color: &str) -> Result<()> {
    if let Some(rgb) = html::get_single(color) {
        show_generic(rgb, Some(color.to_string() + " = " + &hex::from_rgb(rgb)))?;
    } else {
        let rgb = hex::parse(color)?;
        show_generic(rgb, Some(format!("{} = {:?}", hex::from_rgb(rgb), rgb)))?;
    }
    Ok(())
}

fn show_generic(color: impl ToRgb + Debug, msg: Option<String>) -> Result<()> {
    let color_dbg = format!("{:?}", color);
    let rgb = color.to_rgb();
    let rgb_dbg = format!("{:?}", rgb);

    let msg = msg.unwrap_or_else(|| {
        if color_dbg == rgb_dbg {
            color_dbg
        } else {
            color_dbg + " = " + &rgb_dbg
        }
    });

    show_impl(rgb, msg)
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
