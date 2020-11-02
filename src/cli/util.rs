use anyhow::Result;
use clap::ArgMatches;
use std::iter;

use crate::color::{self, Color, ColorFormat, ColorSpace, ParseError};

pub(super) fn get_color_space(matches: &ArgMatches, arg_name: &str) -> Result<Option<ColorSpace>> {
    Ok(matches
        .value_of(arg_name)
        .map(|v| v.to_lowercase().parse())
        .transpose()?)
}

pub(super) fn values_to_colors(
    values: clap::Values,
) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
    let color_input: String = values
        .flat_map(|s| iter::once(s).chain(iter::once(" ")))
        .collect();
    color::parse(&color_input)
}
