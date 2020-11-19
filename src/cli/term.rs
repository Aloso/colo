use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

use crate::terminal;

use super::Cmd;

/// The `term` subcommand
pub struct Term;

impl Cmd for Term {
    fn command<'a, 'b>(_state: crate::State) -> App<'a, 'b> {
        SubCommand::with_name("term").about("Display the most common terminal colors")
    }

    fn parse(_matches: &ArgMatches, _state: &mut crate::State) -> Result<Self> {
        Ok(Term)
    }

    fn run(&self, _state: crate::State) -> Result<()> {
        terminal::term_colors()
    }
}
