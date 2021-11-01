use anyhow::{bail, Result};
use clap::{App, Arg, ArgMatches, SubCommand};

use super::{show::Show, util, Cmd};
use crate::{
    color::{Color, ColorFormat, ColorSpace},
    terminal::ColorPicker,
    State,
};

const COLOR_HELP_MESSAGE: &str = "\
The initial color of the color picker. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>";

/// The `pick` subcommand
pub struct Pick(pub Show);

impl Cmd for Pick {
    fn command<'a, 'b>(_state: State) -> App<'a, 'b> {
        SubCommand::with_name("pick")
            .about("Terminal color picker")
            .args(&[
                Arg::with_name("output-format")
                    .long("out")
                    .short("o")
                    .takes_value(true)
                    .possible_values(super::COLOR_FORMATS)
                    .case_insensitive(true)
                    .help("Output format (html, hex, or color space)"),
                Arg::with_name("size")
                    .long("size")
                    .short("s")
                    .takes_value(true)
                    .default_value("4")
                    .help("Size of the color square in terminal rows"),
                Arg::with_name("color-space")
                    .index(1)
                    .takes_value(true)
                    .possible_values(&["rgb", "hsl", "hsv", "lab", "cmy"])
                    .case_insensitive(true)
                    .help("Initial color space of the color picker"),
                Arg::with_name("color")
                    .long("color")
                    .short("c")
                    .takes_value(true)
                    .multiple(true)
                    .use_delimiter(false)
                    .help(COLOR_HELP_MESSAGE),
            ])
    }

    fn parse(matches: &ArgMatches, &mut state: &mut State) -> Result<Self> {
        let size = matches
            .value_of("size")
            .map(util::parse_size)
            .unwrap_or(Ok(4))?;
        let output = util::get_color_format(matches, "output-format")?.unwrap_or_default();

        let (color, cs) = get_color_options(matches, state)?;
        let cs = get_color_space_option(matches).or(cs);

        let color = ColorPicker::new(color, cs).display(state)?;

        let show = Show {
            colors: vec![(color, color.get_color_format())],
            output,
            size,
        };
        Ok(Pick(show))
    }

    fn run(&self, state: State) -> Result<()> {
        self.0.run(state)
    }
}

fn get_color_options(
    matches: &ArgMatches<'_>,
    state: State,
) -> Result<(Option<Color>, Option<ColorSpace>)> {
    let values = matches
        .values_of("color")
        .map(|values| {
            let colors = util::values_to_colors(values, state)?;

            if colors.len() > 1 {
                bail!("Only one color can be specified, found {}", colors.len());
            }

            Ok(colors.get(0).map(|&(c, f)| {
                let cs = match f {
                    ColorFormat::Normal(cs) => cs,
                    ColorFormat::Hex | ColorFormat::Html => ColorSpace::Rgb,
                };
                (c, cs)
            }))
        })
        .transpose()?
        .flatten();

    Ok((values.map(|(c, _)| c), values.map(|(_, c)| c)))
}

fn get_color_space_option(matches: &ArgMatches<'_>) -> Option<ColorSpace> {
    matches
        .value_of("color-space")
        .map(|cs| match cs.to_lowercase().as_str() {
            "rgb" => ColorSpace::Rgb,
            "hsl" => ColorSpace::Hsl,
            "hsv" => ColorSpace::Hsv,
            "lab" => ColorSpace::Lab,
            "cmy" => ColorSpace::Cmy,
            _ => panic!("Invalid color space {:?}", cs),
        })
}
