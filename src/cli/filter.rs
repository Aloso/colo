use std::{fmt, iter};

use anyhow::{anyhow, bail, Context, Result};
use clap::{App, Arg, SubCommand};

use crate::{
    color::{space::*, Color, ColorFormat, ColorSpace},
    terminal::stdin,
    State,
};

use super::{parse::parse_filter, util, Cmd, Show};

#[derive(Debug)]
pub struct Filter {
    pub show: Show,
    pub filters: FilterList,
}

#[derive(Debug)]
pub struct FilterList {
    pub(super) items: Vec<(FilterKey, FilterValue)>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum FilterKey {
    Brightness,
    Contrast,
    Grayscale,
    HueRotate,
    Invert,
    Saturate,
    Sepia,
    Other(ColorSpace, String),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FilterValue {
    Add(f64),
    Mul(f64),
    Div(f64),
    Set(f64),
}

impl FilterValue {
    fn get_add(&self) -> Result<f64> {
        match *self {
            FilterValue::Add(n) => Ok(n),
            _ => bail!("Unsupported value `{}`", self),
        }
    }

    fn apply(&self, num: f64) -> f64 {
        match *self {
            FilterValue::Add(n) => num + n,
            FilterValue::Mul(n) => num * n,
            FilterValue::Div(n) => num / n,
            FilterValue::Set(n) => n,
        }
    }
}

impl fmt::Display for FilterValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FilterValue::Add(n) => write!(f, "{}", n),
            FilterValue::Mul(n) => write!(f, "* {}", n),
            FilterValue::Div(n) => write!(f, "/ {}", n),
            FilterValue::Set(n) => write!(f, "= {}", n),
        }
    }
}

const FILTER_HELP_MESSAGE: &str = "\
A list of filters, for example:

colo filter brightness -10%, hue-rotate 30 -- orange

Available filters are:
* brightness
* contrast
* grayscale
* hue-rotate
* invert
* saturate
* sepia
* color components:
   * r, g, b
   * h, s, l
   * when specifying other components,
     the color space is needed, e.g. `cmy:c`

Color components are relative, unless prefixed with `=`, e.g.

colo filter r=10 g+50 lab:l-20 -- orange
 ";

const COLOR_HELP_MESSAGE: &str = "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name, e.g. 'rebeccapurple'
* Hexadecimal RGB color, e.g. '07F', '0077FF'
* Color components, e.g. 'hsl(30, 100%, 50%)'
  Commas and parentheses are optional.
  For supported color spaces, see <https://aloso.github.io/colo/color_spaces>

If colo is used behind a pipe or outside of a terminal, the colors can be provided via stdin, e.g.

$ echo orange blue FF7700 | colo filter contrast 30%";

impl Cmd for Filter {
    fn command<'a, 'b>(state: State) -> App<'a, 'b> {
        SubCommand::with_name("filter")
            .about("Apply filters to the color(s)")
            .visible_alias("f")
            .args(&[
                Arg::with_name("filters")
                    .index(1)
                    .takes_value(true)
                    .multiple(true)
                    .required(true)
                    .allow_hyphen_values(true)
                    .help(FILTER_HELP_MESSAGE),
                Arg::with_name("colors")
                    .takes_value(true)
                    .index(2)
                    .last(true)
                    .required(state.interactive)
                    .multiple(true)
                    .use_delimiter(false)
                    .help(COLOR_HELP_MESSAGE),
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
                Arg::with_name("size")
                    .long("size")
                    .short("s")
                    .takes_value(true)
                    .default_value("4")
                    .help("Size of the color square in terminal rows"),
            ])
    }

