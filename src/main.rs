#![deny(unsafe_code)]

use anyhow::Result;
use atty::Stream;

use cli::MainCmd;

mod cli;
mod color;
mod terminal;

/// The application state. This is currently passed to all functions that need it.
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
    MainCmd::main(&mut state)
}
