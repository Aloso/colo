use anyhow::Result;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::cli::TextStyle;

pub(crate) fn print_text(text: &str, style: TextStyle) -> Result<()> {
    let TextStyle {
        color: (color, _),
        bg_color,
        bold,
        italic,
        underline,
        no_newline,
    } = style;

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
    if !no_newline {
        writeln!(stdout)?;
    }
    Ok(())
}
