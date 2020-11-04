#![deny(unsafe_code)]

use anyhow::Result;
use color::html;
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
    let app = cli::app();
    match app.get_matches().subcommand() {
        ("libs", Some(matches)) => {
            use cli::{APP_NAME, APP_VERSION, DEPENDENCIES};
            let cli::Libs = cli::get_libs(&matches)?;
            println!("{} v{}\n{}", APP_NAME, APP_VERSION, DEPENDENCIES);
        }
        ("term", Some(matches)) => {
            let cli::Term = cli::get_term(matches)?;
            show_term_colors::show_term_colors()?;
        }
        ("print", Some(matches)) => {
            let cli::Print {
                color: (color, _),
                bg_color,
                mut text,
                bold,
                italic,
                underline,
                no_newline,
            } = cli::get_print(matches)?;

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
        }
        ("show", Some(matches)) => {
            let cli::Show {
                colors,
                output,
                size,
            } = cli::get_show(&matches)?;

            println!();
            for (color, input) in colors {
                show_color::show(color, input, output, size)?;
            }
        }
        ("list", Some(matches)) => {
            let cli::List = cli::get_list(&matches)?;

            html::show_all()?;
        }
        _ => {
            cli::app().print_help().unwrap();
        }
    }
    Ok(())
}
