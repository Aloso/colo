use anyhow::Result;
use color_space::Rgb;
use colored::{Color::TrueColor, Colorize};
use std::io::{stdout, Write};

use crate::color::{html::HTML_COLOR_NAMES, Color};
use crate::State;
use crate::{cli::list::List, color::TextColor};

const WHITE: colored::Color = TrueColor {
    r: 255,
    g: 255,
    b: 255,
};
const BLACK: colored::Color = TrueColor { r: 0, g: 0, b: 0 };

pub fn list(_: List, state: State) -> Result<()> {
    let mut stdout = stdout();

    let mut even = false;

    for &(name, color) in HTML_COLOR_NAMES {
        if name == "magenta" || name == "aqua" || name.ends_with("grey") {
            continue;
        }

        if !state.ansi_output {
            writeln!(stdout, "{}", name)?;
            continue;
        }

        let color = Color::Rgb(Rgb::from_hex(color));
        let term_color = color.to_term_color();
        let text_color = match color.text_color() {
            TextColor::Black => BLACK,
            TextColor::White => WHITE,
        };

        let name = format!(" {}{}", name, &"                      "[name.len()..]);
        write!(stdout, "{}", name.color(text_color).on_color(term_color))?;
        if even {
            writeln!(stdout)?;
        }
        even = !even;
    }
    writeln!(stdout)?;

    Ok(())
}
