#![deny(unsafe_code)]

use anyhow::Result;

mod cli;
mod color;
mod output;

/// Entry point for the application.
///
/// It uses `anyhow` everywhere to easily propagate errors. Most errors are not
/// recoverable and simply need to be reported. Rusts runtime handles this
/// automatically, when an error is returned from `main()`.
fn main() -> Result<()> {
    match cli::app().get_matches().subcommand() {
        ("libs", Some(matches)) => {
            output::libs::libs(cli::libs::get(&matches)?);
        }
        ("term", Some(matches)) => {
            output::term::term(cli::term::get(matches)?)?;
        }
        ("print", Some(matches)) => {
            output::print::print(cli::print::get(matches)?)?;
        }
        ("show", Some(matches)) => {
            output::show::show(cli::show::get(&matches)?)?;
        }
        ("list", Some(matches)) => {
            output::list::list(cli::list::get(&matches)?)?;
        }
        _ => {
            cli::app().print_help().unwrap();
        }
    }
    Ok(())
}
