use std::iter;

use anyhow::{bail, Result};
use clap::{Arg, ArgMatches, SubCommand};
use color::ColorFormat;

use super::{util, Cmd};
use crate::{
    color::{self, Color, ColorSpace},
    terminal::{self, stdin},
    State,
};

#[derive(Debug, Clone)]
pub struct Mix {
    colors: Vec<(Color, ColorFormat, f64)>,
    color_space: ColorSpace,
    output: ColorFormat,
    size: u32,
}

const COLOR_HELP_MESSAGE: &str = "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue FF7700 | colo mix";

impl Cmd for Mix {
    fn command<'a, 'b>(state: State) -> clap::App<'a, 'b> {
        SubCommand::with_name("mix")
            .about("Mix colors in a specific color space")
            .args(&[
                Arg::with_name("colors")
                    .takes_value(true)
                    .index(1)
                    .required(state.interactive)
                    .multiple(true)
                    .use_delimiter(false)
                    .help(COLOR_HELP_MESSAGE),
                Arg::with_name("weights")
                    .long("weights")
                    .short("w")
                    .takes_value(true)
                    .min_values(1)
                    .max_values(u64::MAX)
                    .require_delimiter(true)
                    .value_delimiter(",")
                    .help(
                        "The ratio in which the colors are mixed. \
                        For example, `--weights 2,5` indicates a ratio of 2:5. \
                        The default for each color is 1.",
                    ),
                Arg::with_name("color-space")
                    .long("color-space")
                    .short("c")
                    .help(
                        "The color space which the colors are mixed in. \
                        For supported color spaces, see \
                        <https://aloso.github.io/colo/color_spaces>",
                    )
                    .possible_values(super::COLOR_SPACES)
                    .case_insensitive(true)
                    .default_value("lab"),
                Arg::with_name("size")
                    .long("size")
                    .short("s")
                    .takes_value(true)
                    .default_value("4")
                    .help("Size of the color square in terminal rows"),
                Arg::with_name("output-format")
                    .long("out")
                    .short("o")
                    .takes_value(true)
                    .possible_values(super::COLOR_FORMATS)
                    .hide_possible_values(true)
                    .case_insensitive(true)
                    .help(
                        "Output format (html, hex, or color space) [possible values: rgb, cmy, \
                        cmyk, hsv, hsl, lch, luv, lab, hunterlab, xyz, yxy, gry, hex, html]",
                    ),
            ])
    }

    fn parse(matches: &ArgMatches, &mut state: &mut State) -> Result<Self> {
        let size = matches
            .value_of("size")
            .map(util::parse_size)
            .unwrap_or(Ok(4))?;

        let color_space = matches
            .value_of("color-space")
            .unwrap()
            .to_lowercase()
            .parse()?;

        let mut colors = match matches.values_of("colors") {
            Some(values) => util::values_to_colors(values, state)?,
            None => vec![],
        };

        if !state.interactive && colors.is_empty() {
            let input = stdin::read_all()?;
            colors = color::parse(&input, state)?;
        }

        let colors = colors
            .into_iter()
            .zip(parse_weights(matches))
            .map(|((color, fmt), weight)| Ok((color, fmt, weight?)))
            .collect::<Result<Vec<(Color, ColorFormat, f64)>>>()?;

        fn parse_weights<'a>(matches: &'a ArgMatches) -> impl Iterator<Item = Result<f64>> + 'a {
            let values = matches.values_of("weights").unwrap_or_default();
            values
                .map(|s| s.parse::<f64>().map_err(From::from))
                .chain(iter::once(1.0).cycle().map(Ok))
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

        Ok(Mix {
            colors,
            color_space,
            output,
            size,
        })
    }

    fn run(&self, state: State) -> Result<()> {
        // Todo: report wrong type hint to rust-analyzer
        let weight_sum: f64 = self.colors.iter().map(|&(.., w)| w).sum();
        if weight_sum == 0.0 {
            bail!("All weights are 0");
        }

        let (_, components) = self
            .colors
            .iter()
            .map(|&(c, _, w)| (c.to_color_space(self.color_space), w))
            .try_fold(
                (self.color_space, vec![0.0, 0.0, 0.0, 0.0]),
                |left, (right, w)| add_color(left, right, w / weight_sum),
            )?;
        let color = Color::new(self.color_space, &components)?;

        terminal::show_colors(state, iter::once(color), self.output, self.size)
    }
}

fn add_color(
    (cs1, items1): (ColorSpace, Vec<f64>),
    right: Color,
    right_weight: f64,
) -> Result<(ColorSpace, Vec<f64>)> {
    let (cs2, items2) = right.divide();
    if cs1 == cs2 {
        let new_components = items1
            .into_iter()
            .zip(items2)
            .map(|(a, b)| a + (b * right_weight))
            .collect::<Vec<_>>();

        Ok((cs1, new_components))
    } else {
        bail!("Unequal color spaces {:?} and {:?}", cs1, cs2);
    }
}
