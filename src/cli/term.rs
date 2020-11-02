use clap::{App, ArgMatches, SubCommand};

/// Returns the `term` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("term")
        .about("Displays the most common terminal colors")
        .version_short("v")
        .version(super::APP_VERSION)
}

/// Represents the input of the `term` subcommand
pub struct Term;

/// Return the input for the `term` subcommand
pub fn get(matches: &ArgMatches) -> Option<Term> {
    if matches.is_present("term") {
        Some(Term)
    } else {
        None
    }
}
