#![deny(unsafe_code)]

use anyhow::Result;
use command_line::{ColorInput, Input};

mod color;
mod command_line;
mod show_color;
mod show_term_colors;

/// Entry point for the application.
///
/// It uses `anyhow` everywhere to easily propagate errors. Most errors are not
/// recoverable and simply need to be reported. Rusts runtime handles this
/// automatically, when an error is returned from `main()`.
fn main() -> Result<()> {
    match command_line::parse_args()? {
        Input::Terminal => {
            show_term_colors::show_term_colors()?;
        }
        Input::Libraries => {
            use command_line::{APP_NAME, APP_VERSION, DEPENDENCIES};
            println!("{} v{}\n{}", APP_NAME, APP_VERSION, DEPENDENCIES);
        }
        // TODO: Refactor the following two match arms into a single one
        Input::ColorInput {
            input: ColorInput::HexOrHtml(color),
            output,
            text,
            size,
        } => {
            show_color::show_hex_or_html(&color, output, size, text)?;
        }
        Input::ColorInput {
            input: ColorInput::Color(color),
            output,
            text,
            size,
        } => {
            show_color::show(color, output, size, text)?;
        }
    }

    Ok(())
}
