use anyhow::{Error, Result};
use clap::ArgMatches;
use std::{io::Read, iter};

use crate::color::{self, Color, ColorFormat, ParseError};

pub(super) fn get_color_format(
    matches: &ArgMatches,
    arg_name: &str,
) -> Result<Option<ColorFormat>> {
    Ok(matches
        .value_of(arg_name)
        .map(|v| match v.to_lowercase().as_str() {
            "html" => Ok::<_, Error>(ColorFormat::Html),
            "hex" => Ok(ColorFormat::Hex),
            s => Ok(ColorFormat::Normal(s.parse()?)),
        })
        .transpose()?)
}

pub(super) fn values_to_colors<'a>(
    values: impl Iterator<Item = &'a str>,
) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
    let color_input: String = values
        .flat_map(|s| iter::once(s).chain(iter::once(" ")))
        .collect();
    color::parse(&color_input)
}

pub(super) fn read_stdin() -> Result<String> {
    let mut text = Vec::new();
    std::io::stdin().read_to_end(&mut text)?;
    let mut text = String::from_utf8(text)?;
    if text.ends_with('\n') {
        text.truncate(text.len() - 1);
    }
    Ok(text)
}
