use std::iter;

use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use crate::color::{Color, ColorFormat};
use crate::State;

use super::util;

const COLOR_HELP_MESSAGE: &str = "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue FF7700 | colo show";

/// Returns the arguments for displaying a color
pub fn command<'a, 'b>(state: State) -> App<'a, 'b> {
    SubCommand::with_name("show")
        .visible_alias("s")
        .about("Output colors")
        .args(&[
            Arg::with_name("colors")
                .takes_value(true)
                .required(state.interactive)
                .help(COLOR_HELP_MESSAGE)
                .multiple(true)
                .use_delimiter(false),
            Arg::with_name("output-format")
                .long("out")
                .short("o")
                .takes_value(true)
                .possible_values(super::COLOR_FORMATS)
                .hide_possible_values(true)
                .case_insensitive(true)
                .help(
                    "Output format (html, hex, or color space) [possible values: rgb, cmy, \
                    cmyk, hsv, hsl, lch, luv, lab, hunterlab, xyz, yxy, hex, html]",
                ),
            Arg::with_name("size")
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
pub fn get(matches: &ArgMatches, state: State) -> Result<Show> {
    let size = matches.value_of("size").map(parse_size).unwrap_or(Ok(4))?;

    let mut colors = match matches.values_of("colors") {
        Some(values) => util::values_to_colors(values)?,
        None => vec![],
    };

    if !state.interactive && colors.is_empty() {
        let text = util::read_stdin()?;
        colors = util::values_to_colors(iter::once(text.as_str()))?;
    }

    let output = util::get_color_format(&matches, "output-format")?
        .or_else(|| {
            if colors.is_empty() {
                None
            } else if colors.windows(2).all(|c| c[0].1 == c[1].1) {
                Some(colors[0].1).filter(|&c| c != ColorFormat::Html)
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
