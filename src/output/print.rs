use anyhow::Result;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::cli::print::Print;

pub fn print(
    Print {
        color: (color, _),
        bg_color,
        mut text,
        bold,
        italic,
        underline,
        no_newline,
    }: Print,
) -> Result<()> {
    if !no_newline {
        text.push('\n');
    }

    let fg = color.to_term_color();
    let bg = bg_color.map(|(c, _)| c.to_term_color());
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
