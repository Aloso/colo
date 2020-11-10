use anyhow::Result;
use color_space::ToRgb;
use colored::Colorize;
use std::io::{stdout, Write};

use crate::cli::textcolor::TextColor;
use crate::color::{contrast::contrast, space::Rgb, Color};
use crate::State;

pub fn textcolor(TextColor { colors }: TextColor, state: State) -> Result<()> {
    let mut stdout = stdout();

    for (color, _) in colors {
        let rgb = color.to_rgb();
        let black = Rgb::new(0.0, 0.0, 0.0);
        let white = Rgb::new(255.0, 255.0, 255.0);
        let bc = contrast(rgb, black);
        let wc = contrast(rgb, white);

        let other_color_name = if wc >= bc { "white" } else { "black" };

        if state.ansi_output {
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
