use anyhow::{bail, Result};
use clap::{Arg, SubCommand};
use console::Term;

use super::{util, Cmd};
use crate::{
    color::{self, Color, ColorFormat, ColorSpace},
    terminal::{self, stdin},
    State,
};

pub struct Gradient {
    colors: Vec<(Color, ColorFormat)>,
    color_space: ColorSpace,
    output: ColorFormat,
    color_num: Option<usize>,
}

const COLOR_HELP_MESSAGE: &str = "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue FF7700 | colo gradient";

impl Cmd for Gradient {
    fn command<'a, 'b>(state: State) -> clap::App<'a, 'b> {
        SubCommand::with_name("gradient")
            .about("Create a gradient between colors")
            .args(&[
                Arg::with_name("colors")
                    .takes_value(true)
                    .index(1)
                    .required(state.interactive)
                    .multiple(true)
                    .use_delimiter(false)
                    .help(COLOR_HELP_MESSAGE),
                Arg::with_name("color-space")
                    .long("color-space")
                    .short("c")
                    .help(
                        "The color space which the colors are mixed in. \
                        Color spaces are explained here: \
                        <https://aloso.github.io/colo/color_spaces>",
                    )
                    .possible_values(&[
                        "rgb",
                        "cmy",
                        "cmyk",
                        "luv",
                        "lab",
                        "hunterlab",
                        "xyz",
                        "yxy",
                    ])
                    .case_insensitive(true)
                    .default_value("lab"),
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
                Arg::with_name("steps")
                    .long("steps")
                    .short("s")
                    .takes_value(true)
                    .help("Number of color steps, defaults to 10"),
            ])
    }

    fn parse(matches: &clap::ArgMatches, &mut state: &mut State) -> Result<Self> {
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

        if colors.len() != 2 {
            bail!("You have to enter exactly 2 colors");
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

        let color_num = matches
            .value_of("steps")
            .map(|s| {
                let s = s.parse::<usize>()? + 1;
                if s < colors.len() {
                    bail!("Fewer steps than colors");
                }
                Ok(s)
            })
            .transpose()?;

        Ok(Gradient {
            colors,
            color_space,
            output,
            color_num,
        })
    }

    fn run(&self, state: State) -> Result<()> {
        let mut term_width = None;
        let mut get_term_width = || {
            if let Some(w) = term_width {
                w
            } else {
                let w = Term::stdout().size().1 as usize;
                term_width = Some(w);
                w
            }
        };

        let color_steps = self.color_num.unwrap_or_else(|| {
            if state.color {
                (get_term_width() * 2) - 1
            } else {
                10
            }
        });
        let (c1, c2) = (self.colors[0].0, self.colors[1].0);

        if state.color {
            let w = get_term_width();
            terminal::list_small(
                state,
                None,
                (0..=color_steps).map(|i| {
                    let ratio = (i as f64) / (color_steps as f64);
                    let color = c1.mix_with(c2, self.color_space, ratio);
                    (color, self.output)
                }),
                2 * w / (color_steps + 1),
            )?;
        } else {
            for i in 0..=color_steps {
                let ratio = (i as f64) / (color_steps as f64);
                let color = c1.mix_with(c2, self.color_space, ratio);
                println!("{}", self.output.format_or_hex(color));
            }
        }
        Ok(())
    }
}
