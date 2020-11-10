#![deny(unsafe_code)]

use anyhow::{bail, Result};
use atty::Stream;
use std::env;

mod cli;
mod color;
mod output;

#[derive(Debug, Copy, Clone)]
pub struct State {
    interactive: bool,
    ansi_output: bool,
}

/// Entry point for the application.
///
/// It uses `anyhow` everywhere to easily propagate errors. Most errors are not
/// recoverable and simply need to be reported. Rusts runtime handles this
/// automatically, when an error is returned from `main()`.
fn main() -> Result<()> {
    let force_ansi_output = env::var("FORCE_ANSI_OUTPUT").as_deref() == Ok("1");
    let state = State {
        interactive: atty::is(Stream::Stdin),
        ansi_output: atty::is(Stream::Stdout) || force_ansi_output,
    };
    if force_ansi_output {
        colored::control::set_override(true);
    }

    match cli::app(state).get_matches().subcommand() {
        ("libs", Some(matches)) => {
            output::libs::libs(cli::libs::get(&matches)?);
        }
        ("term", Some(matches)) => {
            output::term::term(cli::term::get(matches)?)?;
        }
        ("print", Some(matches)) => {
            output::print::print(cli::print::get(matches, state)?)?;
        }
        ("show", Some(matches)) => {
            output::show::show(cli::show::get(&matches, state)?, state)?;
        }
        ("list", Some(matches)) => {
            output::list::list(cli::list::get(&matches)?, state)?;
        }
        ("contrast", Some(matches)) => {
            output::contrast::contrast(cli::contrast::get(&matches, state)?, state)?;
        }
        ("textcolor", Some(matches)) => {
            output::textcolor::textcolor(cli::textcolor::get(&matches, state)?, state)?;
        }
        (c, _) => bail!("Unknown subcommand {:?}", c),
    }
    Ok(())
}
