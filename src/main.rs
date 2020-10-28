use anyhow::Result;
use command_line::{ColorInput, Input};

mod color;
mod command_line;
mod show_color;
mod show_term_colors;

fn main() -> Result<()> {
    match command_line::parse_args()? {
        Input::Terminal => {
            show_term_colors::show_term_colors()?;
        }
        Input::ColorInput {
            input: ColorInput::HexOrHtml(color),
            output,
            size,
        } => {
            show_color::show_hex_or_html(&color, output, size)?;
        }
        Input::ColorInput {
            input: ColorInput::Color(color),
            output,
            size,
        } => {
            show_color::show(color, output, size)?;
        }
    }

    Ok(())
}
