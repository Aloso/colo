use clap::{App, AppSettings};

pub mod contrast;
pub mod libs;
pub mod list;
pub mod print;
pub mod show;
pub mod term;
pub mod textcolor;

mod util;

/// Name of the program
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Version of the program
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The dependency tree, generated by the build script
pub const DEPENDENCIES: &str = include_str!(concat!(env!("OUT_DIR"), "/dependencies.txt"));

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

/// Returns the command line arguments parsed by clap.
///
/// Note that, if the `--version` or `--help` flag was provided,
/// clap terminates the application, so this function never returns.
pub fn app<'a, 'b>(interactive: bool) -> App<'a, 'b> {
    App::new(APP_NAME)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::InferSubcommands)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(APP_VERSION)
        .author("Ludwig Stecher <ludwig.stecher@gmx.de>")
        .about("Manages colors in various color spaces.")
        .subcommand(show::command(interactive))
        .subcommand(print::command(interactive))
        .subcommand(term::command())
        .subcommand(contrast::command(interactive))
        .subcommand(textcolor::command(interactive))
        .subcommand(list::command())
        .subcommand(libs::command())
        .max_term_width(100)
}
