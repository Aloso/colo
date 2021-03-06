use anyhow::anyhow;
use std::{cmp::Ordering, num::ParseFloatError};
use thiserror::Error;

use super::{hex, html, Color, ColorFormat, ColorSpace};
use crate::{
    terminal::{stdin, ColorPicker},
    State,
};

use ParseError::*;

/// Error caused by parsing a number in a certain color space.
///
/// This error can occur if the wrong number of color components
/// was supplied (e.g. `rgb` with only 2 components), or if a
/// color component is out of range (for example, `rgb` requires
/// that all components are in 0..=255).
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ParseError {
    #[error("Wrong number of color components (expected {expected}, got {got})")]
    NumberOfComponents { expected: usize, got: usize },
    #[error("Color component {component:?} can't be negative (got {got})")]
    Negative { component: &'static str, got: f64 },
    #[error("Color component {component:?} out of range (expected {min} to {max}, got {got})")]
    OutOfRange {
        component: &'static str,
        min: f64,
        max: f64,
        got: f64,
    },
    #[error("{string:?} could not be parsed as a number. Reason: {cause}")]
    InvalidFloat {
        string: String,
        cause: ParseFloatError,
    },
    #[error("Expected a number, got {got:?}")]
    MissingFloat { got: String },
    #[error("Unclosed {open:?} paren, expected {expected:?} at {string:?}")]
    UnclosedParen {
        open: char,
        expected: char,
        string: String,
    },
    #[error("Expected hex color or HTML color, got {string:?}")]
    ExpectedWord { string: String },
    #[error(transparent)]
    ParseHexError(#[from] hex::ParseHexError),

    #[error("Unknown color {got:?}, did you mean {suggestion:?}?")]
    Misspelled { got: String, suggestion: String },

    #[error("This value in the {cs:?} color space can't be randomly generated")]
    UnsupportedRand { cs: ColorSpace },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Parses a string that can contain an arbitrary number of colors in different
/// formats
pub fn parse(mut input: &str, state: State) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
    let mut output = Vec::new();
    loop {
        let input_i = input.trim_start();
        if input_i.is_empty() {
            return Ok(output);
        }

        let (cs, input_i) = parse_color_space(input_i);
        let input_i = input_i.trim_start();
        let (open_paren, input_i) = open_paren(input_i);
        let mut input_i = input_i.trim_start();

        if let Some(cs) = cs {
            let expected = cs.num_components();
            let mut nums = [0.0, 0.0, 0.0, 0.0];

            for (i, num) in nums.iter_mut().enumerate().take(expected) {
                input_i = input_i.trim_start();
                input_i = skip(input_i, ',');
                input_i = input_i.trim_start();
                let (n, input_ii) = parse_number(input_i)?
                    .map(Ok)
                    .or_else(|| parse_rand_component(input_i, cs, i).transpose())
                    .transpose()?
                    .ok_or_else(|| MissingFloat {
                        got: input_i.into(),
                    })?;
                *num = n;

                input_i = input_ii.trim_start();
            }

            let nums = &nums[0..expected];
            let color: Color = Color::new(cs, nums).map_err(|err| err)?;
            output.push((color, ColorFormat::Normal(cs)));
            input_i = input_i.trim_start();
        } else if input_i.starts_with("- ") || input_i.starts_with("-,") {
            input_i = input_i[2..].trim_start();

            let new_values = stdin::read_line(state)?;
            let colors = parse(&new_values, state)?;
            if colors.len() != 1 {
                return Err(anyhow!("Expected 1 color, got {}", colors.len()).into());
            }
            output.push(colors[0]);
        } else {
            let (word, input_ii) = take_word(input_i).ok_or_else(|| ParseError::ExpectedWord {
                string: input_i.into(),
            })?;

            let color = if word == "pick" {
                let color = ColorPicker::new(None, None).display(state)?;
                (color, color.get_color_format())
            } else if word == "rand" {
                (Color::random_rgb(), ColorFormat::Hex)
            } else if let Some(color) = html::get(word) {
                (Color::Rgb(color), ColorFormat::Html)
            } else {
                match hex::parse(word) {
                    Ok(hex) => (Color::Rgb(hex), ColorFormat::Hex),
                    Err(err) => {
                        if word.chars().all(|c| c.is_ascii_alphabetic()) && word.len() > 3 {
                            let mut similar = html::get_similar(word);
                            if !similar.is_empty() {
                                similar.sort_by(|&(_, l), &(_, r)| {
                                    if l < r {
                                        Ordering::Less
                                    } else if l > r {
                                        Ordering::Greater
                                    } else {
                                        Ordering::Equal
                                    }
                                });
                                let (suggestion, _) = similar[similar.len() - 1];
                                return Err(ParseError::Misspelled {
                                    got: word.to_string(),
                                    suggestion: suggestion.to_string(),
                                });
                            }
                        }
                        return Err(err.into());
                    }
                }
            };
            output.push(color);

            let input_ii = input_ii.trim_start();
            input_i = input_ii;
        }

        if let Some(open) = open_paren {
            input_i = close_paren(input_i, open)?;
        }
        input_i = skip(input_i, ',');

        input = input_i;
    }
}

fn parse_color_space(input: &str) -> (Option<ColorSpace>, &str) {
    if let Some((word, rest)) = take_word(input) {
        if let Ok(cs) = word.parse::<ColorSpace>() {
            return (Some(cs), rest);
        }
    }
    (None, input)
}

fn open_paren(input: &str) -> (Option<char>, &str) {
    let mut chars = input.chars();
    match chars.next() {
        Some(c) if c == '(' || c == '[' || c == '{' => (Some(c), chars.as_str()),
        _ => (None, input),
    }
}

fn close_paren(input: &str, open: char) -> Result<&str, ParseError> {
    let expected = closing_for(open);
    let mut chars = input.chars();
    match chars.next() {
        Some(c) if c == expected => Ok(chars.as_str()),
        _ => Err(ParseError::UnclosedParen {
            open,
            expected,
            string: input.into(),
        }),
    }
}

fn closing_for(c: char) -> char {
    match c {
        '<' => '>',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        c => c,
    }
}

fn parse_number(input: &str) -> Result<Option<(f64, &str)>, ParseError> {
    let (num, rest) = take_until(input, |c| !matches!(c, '0'..='9' | '.' | '_' | '-'));
    if num.is_empty() {
        return Ok(None);
    }
    let mut num = num.parse().map_err(|cause| InvalidFloat {
        string: num.into(),
        cause,
    })?;
    let mut rest = rest.trim_start();
    if rest.starts_with('%') {
        rest = &rest[1..];
        num /= 100.0;
    }
    Ok(Some((num, rest)))
}

fn parse_rand_component(
    input: &str,
    cs: ColorSpace,
    i: usize,
) -> Result<Option<(f64, &str)>, ParseError> {
    if let Some((word, rest)) = take_word(input) {
        if word == "rand" {
            return Ok(Some((
                match cs {
                    ColorSpace::Rgb => fastrand::u8(..) as f64,
                    ColorSpace::Cmy => fastrand::f64(),
                    ColorSpace::Cmyk => fastrand::f64(),
                    ColorSpace::Hsv | ColorSpace::Hsl => match i {
                        0 => fastrand::u32(0..360) as f64,
                        _ => fastrand::f64(),
                    },
                    ColorSpace::Lch => match i {
                        0 | 1 => fastrand::u32(0..=100) as f64,
                        _ => fastrand::u32(0..360) as f64,
                    },
                    ColorSpace::Luv => match i {
                        0 | 1 => fastrand::u32(0..=100) as f64,
                        _ => fastrand::u32(0..=360) as f64,
                    },
                    ColorSpace::Lab if i == 0 => fastrand::u32(0..=100) as f64,
                    ColorSpace::HunterLab if i == 0 => fastrand::u32(0..=100) as f64,
                    ColorSpace::Xyz if i == 1 => fastrand::u32(0..=100) as f64,
                    ColorSpace::Yxy if i == 0 => fastrand::u32(0..=100) as f64,
                    ColorSpace::Gray => fastrand::f64(),
                    _ => return Err(ParseError::UnsupportedRand { cs }),
                },
                rest,
            )));
        }
    }
    Ok(None)
}

fn take_word(input: &str) -> Option<(&str, &str)> {
    let res = take_until(input, |c| {
        !(c.is_ascii_alphanumeric() || c == '_' || c == '#')
    });
    Some(res).filter(|(word, _)| !word.is_empty())
}

fn take_until(input: &str, f: impl FnMut(char) -> bool) -> (&str, &str) {
    let next = input.split(f).next().unwrap_or("");
    let rest = &input[next.len()..];
    (next, rest)
}

fn skip(input: &str, c: char) -> &str {
    let mut chars = input.chars();
    match chars.next() {
        Some(char) if char == c => chars.as_str(),
        _ => input,
    }
}
