use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::color::{Color, ColorFormat};

use super::util;

/// Returns the arguments for displaying a color
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .visible_alias("s")
        .about("Output colors")
        .version(super::APP_VERSION)
        .args(&[
            Arg::with_name("COLOR")
                .takes_value(true)
                .required(false)
                .help(super::COLOR_HELP_MESSAGE)
                .long_help(super::COLOR_HELP_LONG_MESSAGE)
                .multiple(true)
                .use_delimiter(false),
            Arg::with_name("OUTPUT_FORMAT")
                .long("out")
                .short("o")
                .takes_value(true)
                .possible_values(super::COLOR_FORMATS)
                .hide_possible_values(true)
                .case_insensitive(true)
                .help(
                    "Output format (html, hex) or color space [possible values: rgb, cmy, \
                    cmyk, hsv, hsl, lch, luv, lab, hunterlab, xyz, yxy, hex, html]",
                ),
            Arg::with_name("SIZE")
                .long("size")
                .short("s")
                .takes_value(true)
                .default_value("4")
                .help("Size of the color square in terminal rows"),
        ])
}

/// Represents the arguments for displaying a color
pub struct Show {
    pub colors: Vec<(Color, ColorFormat)>,
    pub output: ColorFormat,
    pub size: u32,
}

/// Parse a u32
fn parse_size(s: &str) -> Result<u32> {
    s.parse()
        .with_context(|| format!("The size {:?} could not be parsed", s))
}

/// Returns all the arguments relevant for displaying colors
pub fn get(matches: &ArgMatches) -> Result<Show> {
    let size = matches.value_of("SIZE").map(parse_size).unwrap_or(Ok(4))?;

    let colors = match matches.values_of("COLOR") {
        Some(values) => util::values_to_colors(values)?,
        None => vec![],
    };

    let output = util::get_color_format(&matches, "OUTPUT_FORMAT")?
        .or_else(|| {
            if colors.len() == 1 {
                Some(colors[0].1)
            } else {
                None
            }
        })
        .unwrap_or_default();

    Ok(Show {
        colors,
        output,
        size,
    })
}
