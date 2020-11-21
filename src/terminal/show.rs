use anyhow::Result;
use color_space::ToRgb;
use colored::{ColoredString, Colorize};
use std::io::{stdout, Stdout, Write};
use std::iter;

use crate::{
    color::{format, Color, ColorFormat},
    State,
};

pub fn show_colors(
    state: State,
    colors: impl IntoIterator<Item = Color>,
    output: ColorFormat,
    square_size: u32,
) -> Result<()> {
    let mut stdout = stdout();

    if state.color && square_size > 0 {
        writeln!(stdout)?;
    }
    for color in colors {
        show_color(state, &mut stdout, color, output, square_size)?;
    }
    Ok(())
}

/// Print a colored square
fn show_color(
    state: State,
    stdout: &mut Stdout,
    color: Color,
    output: ColorFormat,
    square_size: u32,
) -> Result<()> {
    let color = match output {
        ColorFormat::Normal(s) => color.to_color_space(s),
        ColorFormat::Hex | ColorFormat::Html => color,
    };
    let rgb = color.to_rgb();

    let term_color = colored::Color::TrueColor {
        r: rgb.r.round() as u8,
        g: rgb.g.round() as u8,
        b: rgb.b.round() as u8,
    };

    if !state.color {
        let color = output.format_or_hex(color);

        writeln!(stdout, "{}", color)?;
    } else if square_size >= 1 {
        let formats = format::PREFERRED_FORMATS
            .iter()
            .take(square_size as usize)
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

        print_color(stdout, term_color, &make_square(square_size), formats, true)?;
        if square_size > 0 {
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
        if square_size > 0 {
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
