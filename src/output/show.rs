use anyhow::{Context, Result};
use atty::Stream;
use color_space::ToRgb;
use colored::{ColoredString, Colorize};
use std::io::{stdout, Stdout, Write};
use std::iter;

use crate::cli::show::Show;
use crate::color::{format, Color, ColorFormat};

pub fn show(
    Show {
        colors,
        output,
        size,
    }: Show,
) -> Result<()> {
    let interactive = atty::is(Stream::Stdout);
    let mut stdout = stdout();

    if interactive && size > 0 {
        writeln!(stdout)?;
    }
    for (color, input) in colors {
        show_color(interactive, &mut stdout, color, input, output, size)?;
    }
    Ok(())
}

/// Print a colored square
fn show_color(
    interactive: bool,
    stdout: &mut Stdout,
    color: Color,
    _input: ColorFormat,
    output: ColorFormat,
    size: u32,
) -> Result<()> {
    let rgb = color.to_rgb();
    let term_color = colored::Color::TrueColor {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    if !interactive {
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
        if size > 0 {
            writeln!(stdout)?;
        }
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
        if size > 0 {
            writeln!(stdout)?;
        }
    }

    Ok(())
}

/// Prints the color square and the color formats on its right
fn print_color<I, J>(
    stdout: &mut Stdout,
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
        // Print one line of the square
        write!(stdout, "{}", line.color(term_color))?;

        // Print one line of the color formats
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

/// Generates an ASCII square with the given size
fn make_square(size: u32) -> String {
    let mut s = String::new();
    for _ in 0..size {
        s.push(' ');
        for _ in 0..size {
            s.push('█');
            s.push('█');
        }
        s.push('\n');
    }
    s
}

/// Generates a tiny ASCII rectangle that fits in one line
fn make_tiny_square() -> &'static str {
    "██"
}
