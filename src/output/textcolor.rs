use std::io::{stdout, Write};

use crate::color::{contrast::contrast, space::Rgb, Color};
use anyhow::Result;
use atty::Stream;
use color_space::ToRgb;
use colored::Colorize;

use crate::cli::textcolor::TextColor;

pub fn textcolor(TextColor { colors }: TextColor) -> Result<()> {
    let mut stdout = stdout();

    for (color, _) in colors {
        let interactive = atty::is(Stream::Stdout);

        let rgb = color.to_rgb();
        let black = Rgb::new(0.0, 0.0, 0.0);
        let white = Rgb::new(255.0, 255.0, 255.0);
        let bc = contrast(rgb, black);
        let wc = contrast(rgb, white);

        let other_color_name = if wc >= bc { "white" } else { "black" };

        if interactive {
            let other_color = Color::Rgb(match other_color_name {
                "white" => white,
                _ => black,
            });

            let color_block = format!("  {}  ", other_color_name)
                .color(other_color.to_term_color())
                .on_color(color.to_term_color());

            writeln!(stdout, "{}", color_block)?;
        } else {
            writeln!(stdout, "{}", other_color_name)?;
        }
    }
    Ok(())
}
