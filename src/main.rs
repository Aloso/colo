#![deny(unsafe_code)]

use anyhow::{bail, Result};
use color::ColorFormat;
use color_space::ToRgb;

mod cli;
mod color;
mod show_color;
mod show_term_colors;

/// Entry point for the application.
///
/// It uses `anyhow` everywhere to easily propagate errors. Most errors are not
/// recoverable and simply need to be reported. Rusts runtime handles this
/// automatically, when an error is returned from `main()`.
fn main() -> Result<()> {
    let matches = cli::clap_args();

    if let Some(cli::Libs) = cli::get_libs(&matches) {
        use cli::{APP_NAME, APP_VERSION, DEPENDENCIES};
        println!("{} v{}\n{}", APP_NAME, APP_VERSION, DEPENDENCIES);
    } else if let Some(cli::Print {
        color: (color, _),
        bg_color,
        mut text,
        bold,
        italic,
        underline,
        no_newline,
    }) = cli::get_print(&matches)?
    {
        if !no_newline {
            text.push('\n');
        }
        show_color::show_text(
            color.to_rgb(),
            bg_color.map(|(c, _)| c.to_rgb()),
            text,
            italic,
            bold,
            underline,
        )?;
    } else if let Some(cli::Term) = cli::get_term(&matches) {
        show_term_colors::show_term_colors()?;
    } else if let Some(cli::Show {
        colors,
        output,
        size,
    }) = cli::get_show(&matches)?
    {
        for (color, color_format) in colors {
            match color_format {
                ColorFormat::Hex { .. } | ColorFormat::Html => {
                    show_color::show_hex_or_html(color, output, size)?;
                }
                ColorFormat::Components { .. } => {
                    show_color::show(color, output, size)?;
                }
            }
        }
    } else {
        bail!("No input provided");
    }

    Ok(())
}
