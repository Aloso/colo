use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::iter;

use super::util;
use crate::color::{Color, ColorFormat};
use crate::State;

const COLOR_HELP_MESSAGE: &str = "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue FF7700 | colo textcolor";

/// Returns the `list` subcommand
pub fn command<'a, 'b>(state: State) -> App<'a, 'b> {
    SubCommand::with_name("textcolor")
        .alias("textcolour")
        .about("Get a readable text color for a given background color")
        .long_about(
            "Return a readable text color (black or white) for each given background color. \
            This can also be used in the opposite way, i.e. to create a background color \
            for a given text color.",
        )
        .arg(
            Arg::with_name("colors")
                .help(COLOR_HELP_MESSAGE)
                .index(1)
                .multiple(true)
                .required(state.interactive),
        )
}

/// The struct representing the `list` subcommand
pub struct TextColor {
    pub colors: Vec<(Color, ColorFormat)>,
}

/// Return the input for the `libs` subcommand
pub fn get(matches: &ArgMatches, state: State) -> Result<TextColor> {
    let mut colors = match matches.values_of("colors") {
        Some(values) => util::values_to_colors(values)?,
        None => vec![],
    };

    if !state.interactive && colors.is_empty() {
        let text = util::read_stdin()?;
        colors = util::values_to_colors(iter::once(text.as_str()))?;
    }

    Ok(TextColor { colors })
}
