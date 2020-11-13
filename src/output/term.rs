use anyhow::Result;
use colored::{
    Color::{Black, BrightBlack, BrightWhite, White},
    Colorize,
};
use std::io::{stdout, Stdout, Write};

use crate::cli::term::Term;

pub fn term(_: Term) -> Result<()> {
    let mut stdout = stdout();

    let colors = &[
        BrightWhite,
        White,
        BrightBlack,
        Black,
        colored::Color::BrightRed,
        colored::Color::Red,
        colored::Color::BrightYellow,
        colored::Color::Yellow,
        colored::Color::BrightGreen,
        colored::Color::Green,
        colored::Color::BrightCyan,
        colored::Color::Cyan,
        colored::Color::BrightBlue,
        colored::Color::Blue,
        colored::Color::BrightMagenta,
        colored::Color::Magenta,
    ];

    writeln!(
        stdout,
        "The appearance of these colors depends on your terminal.\n"
    )?;

    for (i, line) in colors.chunks(2).enumerate() {
        write!(stdout, "   ")?;
        if let [c1, c2] = *line {
            let tc1 = if i == 1 { BrightWhite } else { Black };
            let tc2 = if i == 0 { Black } else { BrightWhite };
            print_color_block(&mut stdout, c1, 17, tc1)?;
            print_color_block(&mut stdout, c2, 11, tc2)?;

            write!(stdout, "  ")?;

            print_color_text(&mut stdout, c1, 16)?;
            print_color_text(&mut stdout, c2, 10)?;

            writeln!(stdout)?;
        }
    }

    writeln!(stdout)?;

    Ok(())
}

fn print_color_block(
    stdout: &mut Stdout,
    color: colored::Color,
    max_len: usize,
    text_color: colored::Color,
) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let spaces = &"               "[0..len];

    write!(
        stdout,
        "{}",
        (text + spaces).color(text_color).on_color(color),
    )?;
    Ok(())
}

fn print_color_text(stdout: &mut Stdout, color: colored::Color, max_len: usize) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let space = &"               "[0..len];

    write!(stdout, "{}{}", text.color(color), space)?;
    Ok(())
}
