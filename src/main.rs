use anyhow::Result;
use command_line::Input;

mod color;
mod command_line;
mod show_color;
mod show_term_colors;

fn main() -> Result<()> {
    match command_line::parse_args()? {
        Input::Terminal => {
            show_term_colors::show_term_colors()?;
        }
        Input::ColorString(color, out) => {
            show_color::show_hex_or_html(&color, out)?;
        }
        Input::Color(color, out) => {
            show_color::show(color, out)?;
        }
    }

    Ok(())
}
