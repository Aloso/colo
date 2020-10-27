use anyhow::{bail, Context, Result};
use clap::{App, Arg, ArgGroup};

use crate::color::{Color, ColorSpace};

macro_rules! color_arg {
    ($name:expr, $($rest:tt)*) => {
        color_arg!(in Arg::with_name($name)
            .long($name)
            .use_delimiter(true)
            .value_delimiter("/")
            .allow_hyphen_values(true), $($rest)*)
    };
    (in $arg:expr, short: $short:expr, $($rest:tt)*) => {
        color_arg!(in $arg.
            short($short), $($rest)*)
    };
    (in $arg:expr, value_names: $value_names:expr, $($rest:tt)*) => {
        color_arg!(in $arg
            .number_of_values($value_names.len() as u64)
            .value_names($value_names), $($rest)*)
    };
    (in $arg:expr, help: $help:literal, $($rest:tt)*) => {
        color_arg!(in $arg
            .long_help($help)
            .help($help.split('\n').next().unwrap()), $($rest)*)
    };
    (in $arg:expr, ) => {
        $arg
    };
}

fn clap_args() -> clap::ArgMatches<'static> {
    App::new("colo")
        .version("0.1")
        .author("Ludwig Stecher <ludwig.stecher@gmx.de>")
        .about("Displays colors in various color spaces.")
        .usage("colo [FLAGS]\n    colo <COLOR>\n    colo [OPTIONS]")
        .arg(
            Arg::with_name("terminal")
                .long("terminal")
                .short("t")
                .help("Show default terminal colors"),
        )
        .arg(
            Arg::with_name("COLOR")
                .long_help(
                    "The color to display. Supported formats:\n\
                    - HTML color name (e.g. 'rebeccapurple')\n\
                    - Hexadecimal RGB color (e.g. '07F', '0077FF', '000_777_FFF')\n",
                )
                .index(1),
        )
        .arg(color_arg!("rgb", short: "r",
            value_names: &["red", "green", "blue"],
            help: "RGB color. Requires three values (0-255)",
        ))
        .arg(color_arg!("cmy", short: "c",
            value_names: &["cyan", "magenta", "yellow"],
            help: "CMY color. Requires three values (0-1)",
        ))
        .arg(color_arg!("cmyk", short: "k",
            value_names: &["cyan", "magenta", "yellow", "key"],
            help: "CMYK color. Requires four values (0-1)",
        ))
        .arg(color_arg!("hsv", short: "v",
            value_names: &["hue", "saturation", "value"],
            help: "HSV color (hue: 0-360, saturation: 0-1, value: 0-1)",
        ))
        .arg(color_arg!("hsl", short: "l",
            value_names: &["hue", "saturation", "light"],
            help: "HSL color (hue: 0-360, saturation: 0-1, light: 0-1)",
        ))
        .arg(color_arg!("lch",
            value_names: &["luminance", "chroma", "hue"],
            help: "LCH color. Requires three values (0-1)",
        ))
        .arg(color_arg!("luv",
            value_names: &["luminance", "u", "v"],
            help: "CIELUV color (luminance: 0 to 100, u: -134 to 220, v: -140 to 122)",
        ))
        .arg(color_arg!("lab",
            value_names: &["lightness", "a", "b"],
            help: "CIELAB color\n\
                <lightness>  (0 to 100)\n\
                <a>          green (negative) and red (positive) component\n\
                <b>          blue (negative) and yellow (positive) component",
        ))
        .arg(color_arg!("hunterlab",
            value_names: &["lightness", "a", "b"],
            help: "Hunter Lab color\n\
                <lightness>  (0 to 100)\n\
                <a>          green (negative) and red (positive) component\n\
                <b>          blue (negative) and yellow (positive) component",
        ))
        .arg(color_arg!("xyz",
            value_names: &["x", "y", "z"],
            help: "CIE 1931 XYZ color",
        ))
        .arg(color_arg!("yxy",
            value_names: &["y", "x", "y"],
            help: "CIE YXY color",
        ))
        .group(
            ArgGroup::with_name("color_group")
                .args(&[
                    "COLOR",
                    "rgb",
                    "cmy",
                    "cmyk",
                    "hsv",
                    "hsl",
                    "lch",
                    "luv",
                    "lab",
                    "hunterlab",
                    "xyz",
                    "yxy",
                ])
                .conflicts_with("terminal")
                .multiple(false),
        )
        .get_matches()
}

pub enum Input {
    Terminal,
    ColorString(String),
    Color(Color),
}

pub fn parse_args() -> Result<Input> {
    let matches = clap_args();

    Ok(if matches.is_present("terminal") {
        Input::Terminal
    } else if let Some(color_arg) = matches.args.get("COLOR") {
        Input::ColorString(
            color_arg.vals[0]
                .to_str()
                .context("Invalid UTF-8")?
                .to_string(),
        )
    } else if let Some(values) = matches.args.get("color_group") {
        let components = values
            .vals
            .iter()
            .map(|s| s.to_str().context("UTF-8").and_then(|s| Ok(s.parse()?)))
            .collect::<Result<Vec<f64>, _>>()?;

        let color_space = if matches.is_present("rgb") {
            ColorSpace::Rgb
        } else if matches.is_present("cmy") {
            ColorSpace::Cmy
        } else if matches.is_present("cmyk") {
            ColorSpace::Cmyk
        } else if matches.is_present("hsv") {
            ColorSpace::Hsv
        } else if matches.is_present("hsl") {
            ColorSpace::Hsl
        } else if matches.is_present("lch") {
            ColorSpace::Lch
        } else if matches.is_present("luv") {
            ColorSpace::Luv
        } else if matches.is_present("lab") {
            ColorSpace::Lab
        } else if matches.is_present("hunterlab") {
            ColorSpace::HunterLab
        } else if matches.is_present("xyz") {
            ColorSpace::Xyz
        } else if matches.is_present("yxy") {
            ColorSpace::Yxy
        } else {
            bail!("No color space found")
        };
        Input::Color(Color::new(color_space, &components)?)
    } else {
        bail!("No argument was provided\n\nFor more information try `--help`")
    })
}
