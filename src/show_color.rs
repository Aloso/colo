use anyhow::Result;
use atty::Stream;
use color_space::ToRgb;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::color::{hex, html, json, space, Color, ColorSpace};

pub fn show(color: Color, out: ColorSpace, size: u32) -> Result<()> {
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

    show_impl(rgb, input + " ~ " + &second_str, json, size)
}

pub fn show_hex_or_html(color: &str, out: ColorSpace, size: u32) -> Result<()> {
    let rgb = html::get(color).map_or_else(|| hex::parse(color), Ok)?;
    let input = hex::from_rgb(rgb);
    let converted = Color::Rgb(rgb).to_color_space(out);
    let json = json::from_color(converted);
    let converted = converted.to_string();

    show_impl(rgb, input + " ~ " + &converted, json, size)
}

fn show_impl(rgb: space::Rgb, msg: String, json: String, size: u32) -> Result<()> {
    let color = colored::Color::TrueColor {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    let mut stdout = stdout();

    if !atty::is(Stream::Stdout) {
        write!(stdout, "{}", json)?;
    } else {
        let square = make_square(size);
        writeln!(stdout, "{}\n{}", msg, square.color(color))?;
    }

    Ok(())
}

pub fn show_text(
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
    s
}
