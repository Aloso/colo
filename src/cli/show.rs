use anyhow::{Context, Result};
use clap::{Arg, ArgMatches};

use crate::color::{Color, ColorFormat, ColorSpace};

use super::util;

/// Returns the arguments for displaying a color
pub fn args<'a, 'b>() -> [Arg<'a, 'b>; 3] {
    [
        Arg::with_name("COLOR")
            .takes_value(true)
            .required(false)
            .help(super::COLOR_HELP_MESSAGE)
            .long_help(super::COLOR_HELP_LONG_MESSAGE)
            .multiple(true)
            .use_delimiter(false),
        Arg::with_name("OUTPUT_COLOR_SPACE")
            .long("out")
            .short("o")
            .takes_value(true)
            .possible_values(super::COLOR_SPACES)
            .hide_possible_values(true)
            .case_insensitive(true)
            .help(
                "Output color space [possible values: rgb, cmy, \
                cmyk, hsv, hsl, lch, luv, lab, hunterlab, xyz, yxy]",
            )
            .default_value("rgb"),
        Arg::with_name("SIZE")
            .long("size")
            .short("s")
            .takes_value(true)
            .default_value("4")
            .help("Size of the color square in terminal rows. Set to 0 to hide it"),
    ]
}

/// Represents the arguments for displaying a color
pub struct Show {
    pub colors: Vec<(Color, ColorFormat)>,
    pub output: ColorSpace,
    pub size: u32,
}

/// Parse a u32
fn parse_size(s: &str) -> Result<u32> {
    s.parse()
        .with_context(|| format!("The size {:?} could not be parsed", s))
}

/// Returns all the arguments relevant for displaying colors
pub fn get(matches: &ArgMatches) -> Result<Option<Show>> {
    let color_values = match matches.values_of("COLOR") {
        Some(values) => values,
        None => return Ok(None),
    };
    let colors = util::values_to_colors(color_values)?;

    let output = util::get_color_space(&matches, "OUTPUT_COLOR_SPACE")?.unwrap_or(ColorSpace::Rgb);
    let size = matches.value_of("SIZE").map(parse_size).unwrap_or(Ok(4))?;

    Ok(Some(Show {
        colors,
        output,
        size,
    }))
}
