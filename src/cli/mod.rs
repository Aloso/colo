use anyhow::{bail, Result};
use clap::{App, AppSettings, Arg, ArgMatches};

use crate::State;

mod contrast;
mod libs;
mod list;
mod mix;
mod pick;
mod print;
mod show;
mod term;
mod textcolor;

pub(crate) use self::mix::Mix;
pub(crate) use contrast::Contrast;
pub(crate) use libs::Libs;
pub(crate) use list::List;
pub(crate) use pick::Pick;
pub(crate) use print::{Print, TextStyle};
pub(crate) use show::Show;
pub(crate) use term::Term;
pub(crate) use textcolor::TextColor;

mod util;

/// Name of the program
pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Version of the program
pub(crate) const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The dependency tree, generated by the build script
pub(crate) const DEPENDENCIES: &str = include_str!(concat!(env!("OUT_DIR"), "/dependencies.txt"));

/// Color formats, as they can be provided in the command line
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
    "gry",
];

/// A clap (sub)command.
///
/// Each subcommand is defined in its own module and is registered in
/// `MainCmd::command` and in `MainCmd::parse`.
pub(crate) trait Cmd {
    /// Returns a clap app for this (sub)command.
    fn command<'a, 'b>(state: State) -> App<'a, 'b>
    where
        Self: Sized;

    /// Parses the CLI arguments received from clap.
    fn parse(matches: &ArgMatches, state: &mut State) -> Result<Self>
    where
        Self: Sized;

    /// Executes the appropriate function for this (sub)command.
    fn run(&self, state: State) -> Result<()>;
}

/// The main CLI command
pub(crate) struct MainCmd {
    subcommand: Box<dyn Cmd>,
}

impl MainCmd {
    /// Parses the CLI arguments and executes the appropriate function.
    pub(crate) fn main(state: &mut State) -> Result<()> {
        let matches = Self::command(*state).get_matches();
        let s = Self::parse(&matches, state)?;
        s.run(*state)
    }
}

impl Cmd for MainCmd {
    /// Returns the command line arguments parsed by clap.
    ///
    /// Note that, if the `--version` or `--help` flag was provided,
    /// clap terminates the application, so this function never returns.
    fn command<'a, 'b>(state: State) -> App<'a, 'b> {
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
            .subcommand(Show::command(state))
            .subcommand(Print::command(state))
            .subcommand(Pick::command(state))
            .subcommand(Term::command(state))
            .subcommand(Contrast::command(state))
            .subcommand(TextColor::command(state))
            .subcommand(Mix::command(state))
            .subcommand(List::command(state))
            .subcommand(Libs::command(state))
            .arg(
                Arg::with_name("color")
                    .long("color")
                    .takes_value(true)
                    .possible_values(&["always", "never", "auto"])
                    .default_value("auto")
                    .help(
                        "Define if the output should be colored. \
                        By default colo disables color output \
                        when used behind a pipe or not in a tty",
                    ),
            )
            .max_term_width(100)
    }

    fn parse(matches: &ArgMatches, state: &mut State) -> Result<Self> {
        match matches.value_of("color").unwrap() {
            "always" => {
                colored::control::set_override(true);
                state.color = true;
            }
            "never" => {
                colored::control::set_override(false);
                state.color = false;
            }
            _ => {}
        }

        let subcommand: Box<dyn Cmd> = match matches.subcommand() {
            ("show", Some(matches)) => Box::new(Show::parse(matches, state)?),
            ("libs", Some(matches)) => Box::new(Libs::parse(matches, state)?),
            ("term", Some(matches)) => Box::new(Term::parse(matches, state)?),
            ("print", Some(matches)) => Box::new(Print::parse(matches, state)?),
            ("pick", Some(matches)) => Box::new(Pick::parse(matches, state)?),
            ("list", Some(matches)) => Box::new(List::parse(matches, state)?),
            ("contrast", Some(matches)) => Box::new(Contrast::parse(matches, state)?),
            ("textcolor", Some(matches)) => Box::new(TextColor::parse(matches, state)?),
            ("mix", Some(matches)) => Box::new(Mix::parse(matches, state)?),
            (c, _) => bail!("Unknown subcommand {:?}", c),
        };

        Ok(MainCmd { subcommand })
    }

    fn run(&self, state: State) -> Result<()> {
        self.subcommand.run(state)
    }
}
