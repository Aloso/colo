use std::{error::Error, fmt, num::ParseFloatError};

use super::{
    hex::{self, ParseHexError},
    html, Color, ColorSpace,
};
use ParseError::*;

/// Error caused by parsing a number in a certain color space.
///
/// This error can occur if the wrong number of color components
/// was supplied (e.g. `rgb` with only 2 components), or if a
/// color component is out of range (for example, `rgb` requires
/// that all components are in 0..=255).
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    NumberOfComponents {
        expected: usize,
        got: usize,
    },
    Negative {
        component: &'static str,
        got: f64,
    },
    OutOfRange {
        component: &'static str,
        min: f64,
        max: f64,
        got: f64,
    },

    InvalidFloat {
        string: String,
        cause: ParseFloatError,
    },
    MissingFloat {
        got: String,
    },
    UnclosedParen {
        open: char,
        expected: char,
        string: String,
    },
    ExpectedWord {
        string: String,
    },

    ParseHexError(ParseHexError),
}

impl From<ParseHexError> for ParseError {
    fn from(err: ParseHexError) -> Self {
        ParseError::ParseHexError(err)
    }
}

impl Error for ParseError {}

// Note that this could be simplified with `thiserror`, but I'm currently
// reluctant to add more dependencies
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &NumberOfComponents { expected, got } => write!(
                f,
                "Wrong number of color components (expected {}, got {})",
                expected, got
            ),
            &Negative { component, got } => write!(
                f,
                "Color component {:?} can't be negative (got {})",
                component, got
            ),
            &OutOfRange {
                component,
                min,
                max,
                got,
            } => write!(
                f,
                "Color component {:?} out of range (expected {} to {}, got {})",
                component, min, max, got
            ),
            InvalidFloat { string, cause } => write!(
                f,
                "{:?} could not be parsed as a number. Reason: {}",
                string, cause
            ),
            MissingFloat { got } => write!(f, "Expected a number, got {:?}", got),
            UnclosedParen {
                open,
                expected,
                string,
            } => write!(
                f,
                "Unclosed {:?} paren, expected {:?} at {:?}",
                open, expected, string
            ),
            ExpectedWord { string } => {
                write!(f, "Expected hex color or HTML color, got {:?}", string)
            }
            ParseError::ParseHexError(e) => fmt::Display::fmt(e, f),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ColorFormat {
    Html,
    Hex { digits_per_component: u8 },
    Components { color_space_set: bool },
}

/// Parses a string that can contain an arbitrary number of colors in different
/// formats
pub fn parse(mut input: &str) -> Result<Vec<(Color, ColorFormat)>, ParseError> {
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

        if let Some((n1, mut input_ii)) = parse_number(input_i)? {
            let expected = match cs {
                Some(ColorSpace::Cmyk) => 4,
                _ => 3,
            };
            let mut nums = [n1, 0.0, 0.0, 0.0];

            for num in nums.iter_mut().take(expected).skip(1) {
                input_ii = input_ii.trim_start();
                input_ii = skip(input_ii, ',');
                input_ii = input_ii.trim_start();
                let (n, input_iii) = parse_number(input_ii)?.ok_or_else(|| MissingFloat {
                    got: input_ii.into(),
                })?;
                *num = n;
                input_ii = input_iii;
            }

            let nums = &nums[0..expected];
            let color: Color =
                Color::new(cs.unwrap_or(ColorSpace::Rgb), nums).map_err(|err| err)?;
            output.push((
                color,
                ColorFormat::Components {
                    color_space_set: cs.is_some(),
                },
            ));
            input_ii = input_ii.trim_start();
            input_i = input_ii;
        } else {
            let (word, input_ii) = take_word(input_i).ok_or_else(|| ParseError::ExpectedWord {
                string: input_i.into(),
            })?;
            let color = if let Some(color) = html::get(word) {
                (Color::Rgb(color), ColorFormat::Html)
            } else {
                let digits_per_component = (word.chars().filter(|&c| c != '_').count() / 3) as u8;
                (
                    Color::Rgb(hex::parse(word)?),
                    ColorFormat::Hex {
                        digits_per_component,
                    },
                )
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
    let num = num.parse().map_err(|cause| InvalidFloat {
        string: num.into(),
        cause,
    })?;
    Ok(Some((num, rest)))
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
