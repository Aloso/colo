use crossterm::style::{
    self, Colorize, Print, PrintStyledContent, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::{cursor, queue, Result};
use std::io::{stdout, Stdout, Write};

pub fn show_term_colors() -> Result<()> {
    let mut stdout = stdout();

    let colors = &[
        style::Color::White,
        style::Color::DarkGrey,
        style::Color::Grey,
        style::Color::Black,
        style::Color::Red,
        style::Color::DarkRed,
        style::Color::Yellow,
        style::Color::DarkYellow,
        style::Color::Green,
        style::Color::DarkGreen,
        style::Color::Cyan,
        style::Color::DarkCyan,
        style::Color::Blue,
        style::Color::DarkBlue,
        style::Color::Magenta,
        style::Color::DarkMagenta,
    ];

    queue!(
        stdout,
        Print("The appearance of these colors depends on your terminal.\n\n"),
    )?;

    for (_i, line) in colors.chunks(2).enumerate() {
        queue!(stdout, cursor::MoveRight(3))?;
        if let [c1, c2] = *line {
            print_color_block(&mut stdout, c1, 11, false)?;
            print_color_block(&mut stdout, c2, 15, true)?;

            queue!(stdout, Print("    "))?;

            print_color_text(&mut stdout, c1, 10)?;
            print_color_text(&mut stdout, c2, 14)?;

            queue!(stdout, Print("\n"))?;
        }
    }

    queue!(stdout, Print("\n"))?;
    stdout.flush().unwrap();

    Ok(())
}

fn print_color_block(
    stdout: &mut Stdout,
    color: style::Color,
    max_len: usize,
    white: bool,
) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let text = if white { text.white() } else { text.black() };
    let space = &"               "[0..len];

    queue!(
        stdout,
        SetBackgroundColor(color),
        PrintStyledContent(text),
        Print(space),
        ResetColor,
    )
}

fn print_color_text(stdout: &mut Stdout, color: style::Color, max_len: usize) -> Result<()> {
    let text = format!("  {:?}", color);
    let len = max_len - text.len();
    let space = &"               "[0..len];

    queue!(
        stdout,
        SetForegroundColor(color),
        Print(text),
        Print(space),
        ResetColor,
    )
}
