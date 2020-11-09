use std::io::{stdout, Write};

use crate::color::contrast::{contrast as calc_contrast, relative_luminance};
use anyhow::Result;
use atty::Stream;
use color_space::ToRgb;
use colored::Colorize;

use crate::cli::contrast::Contrast;

pub fn contrast(Contrast { color1, color2 }: Contrast) -> Result<()> {
    let interactive = atty::is(Stream::Stdout);
    let mut stdout = stdout();

    let rgb1 = color1.to_rgb();
    let rgb2 = color2.to_rgb();

    let r1 = relative_luminance(rgb1);
    let r2 = relative_luminance(rgb2);
    let c = calc_contrast(rgb1, rgb2);

    #[allow(illegal_floating_point_literal_pattern)]
    let level = match c {
        0.0..=3.0 => colored::Color::Red,
        3.0..=4.5 => colored::Color::Yellow,
        4.5..=6.0 => colored::Color::White,
        _ => colored::Color::Green,
    };

    if interactive {
        writeln!(
            stdout,
            " {}{}  {:.4}",
            "████".color(color1.to_term_color()),
            "████".color(color2.to_term_color()),
            c.to_string().color(level),
        )?;
        writeln!(
            stdout,
            " {}{}  {}",
            "████".color(color1.to_term_color()),
            "████".color(color2.to_term_color()),
            format!("(relative luminance: {:.3} / {:.3})", r1, r2).dimmed(),
        )?;
    } else {
        writeln!(stdout, "{}", c)?;
    }

    Ok(())
}
