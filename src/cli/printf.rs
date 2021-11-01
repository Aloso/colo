use anyhow::Result;
use clap::{App, Arg, ArgMatches, SubCommand};

use super::{util, Cmd};
use crate::{
    color::{Color, ColorFormat},
    terminal, State,
};

const TEXT_HELP: &str = r#"Format string. May contain the following formatting markers:

%B  -- make the text bold
%I  -- make the text italic
%U  -- make the text underlined
%F  -- make the text faint
%S  -- make the text strikethrough
%X  -- make the text inverted
%c  -- set text color
%b  -- set background color
%r  -- reset all styles
\%  -- escaped percent sign

\a, \b, \e, \f, \n, \r, \t, \v, \\
    -- escaped characters (same as in C's printf function)

EXAMPLE:
    colo printf "%B%cHello world!%r\n" red


"#;

const COLOR_HELP: &str = "\
Input colors. You must specify one color for each occurrence of %c and %b. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>
";

/// The `printf` subcommand
pub(crate) struct Printf {
    text: String,
    colors: Vec<(Color, ColorFormat)>,
}

impl Cmd for Printf {
    fn command<'a, 'b>(state: State) -> App<'a, 'b> {
        SubCommand::with_name("printf")
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
                    .required(false),
            ])
    }

    fn parse(matches: &ArgMatches, &mut state: &mut State) -> Result<Self> {
        let text = matches
            .value_of("text")
            .expect("text not present")
            .to_string();

        let color_matches = matches.values_of("colors").unwrap_or_default();
        let colors = util::values_to_colors(color_matches, state)?;

        Ok(Printf { text, colors })
    }

    fn run(&self, _state: State) -> Result<()> {
        terminal::print_text(&self.text, &self.colors)
    }
}
