use anyhow::{bail, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use super::util;
use crate::color::{Color, ColorFormat};

const COLOR_HELP: &str = "\
The input colors. Multiple colors can be separated with a comma. Supported formats:

* HTML color name (e.g. 'rebeccapurple')
* Hexadecimal RGB color (e.g. '07F', '0077FF')
* Color components (e.g. '127, 0, 255', 'hsl(30, 1, .5)').
  If no color space is specified, it defaults to 'rgb'.
  Commas and parentheses are optional.
";

/// Returns the `print` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("print")
        .about("Print formatted text")
        .version(super::APP_VERSION)
        .args(&[
            Arg::with_name("COLOR")
                .index(2)
                .help(COLOR_HELP)
                .multiple(true)
                .use_delimiter(false)
                .required(true),
            Arg::with_name("TEXT")
                .index(1)
                .help("Text to print in the specified color")
                .required(true),
            Arg::with_name("BOLD")
                .long("bold")
                .short("b")
                .help("Print text in bold"),
            Arg::with_name("ITALIC")
                .long("italic")
                .short("i")
                .help("Print text in italic"),
            Arg::with_name("UNDERLINE")
                .long("underline")
                .short("u")
                .help("Print text underlined"),
            Arg::with_name("NO_NEWLINE")
                .long("no_newline")
                .short("n")
                .help("Don't add new-line after text"),
        ])
}

/// The struct representing the `print` subcommand
pub struct Print {
    pub color: (Color, ColorFormat),
    pub bg_color: Option<(Color, ColorFormat)>,
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub no_newline: bool,
}

/// Returns the input for the `print` subcommand
pub fn get(matches: &ArgMatches) -> Result<Print> {
    let text = matches
        .value_of("TEXT")
        .expect("text not present")
        .to_string();

    let color_matches = matches.values_of("COLOR").expect("color not present");
    let colors = util::values_to_colors(color_matches)?;

    if colors.len() > 2 {
        bail!("At most two colors (text and background color) can be specified.")
    }
    if colors.is_empty() {
        bail!("At least one color must be specified.")
    }
    let color = colors[0];
    let bg_color = colors.get(1).copied();

    let bold = matches.is_present("BOLD");
    let italic = matches.is_present("ITALIC");
    let underline = matches.is_present("UNDERLINE");
    let no_newline = matches.is_present("NO_NEWLINE");

    Ok(Print {
        color,
        bg_color,
        text,
        bold,
        italic,
        underline,
        no_newline,
    })
}
