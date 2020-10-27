use anyhow::Result;
use command_line::Input;

mod colors;
mod command_line;
mod html_colors;
mod show_color;
mod show_term_colors;

fn main() -> Result<()> {
    match command_line::parse_args()? {
        Input::Terminal => {
            show_term_colors::show_term_colors()?;
        }
        Input::ColorString(color) => {
            show_color::show_hex_or_html_color(&color)?;
        }
        Input::Color(color_space, values) => {
            show_color::show_color(color_space, &values)?;
        }
    }

    Ok(())
}
