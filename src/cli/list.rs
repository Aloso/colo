use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

use crate::terminal;

use super::Cmd;

/// The `list` subcommand
pub struct List;

impl Cmd for List {
    fn command<'a, 'b>(_state: crate::State) -> App<'a, 'b> {
        SubCommand::with_name("list").about("Lists all HTML colors")
    }

    fn parse(_matches: &ArgMatches, _state: &mut crate::State) -> Result<Self> {
        Ok(List)
    }

    fn run(&self, state: crate::State) -> Result<()> {
        terminal::list(state)
    }
}