    fn parse(matches: &clap::ArgMatches, &mut state: &mut State) -> Result<Self> {
        let mut colors = match matches.values_of("colors") {
            Some(values) => util::values_to_colors(values, state)?,
            None => vec![],
        };
        if !state.interactive && colors.is_empty() {
            let text = stdin::read_all()?;
            colors = util::values_to_colors(iter::once(text.as_str()), state)?;
        }

        let filters = matches
            .values_of("filters")
            .ok_or_else(|| anyhow!("Filters not present"))?
            .flat_map(|s| iter::once(s).chain(iter::once(" ")))
            .collect::<String>();

        let filters = parse_filter(&filters)?;

        let size = matches
            .value_of("size")
            .map(util::parse_size)
            .unwrap_or(Ok(4))?;

        let output = util::get_color_format(&matches, "output-format")?;

        let show = Show {
            colors,
            size,
            output,
        };
        Ok(Self { show, filters })
    }

    fn run(&self, state: State) -> Result<()> {
        let mut show = self.show.clone();

        for (key, value) in &self.filters.items {
            match *key {
                FilterKey::Brightness => {
                    for (color, fmt) in &mut show.colors {
                        brightness(color, fmt, value)?;
                    }
                }
                FilterKey::Contrast => {
                    bail!("`contrast` is not yet implemented");
                }
                FilterKey::Grayscale => {
                    bail!("`grayscale` is not yet implemented");
                }
                FilterKey::HueRotate => {
                    for (color, fmt) in &mut show.colors {
                        hue_rotate(color, fmt, value)?;
                    }
                }
                FilterKey::Invert => {
                    for (color, fmt) in &mut show.colors {
                        invert(color, fmt, value)?;
                    }
                }
                FilterKey::Saturate => {
                    bail!("`saturate` is not yet implemented");
                }
                FilterKey::Sepia => {
                    bail!("`sepia` is not yet implemented");
                }
                FilterKey::Other(s, ref comp) => {
                    for (color, fmt) in &mut show.colors {
                        other(color, fmt, value, s, comp)?;
                    }
                }
            }
        }

        show.run(state)
    }
}

fn brightness(color: &mut Color, fmt: &mut ColorFormat, value: &FilterValue) -> Result<()> {
    fn br(component: f64, amount: f64) -> f64 {
        (component * amount).min(255.0)
    }

    let amount = value.get_add().context("Invalid `invert` value")?;

    let Rgb { r, g, b } = (*color).into();
    let rgb = Rgb::new(br(r, amount), br(g, amount), br(b, amount));
    *color = Color::Rgb(rgb);
    *fmt = ColorFormat::Normal(ColorSpace::Rgb);
    Ok(())
}

fn hue_rotate(color: &mut Color, fmt: &mut ColorFormat, value: &FilterValue) -> Result<()> {
    let amount = value.get_add().context("Invalid `hue-rotate` value")?;

    let Hsl { h, s, l } = (*color).into();
    let hsl = Hsl::new((h + amount).rem_euclid(360.0), s, l);
    *color = Color::Hsl(hsl);
    *fmt = ColorFormat::Normal(ColorSpace::Hsl);
    Ok(())
}

fn invert(color: &mut Color, fmt: &mut ColorFormat, value: &FilterValue) -> Result<()> {
    fn inv(component: f64, amount: f64) -> f64 {
        let diff = 255.0 - component * 2.0;
        component + diff * amount
    }

    let amount = value.get_add().context("Invalid `invert` value")?;

    let Rgb { r, g, b } = (*color).into();
    let rgb = Rgb::new(inv(r, amount), inv(g, amount), inv(b, amount));
    *color = Color::Rgb(rgb);
    *fmt = ColorFormat::Normal(ColorSpace::Rgb);
    Ok(())
}

fn other(
    color: &mut Color,
    fmt: &mut ColorFormat,
    value: &FilterValue,
    s: ColorSpace,
    comp: &str,
) -> Result<()> {
    let mut new_color = color.to_color_space(s);
    let c = new_color
        .get_component(comp)
        .with_context(|| anyhow!("Component {} doesn't exist in the {} color space", comp, s))?;
    *c = value.apply(*c);

    *color = new_color;
    *fmt = ColorFormat::Normal(s);
    Ok(())
}
