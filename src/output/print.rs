use anyhow::Result;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::color::space;

pub fn print(
    rgb: space::Rgb,
    bg: Option<space::Rgb>,
    text: String,
    italic: bool,
    bold: bool,
    underline: bool,
) -> Result<()> {
    let fg = colored::Color::TrueColor {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };
    let bg = bg.map(|c| colored::Color::TrueColor {
        r: c.r.round() as u8,
        g: c.g.round() as u8,
        b: c.b.round() as u8,
    });
    let mut text = text.color(fg);

    if italic {
        text = text.italic();
    }
    if bold {
        text = text.bold();
    }
    if underline {
        text = text.underline();
    }
    if let Some(bg) = bg {
        text = text.on_color(bg);
    }

    let mut stdout = stdout();
    write!(stdout, "{}", text)?;
    Ok(())
}
