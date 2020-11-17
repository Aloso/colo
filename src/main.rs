#![deny(unsafe_code)]

use anyhow::{bail, Result};
use atty::Stream;

mod cli;
mod color;
mod output;
mod picker;

#[derive(Debug, Copy, Clone)]
pub struct State {
    interactive: bool,
    color: bool,
}

/// Entry point for the application.
///
/// It uses `anyhow` everywhere to easily propagate errors. Most errors are not
/// recoverable and simply need to be reported. Rusts runtime handles this
/// automatically, when an error is returned from `main()`.
fn main() -> Result<()> {
    let mut state = State {
        interactive: atty::is(Stream::Stdin),
        color: atty::is(Stream::Stdout),
    };
    let matches = cli::app(state).get_matches();
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

    match matches.subcommand() {
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
        ("pick", Some(matches)) => {
            output::show::show(cli::pick::get(&matches)?, state)?;
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
