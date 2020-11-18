use anyhow::{Context, Error, Result};
use clap::ArgMatches;
use std::iter;

use crate::{
    color::{self, Color, ColorFormat, ParseError},
    State,
};

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
    state: State,
) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
    let color_input: String = values
        .flat_map(|s| iter::once(s).chain(iter::once(" ")))
        .collect();
    color::parse(&color_input, state)
}

/// Parse a u32
pub(super) fn parse_size(s: &str) -> Result<u32> {
    s.parse()
        .with_context(|| format!("The size {:?} could not be parsed", s))
}
