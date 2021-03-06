use std::iter;

use anyhow::{bail, Context, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use super::{util, Cmd};
use crate::{
    color::{Color, ColorFormat},
    terminal::{self, stdin},
    State,
};

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
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>
";

/// The `print` subcommand
pub(crate) struct Print {
    text: String,
    style: TextStyle,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct TextStyle {
    pub color: (Color, ColorFormat),
    pub bg_color: Option<(Color, ColorFormat)>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub no_newline: bool,
}

impl Cmd for Print {
    fn command<'a, 'b>(state: State) -> App<'a, 'b> {
        SubCommand::with_name("print")
            .about("Print formatted text")
            .args(&[
                Arg::with_name("text")
                    .index(1)
                    .help(TEXT_HELP)
                    .required(state.interactive),
                Arg::with_name("colors")
                    .index(2)
                    .help(COLOR_HELP)
                    .multiple(true)
                    .use_delimiter(false)
                    .required(state.interactive),
                Arg::with_name("bold")
                    .long("bold")
                    .short("b")
                    .help("Print text in bold"),
                Arg::with_name("italic")
                    .long("italic")
                    .alias("oblique")
                    .short("i")
                    .help("Print text in italic"),
                Arg::with_name("underline")
                    .long("underline")
                    .short("u")
                    .help("Print text underlined"),
                Arg::with_name("no-newline")
                    .long("no-newline")
                    .alias("no_newline")
                    .short("n")
                    .help("Don't add new-line after text"),
            ])
    }

    fn parse(matches: &ArgMatches, &mut state: &mut State) -> Result<Self> {
        let (colors, text) = if state.interactive {
            let text = matches
                .value_of("text")
                .expect("text not present")
                .to_string();

            let color_matches = matches.values_of("colors").expect("color not present");
            let colors = util::values_to_colors(color_matches, state)?;
            (colors, text)
        } else {
            let text = stdin::read_all()?;

            // in non-interactive mode, the TEXT argument is actually the first color.
            let arg1 = matches.value_of("text").context(
                "The following required arguments were not provided:\
                \n    <COLOR>...\n\n\
                USAGE:\
                \n    echo <TEXT> | colo print [FLAGS] <COLOR>...\n\n\
                For more information try --help",
            )?;
            let remaining_args = matches.values_of("colors").unwrap_or_default();
            let colors = util::values_to_colors(iter::once(arg1).chain(remaining_args), state)?;

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

        let bold = matches.is_present("bold");
        let italic = matches.is_present("italic");
        let underline = matches.is_present("underline");
        let no_newline = matches.is_present("no-newline");

        let style = TextStyle {
            color,
            bg_color,
            bold,
            italic,
            underline,
            no_newline,
        };
        Ok(Print { text, style })
    }

    fn run(&self, _state: State) -> Result<()> {
        terminal::print_text(&self.text, self.style)
    }
}
