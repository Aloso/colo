mod list;
mod picker;
mod print;
mod show;
pub(crate) mod stdin;
mod term;
mod textcolor;

use anyhow::Result;
use colored::{ColoredString, Colorize};
use std::{
    io::{self, Write},
    iter,
};

use crate::{
    color::{Color, ColorFormat},
    State,
};

pub(crate) use list::list;
pub(crate) use picker::ColorPicker;
pub(crate) use print::print_text;
pub(crate) use show::show_colors;
pub(crate) use term::term_colors;
pub(crate) use textcolor::text_colors;

pub(crate) fn compare_colors(
    state: State,
    color1: Color,
    color2: Color,
    line1: ColoredString,
    line2: &str,
) -> Result<()> {
    let mut stdout = io::stdout();

    if state.color {
        let color1 = color1.to_term_color();
        let color2 = color2.to_term_color();

        writeln!(
            stdout,
            " {}{}  {}",
            "████".color(color1),
            "████".color(color2),
            line1,
        )?;
        writeln!(
            stdout,
            " {}{}  {}",
            "████".color(color1),
            "████".color(color2),
            line2.dimmed(),
        )?;
    } else {
        writeln!(stdout, "{}", line1)?;
    }

    Ok(())
}

pub(crate) fn list_small(
    state: State,
    title: Option<&str>,
    colors: impl IntoIterator<Item = (Color, ColorFormat)>,
    color_width: usize,
) -> Result<()> {
    let mut stdout = io::stdout();

    if state.color {
        let colors = colors.into_iter().map(|(color, _)| color);

        if let Some(title) = title {
            write!(stdout, "{}: ", title)?;
        }
        if color_width >= 2 {
            let s: String = iter::once(' ').cycle().take(color_width / 2).collect();
            for c in colors {
                write!(stdout, "{}", s.on_color(c.to_term_color()))?;
            }
        } else {
            let mut it = colors.peekable();

            while let Some(c1) = it.next() {
                let s = "▌".color(c1.to_term_color());
                if let Some(&c2) = it.peek() {
                    write!(stdout, "{}", s.on_color(c2.to_term_color()))?;
                    it.next().unwrap();
                } else {
                    write!(stdout, "{}", s)?;
                }
            }
        }
        writeln!(stdout)?;
    } else {
        for (color, format) in colors {
            writeln!(stdout, "{}", format.format_or_hex(color))?;
        }
    }
    Ok(())
}
