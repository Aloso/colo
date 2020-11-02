use anyhow::{Context, Result};
use atty::Stream;
use color_space::ToRgb;
use colored::{ColoredString, Colorize};
use std::{
    io::{stdout, Stdout, Write},
    iter,
};

use crate::color::{format, space, Color, ColorFormat};

/// Print a colored square
pub fn show(color: Color, _input: ColorFormat, output: ColorFormat, size: u32) -> Result<()> {
    let rgb = color.to_rgb();
    let term_color = colored::Color::TrueColor {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    let mut stdout = stdout();

    if !atty::is(Stream::Stdout) {
        let color = output
            .format(color)
            .or_else(|| ColorFormat::Hex.format(color))
            .with_context(|| format!("Color could not be formatted as {:?}", output))?;

        writeln!(stdout, "{}", color)?;
    } else if size >= 1 {
        let formats = format::PREFERRED_FORMATS
            .iter()
            .take(size as usize)
            .map(|&c| {
                c.iter().copied().flat_map(|c| {
                    Some((
                        c,
                        if c != output {
                            c.format(color).map(|s| s.dimmed())?
                        } else {
                            c.format(color).map(|s| s.bold())?
                        },
                    ))
                })
            })
            .map(Some)
            .chain(iter::once(None).cycle());

        print_color(stdout, term_color, &make_square(size), formats, true)?;
    } else {
        let formats = iter::once(output)
            .chain(
                format::PREFERRED_FORMATS_SHORT
                    .iter()
                    .copied()
                    .filter(|&f| f != output),
            )
            .filter_map(|c| {
                Some((
                    c,
                    if c != output {
                        c.format(color).map(|s| s.dimmed())?
                    } else {
                        c.format(color).map(|s| s.bold())?
                    },
                ))
            })
            .take(3);

        print_color(
            stdout,
            term_color,
            make_tiny_square(),
            iter::once(Some(formats)),
            false,
        )?;
    }

    Ok(())
}

fn print_color<I, J>(
    mut stdout: Stdout,
    term_color: colored::Color,
    square: &str,
    formats: I,
    add_padding: bool,
) -> Result<()>
where
    I: Iterator<Item = Option<J>>,
    J: Iterator<Item = (ColorFormat, ColoredString)>,
{
    for (line, colors) in square.lines().zip(formats) {
        write!(stdout, "{}", line.color(term_color))?;
        if let Some(colors) = colors {
            let mut step = 0;
            for (c, col) in colors {
                match c {
                    ColorFormat::Hex => {
                        write!(stdout, "  {}", col)?;
                        step += 1;
                    }
                    ColorFormat::Html => {
                        write!(stdout, "  {:16}", col)?;
                        step += 1;
                    }
                    _ => {
                        if add_padding && step == 1 {
                            write!(stdout, "                  ")?;
                        }
                        write!(stdout, "  {:25}", col)?;
                    }
                }
            }
        }
        writeln!(stdout)?;
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

/// Generates a tiny ASCII rectangle that fits in one line
fn make_tiny_square() -> &'static str {
    "██"
}
