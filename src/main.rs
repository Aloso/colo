#![deny(unsafe_code)]

use anyhow::Result;
use color::{hex, html};
use color_space::ToRgb;
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
        Input::ColorOutput {
            input,
            output,
            size,
        } => match input {
            ColorInput::HexOrHtml(color) => {
                show_color::show_hex_or_html(&color, output, size)?;
            }
            ColorInput::Color(color) => {
                show_color::show(color, output, size)?;
            }
        },

        Input::TextOutput {
            input,
            mut text,
            bold,
            italic,
            underlined,
            no_newline,
        } => {
            if !no_newline {
                text.push('\n');
            }
            match input {
                ColorInput::HexOrHtml(color) => {
                    let color = html::get(&color).map_or_else(|| hex::parse(&color), Ok)?;
                    show_color::show_text(color.to_rgb(), None, text, italic, bold, underlined)?;
                }
                ColorInput::Color(color) => {
                    show_color::show_text(color.to_rgb(), None, text, italic, bold, underlined)?;
                }
            }
        }
    }

    Ok(())
}
