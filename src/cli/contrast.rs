use std::iter;

use anyhow::{bail, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use super::util;
use crate::color::{space::Rgb, Color, ColorFormat};

/// Returns the `list` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("contrast")
        .about(
            "Return the contrast between two colors according to the W3 specification. \
            If only one color is provided, the other color defaults to white. \
            The contrast is a value between 1 and 21. \
            Text contrast should always be at least 4.5, or 3 for large text.",
        )
        .version(super::APP_VERSION)
        .arg(
            Arg::with_name("COLORS")
                .help("TODO!")
                .index(1)
                .multiple(true),
        )
}

/// The struct representing the `list` subcommand
pub struct Contrast {
    pub color1: Color,
    pub color2: Color,
}

/// Return the input for the `libs` subcommand
pub fn get(matches: &ArgMatches, interactive: bool) -> Result<Contrast> {
    let mut colors = match matches.values_of("COLORS") {
        Some(values) => util::values_to_colors(values)?,
        None => vec![],
    };

    if !interactive && colors.is_empty() {
        let text = util::read_stdin()?;
        colors = util::values_to_colors(iter::once(text.as_str()))?;
    }
    if colors.len() == 1 {
        let white = Color::Rgb(Rgb::new(255.0, 255.0, 255.0));
        colors.push((white, ColorFormat::Html));
    }
    if colors.len() != 2 {
        bail!("Expected 2 colors, got {}", colors.len());
    }

    Ok(Contrast {
        color1: colors[0].0,
        color2: colors[1].0,
    })
}
