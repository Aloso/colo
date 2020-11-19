use anyhow::Result;
use clap::{App, ArgMatches, SubCommand};

use super::{Cmd, APP_NAME, APP_VERSION, DEPENDENCIES};
use crate::State;

/// The `libs` subcommand
pub struct Libs;

impl Cmd for Libs {
    fn command<'a, 'b>(_state: crate::State) -> App<'a, 'b> {
        SubCommand::with_name("libs").about("Displays the dependency tree")
    }

    fn parse(_matches: &ArgMatches, _state: &mut crate::State) -> Result<Self> {
        Ok(Libs)
    }

    fn run(&self, _state: State) -> Result<()> {
        println!("{} v{}\n{}", APP_NAME, APP_VERSION, DEPENDENCIES);

        Ok(())
    }
}
