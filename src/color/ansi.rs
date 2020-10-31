use std::fmt;

/// An enum containing an ANSI 8-bit color. These particular colors are
/// supported by the vast majority of terminals.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AnsiColor {
    White,
    Grey,
    DarkGrey,
    Black,
    Red,
    DarkRed,
    Yellow,
    DarkYellow,
    Green,
    DarkGreen,
    Cyan,
    DarkCyan,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
}

impl AnsiColor {
    fn ansi_256_code(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            AnsiColor::White => "5;15m",
            AnsiColor::Grey => "5;7m",
            AnsiColor::DarkGrey => "5;8m",
            AnsiColor::Black => "5;0m",
            AnsiColor::Red => "5;9m",
            AnsiColor::DarkRed => "5;1m",
            AnsiColor::Yellow => "5;11m",
            AnsiColor::DarkYellow => "5;3m",
            AnsiColor::Green => "5;10m",
            AnsiColor::DarkGreen => "5;2m",
            AnsiColor::Cyan => "5;14m",
            AnsiColor::DarkCyan => "5;6m",
            AnsiColor::Blue => "5;12m",
            AnsiColor::DarkBlue => "5;4m",
            AnsiColor::Magenta => "5;13m",
            AnsiColor::DarkMagenta => "5;5m",
        })
    }
}

/// Formats the contained color as background color.
pub struct Bg(pub AnsiColor);

/// Formats the contained color as foreground color.
pub struct Fg(pub AnsiColor);

/// Resets the background color
pub struct ResetBg;

/// Resets the foreground color
pub struct ResetFg;

impl fmt::Display for Bg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1B[48;")?;
        self.0.ansi_256_code(f)
    }
}

impl fmt::Display for Fg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1B[38;")?;
        self.0.ansi_256_code(f)
    }
}

impl fmt::Display for ResetBg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1B[49m")
    }
}

impl fmt::Display for ResetFg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1B[39m")
    }
}
