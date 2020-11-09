use std::iter;

use anyhow::{bail, Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use super::util;
use crate::color::{Color, ColorFormat};

const TEXT_HELP: &str = "\
Text to print in the specified color. If colo is used behind a pipe \
or outside of the terminal, the text must be passed via stdin instead, e.g.

$ echo Hello world! | colo print orange
 ";

const COLOR_HELP: &str = "\
The input colors. You can specify up to 2 colors (text and background color). Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see
  <https://aloso.github.io/colo/color_spaces>
";

/// Returns the `print` subcommand
pub fn command<'a, 'b>(interactive: bool) -> App<'a, 'b> {
    SubCommand::with_name("print")
        .about("Print formatted text")
        .version(super::APP_VERSION)
        .usage(
            "colo print [FLAGS] <TEXT> <COLOR>...\
            \n    echo <TEXT> | colo print [FLAGS] <COLOR>...",
        )
        .args(&[
            Arg::with_name("TEXT")
                .index(1)
                .help(TEXT_HELP)
                .required(interactive),
            Arg::with_name("COLOR")
                .index(2)
                .help(COLOR_HELP)
                .multiple(true)
                .use_delimiter(false)
                .required(interactive),
            Arg::with_name("BOLD")
                .long("bold")
                .short("b")
                .help("Print text in bold"),
            Arg::with_name("ITALIC")
                .long("italic")
                .alias("oblique")
                .short("i")
                .help("Print text in italic"),
            Arg::with_name("UNDERLINE")
                .long("underline")
                .short("u")
                .help("Print text underlined"),
            Arg::with_name("NO_NEWLINE")
                .long("no-newline")
                .alias("no_newline")
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
pub fn get(matches: &ArgMatches, interactive: bool) -> Result<Print> {
    let (colors, text) = if interactive {
        let text = matches
            .value_of("TEXT")
            .expect("text not present")
            .to_string();

        let color_matches = matches.values_of("COLOR").expect("color not present");
        let colors = util::values_to_colors(color_matches)?;
        (colors, text)
    } else {
        let text = util::read_stdin()?;

        // in non-interactive mode, the TEXT argument is actually the first color.
        let arg1 = matches.value_of("TEXT").context(
            "The following required arguments were not provided:\
            \n    <COLOR>...\n\n\
            USAGE:\
            \n    echo <TEXT> | colo print [FLAGS] <COLOR>...\n\n\
            For more information try --help",
        )?;
        let remaining_args = matches.values_of("COLOR").unwrap_or_default();
        let colors = util::values_to_colors(iter::once(arg1).chain(remaining_args))?;

        (colors, text)
    };

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
