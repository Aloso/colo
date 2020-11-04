use anyhow::Result;
use atty::Stream;
use color_space::Rgb;
use colored::{Color::TrueColor, Colorize};
use std::io::{stdout, Write};

use crate::cli::list::List;
use crate::color::{html::HTML_COLOR_NAMES, space, Color};

const WHITE: colored::Color = TrueColor {
    r: 255,
    g: 255,
    b: 255,
};
const BLACK: colored::Color = TrueColor { r: 0, g: 0, b: 0 };

pub fn list(_: List) -> Result<()> {
    let mut stdout = stdout();

    let mut even = false;

    let is_atty = atty::is(Stream::Stdout);

    for &(name, color) in HTML_COLOR_NAMES {
        if name == "magenta" || name == "aqua" || name.ends_with("grey") {
            continue;
        }

        if !is_atty {
            writeln!(stdout, "{}", name)?;
            continue;
        }

        let rgb = Rgb::from_hex(color);
        let lab: space::Lab = rgb.into();

        let color = Color::Rgb(rgb).to_term_color();

        let lab_distance = (lab.a.abs().powi(2) + lab.b.abs().powi(2)).sqrt();
        let colorfulness = lab_distance.min(100.0) / 12.0; // empirically determined values
        let text_color = if lab.l < (60.0 - colorfulness) {
            WHITE
        } else {
            BLACK
        };
        let name = format!(" {}{}", name, &"                      "[name.len()..]);
        write!(stdout, "{}", name.color(text_color).on_color(color))?;
        if even {
            writeln!(stdout)?;
        }
        even = !even;
    }
    writeln!(stdout)?;

    Ok(())
}
