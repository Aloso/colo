use anyhow::{bail, Context, Result};
use clap::{App, Arg, ArgGroup};

use crate::color::{Color, ColorSpace};

/// Creates an CLI flag that acts as an alias for a different input
fn alias<'a, 'b>(name: &'static str, short: &'static str, help: &'static str) -> Arg<'a, 'b> {
    Arg::with_name(name).short(short).help(help)
}

/// Color spaces, as they can be provided in the command line
const COLOR_SPACES: &[&str] = &[
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
];

/// Help about color spaces in the command line
const COLOR_SPACE_HELP: &str = "\
The input color space. Use this argument together with [COLOR], e.g.

$ colo -i cmy 1/0/.5

Supported color spaces
======================
name      arguments                   values
-------------------------------------------------------------------
rgb       red, green, blue            0-255    0-255    0-255
cmy       cyan, magenta, yellow       0-1      0-1      0-1
cmyk      cyan, magenta, yellow, key  0-1      0-1      0-1     0-1
hsv       hue, saturation, value      0-360    0-1      0-1
hsl       hue, saturation, light      0-360    0-1      0-1
lch       luminance, chroma, hue      0-100    0-100    0-360
luv       luminance, u, v (CIELUV)    0-100 –134-220 –140-122
lab       lightness, a, b (CIELAB)    0-100     ?        ?
hunterlab lightness, a, b (CIELAB)    0-100     ?        ?
xyz       x,y,z (CIE 1931 XYZ color)   ?        ?        ?
yxy       y1,x,y2 (CIE YXY color)      ?        ?        ?
 ";

const COLOR_HELP_MESSAGE: &str = "\
The input color. Supported formats:
- HTML color name (e.g. 'rebeccapurple')
- Hexadecimal RGB color (e.g. '07F', '0077FF')
- Color components (e.g. '127/0/255')
  use `--in` to specify the used color space";

/// Returns the command line arguments parsed by clap.
///
/// Note that, if the `--version` or `--help` flag was provided,
/// clap terminates the application, so this function never returns.
fn clap_args() -> clap::ArgMatches<'static> {
    App::new("colo")
        .version(env!("CARGO_PKG_VERSION"))
        .version_short("v")
        .author("Ludwig Stecher <ludwig.stecher@gmx.de>")
        .about("\nDisplays colors in various color spaces.")
        .arg(
            Arg::with_name("TERMINAL")
                .long("terminal")
                .short("t")
                .help("Show default terminal colors")
                .conflicts_with_all(&["COLOR", "INPUT_ALIASES", "OUTPUT_COLOR_SPACE", "SIZE"]),
        )
        .arg(
            Arg::with_name("COLOR")
                .help(COLOR_HELP_MESSAGE)
                .max_values(4)
                .use_delimiter(true)
                .value_delimiter("/"),
        )
        .arg(
            Arg::with_name("INPUT_COLOR_SPACE")
                .long("in")
                .short("i")
                .takes_value(true)
                .possible_values(COLOR_SPACES)
                .hide_possible_values(true)
                .case_insensitive(true)
                .help(
                    "Input color space [possible values: rgb, cmy, \
                    cmyk, hsv, hsl, lch, luv, lab, hunterlab, xyz, yxy]",
                )
                .long_help(COLOR_SPACE_HELP),
        )
        .arg(
            Arg::with_name("OUTPUT_COLOR_SPACE")
                .long("out")
                .short("o")
                .takes_value(true)
                .help("Output color space")
                .possible_values(COLOR_SPACES)
                .case_insensitive(true),
        )
        .arg(alias("RGB", "R", "Alias for `--in rgb`"))
        .arg(alias("CMY", "C", "Alias for `--in cmy`"))
        .arg(alias("CMYK", "K", "Alias for `--in cmyk`"))
        .arg(alias("HSV", "V", "Alias for `--in hsv`"))
        .arg(alias("HSL", "L", "Alias for `--in hsl`"))
        .arg(
            Arg::with_name("SIZE")
                .long("size")
                .short("s")
                .takes_value(true)
                .help(
                    "Size of the color square in terminal rows (default: 4). Set to 0 to hide it.",
                )
                .requires("COLOR"),
        )
        .group(
            ArgGroup::with_name("INPUT_ALIASES")
                .args(&["INPUT_COLOR_SPACE", "RGB", "CMY", "CMYK", "HSV", "HSL"])
                .requires("COLOR"),
        )
        .set_term_width(80)
        .get_matches()
}

/// The CLI input after parsing.
pub enum Input {
    /// If the `--terminal` flag was set
    Terminal,
    /// If a color was provided. Additional arguments are
    ///    - `output`: Output color space (default `Rgb`)
    ///    - `size`: The size of the color square (default `4`)
    ColorInput {
        input: ColorInput,
        output: ColorSpace,
        size: u32,
    },
}

/// The color provided to `colo`
pub enum ColorInput {
    /// A hexadecimal color (e.g. `FF7700`) or a HTML color name (e.g. `red`)
    /// was provided. This has yet to be parsed.
    HexOrHtml(String),
    /// A color space and a list of components (e.g. `--in cmy 1/0/.5`) was
    /// provided. This has already been parsed.
    Color(Color),
}

/// Parses the CLI arguments,
pub fn parse_args() -> Result<Input> {
    let matches = clap_args();

    let input = if matches.is_present("INPUT_ALIASES") {
        let string = match matches.value_of("INPUT_COLOR_SPACE") {
            Some(v) => v,
            None if matches.is_present("RGB") => "rgb",
            None if matches.is_present("CMY") => "cmy",
            None if matches.is_present("CMYK") => "cmyk",
            None if matches.is_present("HSV") => "hsv",
            None if matches.is_present("HSL") => "hsl",
            _ => bail!("No color space found"),
        };
        Some(string.parse()?)
    } else {
        None
    };

    let output = matches
        .value_of("OUTPUT_COLOR_SPACE")
        .unwrap_or("rgb")
        .to_lowercase()
        .parse()
        .expect("Invalid output color space");

    let size = matches
        .value_of("SIZE")
        .map(|s| {
            s.parse()
                .context(format!("The size {:?} could not be parsed", s))
        })
        .unwrap_or(Ok(4))?;

    Ok(if matches.is_present("TERMINAL") {
        Input::Terminal
    } else if let Some(mut color_args) = matches.values_of("COLOR") {
        match input {
            Some(input) => {
                let components = color_args
                    .map(|s| s.parse().context(format!("{:?} could not be parsed", s)))
                    .collect::<Result<Vec<f64>, anyhow::Error>>()?;
                let color = Color::new(input, &components)?;

                Input::ColorInput {
                    input: ColorInput::Color(color),
                    output,
                    size,
                }
            }
            None => {
                if color_args.len() > 1 {
                    bail!("Too many arguments provided\n\nFor more information try `--help`");
                }
                if let Some(color_arg) = color_args.next() {
                    Input::ColorInput {
                        input: ColorInput::HexOrHtml(color_arg.to_string()),
                        output,
                        size,
                    }
                } else {
                    bail!("No argument was provided\n\nFor more information try `--help`");
                }
            }
        }
    } else {
        bail!("No argument was provided\n\nFor more information try `--help`")
    })
}
