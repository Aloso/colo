mod list;
mod picker;
mod print;
mod show;
pub(crate) mod stdin;
mod term;
mod textcolor;

use anyhow::Result;
use colored::{ColoredString, Colorize};
use std::io::{self, Write};

use crate::{color::Color, State};

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
