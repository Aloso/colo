use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

/// Returns the `list` subcommand
pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("Lists all HTML colors")
        .version(super::APP_VERSION)
}

/// The struct representing the `list` subcommand
pub struct List;

/// Return the input for the `libs` subcommand
pub fn get(_matches: &ArgMatches) -> Result<List> {
    Ok(List)
}
