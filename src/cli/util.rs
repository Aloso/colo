use anyhow::{Error, Result};
use clap::{ArgMatches, Values};
use std::iter;

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

pub(super) fn values_to_colors(values: Values) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
    let color_input: String = values
        .flat_map(|s| iter::once(s).chain(iter::once(" ")))
        .collect();
    color::parse(&color_input)
}
