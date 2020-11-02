use clap::{App, ArgMatches, SubCommand};

/// Returns the `libs` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("libs")
        .about("Displays the dependency tree")
        .version_short("v")
        .version(super::APP_VERSION)
}

/// The struct representing the `libs` subcommand
pub struct Libs;

/// Return the input for the `libs` subcommand
pub fn get(matches: &ArgMatches) -> Option<Libs> {
    if matches.is_present("libs") {
        Some(Libs)
    } else {
        None
    }
}
