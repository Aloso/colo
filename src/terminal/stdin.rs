use anyhow::Result;
use console::Term;
use std::io::{self, Read};

use crate::State;

pub(crate) fn read_all() -> Result<String> {
    let mut text = Vec::new();
    io::stdin().read_to_end(&mut text)?;
    let mut text = String::from_utf8(text)?;
    if text.ends_with('\n') {
        text.truncate(text.len() - 1);
    }
    Ok(text)
}

pub(crate) fn read_line(state: State) -> Result<String> {
    let mut text = String::new();

    if state.interactive {
        let term = Term::stdout();
        term.write_str("\n")?;
        term.move_cursor_up(1)?;
        term.write_str("Enter color: ")?;

        io::stdin().read_line(&mut text)?;
        if text.ends_with('\n') {
            text.truncate(text.len() - 1);
        }

        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
    } else {
        io::stdin().read_line(&mut text)?;
        if text.ends_with('\n') {
            text.truncate(text.len() - 1);
        }
    }

    Ok(text)
}
