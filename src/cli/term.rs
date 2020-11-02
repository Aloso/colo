use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

/// Returns the `term` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("term")
        .about("Displays the most common terminal colors")
        .version(super::APP_VERSION)
}

/// Represents the input of the `term` subcommand
pub struct Term;

/// Return the input for the `term` subcommand
pub fn get(_matches: &ArgMatches) -> Result<Term> {
    Ok(Term)
}
