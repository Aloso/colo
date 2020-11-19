use std::iter;

use anyhow::{bail, Result};
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::Colorize;

use super::{util, Cmd};
use crate::{
    color::{contrast, space::Rgb, Color, ColorFormat},
    terminal::{compare_colors, stdin},
    State,
};

const COLOR_HELP: &str = "\
At most 2 colors. If only one color is provided, the other color defaults to white. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue | colo contrast
";

/// The `contrast` subcommand
pub struct Contrast {
    pub color1: Color,
    pub color2: Color,
}

impl Cmd for Contrast {
    fn command<'a, 'b>(state: State) -> App<'a, 'b> {
        SubCommand::with_name("contrast")
            .about("Get the contrast between two colors")
            .long_about(
                "Return the contrast between two colors according to the W3 specification. \
            The contrast is a value between 1 and 21. \
            Text contrast should always be at least 4.5, or 3 for large text.",
            )
            .arg(
                Arg::with_name("colors")
                    .help(COLOR_HELP)
                    .index(1)
                    .multiple(true)
                    .required(state.interactive),
            )
    }

    fn parse(matches: &ArgMatches, &mut state: &mut State) -> Result<Self> {
        let mut colors = match matches.values_of("colors") {
            Some(values) => util::values_to_colors(values, state)?,
            None => vec![],
        };

        if !state.interactive && colors.is_empty() {
            let text = stdin::read_all()?;
            colors = util::values_to_colors(iter::once(text.as_str()), state)?;
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

    fn run(&self, state: State) -> Result<()> {
        let lum1 = self.color1.relative_luminance();
        let lum2 = self.color2.relative_luminance();
        let contrast = contrast(lum1, lum2);

        let level = match contrast {
            _ if contrast < 3.0 => colored::Color::Red,
            _ if contrast < 4.5 => colored::Color::Yellow,
            _ if contrast < 6.0 => colored::Color::BrightWhite,
            _ => colored::Color::Green,
        };

        compare_colors(
            state,
            self.color1,
            self.color2,
            format!("{:.2}", contrast).color(level),
            &format!("(relative luminance: {:.3} / {:.3})", lum1, lum2),
        )
    }
}
