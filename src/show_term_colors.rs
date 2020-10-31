use crate::color::ansi::{AnsiColor, Bg, Fg, ResetBg, ResetFg};
use anyhow::Result;
use std::io::{stdout, Stdout, Write};

pub fn show_term_colors() -> Result<()> {
    let mut stdout = stdout();

    let colors = &[
        AnsiColor::White,
        AnsiColor::DarkGrey,
        AnsiColor::Grey,
        AnsiColor::Black,
        AnsiColor::Red,
        AnsiColor::DarkRed,
        AnsiColor::Yellow,
        AnsiColor::DarkYellow,
        AnsiColor::Green,
        AnsiColor::DarkGreen,
        AnsiColor::Cyan,
        AnsiColor::DarkCyan,
        AnsiColor::Blue,
        AnsiColor::DarkBlue,
        AnsiColor::Magenta,
        AnsiColor::DarkMagenta,
    ];

    writeln!(
        stdout,
        "The appearance of these colors depends on your terminal.\n"
    )?;

    for (_i, line) in colors.chunks(2).enumerate() {
        write!(stdout, "   ")?;
        if let [c1, c2] = *line {
            print_color_block(&mut stdout, c1, 11, AnsiColor::Black)?;
            print_color_block(&mut stdout, c2, 15, AnsiColor::White)?;

            write!(stdout, "    ")?;

            print_color_text(&mut stdout, c1, 10)?;
            print_color_text(&mut stdout, c2, 14)?;

            writeln!(stdout)?;
        }
    }

    writeln!(stdout)?;

    Ok(())
}

fn print_color_block(
    stdout: &mut Stdout,
    color: AnsiColor,
    max_len: usize,
    text_color: AnsiColor,
) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let spaces = &"               "[0..len];

    write!(
        stdout,
        "{}{}{}{}{}{}",
        Bg(color),
        Fg(text_color),
        text,
        spaces,
        ResetFg,
        ResetBg,
    )?;
    Ok(())
}

fn print_color_text(stdout: &mut Stdout, color: AnsiColor, max_len: usize) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let space = &"               "[0..len];

    write!(
        stdout,
        "{}{}{}{}{}",
        Fg(color),
        text,
        space,
        ResetFg,
        ResetBg,
    )?;
    Ok(())
}
