use anyhow::Result;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::{
    color::{self, Color},
    State,
};

pub fn text_colors(state: State, colors: impl IntoIterator<Item = Color>) -> Result<()> {
    let mut stdout = stdout();

    let white = Color::white();
    let black = Color::black();

    for color in colors {
        let textcolor = color.text_color();
        let (other, other_name) = match textcolor {
            color::TextColor::Black => (black, "black"),
            color::TextColor::White => (white, "white"),
        };

        if state.color {
            let color_block = format!("  {}  ", other_name)
                .color(other.to_term_color())
                .on_color(color.to_term_color());

            writeln!(stdout, "{}", color_block)?;
        } else {
            writeln!(stdout, "{}", other_name)?;
        }
    }
    Ok(())
}
