use clap::{App, AppSettings};

mod libs;
mod print;
mod show;
mod term;
mod util;

pub use libs::{get as get_libs, Libs};
pub use print::{get as get_print, Print};
pub use show::{get as get_show, Show};
pub use term::{get as get_term, Term};

/// Name of the program
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Version of the program
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The dependency tree, generated by the build script
pub const DEPENDENCIES: &str = include_str!("../../dependencies.txt");

/// Color spaces, as they can be provided in the command line
const COLOR_FORMATS: &[&str] = &[
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
    "html",
    "hex",
];

macro_rules! color_help_message {
    () => {
        "\
The input colors. Multiple colors can be specified. Supported formats:

* HTML color name (e.g. 'rebeccapurple')
* Hexadecimal RGB color (e.g. '07F', '0077FF')
* Color components (e.g. '127, 0, 255', 'hsl(30, 1, .5)').
  If no color space is specified, it defaults to 'rgb'.
  Commas and parentheses are optional."
    };
}

macro_rules! color_space_help {
    () => {
        "\
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
yxy       y1,x,y2 (CIE YXY color)      ?        ?        ?"
    };
}

const COLOR_HELP_LONG_MESSAGE: &str = concat!(color_help_message!(), "\n\n", color_space_help!());

const COLOR_HELP_MESSAGE: &str = color_help_message!();

/// Returns the command line arguments parsed by clap.
///
/// Note that, if the `--version` or `--help` flag was provided,
/// clap terminates the application, so this function never returns.
pub fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(APP_NAME)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .version(APP_VERSION)
        .author("Ludwig Stecher <ludwig.stecher@gmx.de>")
        .about("Displays colors in various color spaces.")
        .subcommand(term::command())
        .subcommand(libs::command())
        .subcommand(print::command())
        .subcommand(show::command())
        .set_term_width(80)
}
