use anyhow::{bail, Result};
use logos::Logos;

use super::super::filter::{FilterKey, FilterList, FilterValue};
use crate::color::ColorSpace;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[token("brightness", |_| FilterKey::Brightness)]
    #[token("contrast", |_| FilterKey::Contrast)]
    #[token("grayscale", |_| FilterKey::Grayscale)]
    #[token("greyscale", |_| FilterKey::Grayscale)]
    #[token("hue-rotate", |_| FilterKey::HueRotate)]
    #[token("rotate", |_| FilterKey::HueRotate)]
    #[token("invert", |_| FilterKey::Invert)]
    #[token("saturate", |_| FilterKey::Saturate)]
    #[token("sepia", |_| FilterKey::Sepia)]
    CssFilter(FilterKey),

    #[token("rgb:", |_| ColorSpace::Rgb)]
    #[token("cmy:", |_| ColorSpace::Cmy)]
    #[token("cmyk:", |_| ColorSpace::Cmyk)]
    #[token("hsl:", |_| ColorSpace::Hsl)]
    #[token("hsv:", |_| ColorSpace::Hsv)]
    #[token("lab:", |_| ColorSpace::Lab)]
    #[token("lch:", |_| ColorSpace::Lch)]
    #[token("luv:", |_| ColorSpace::Luv)]
    #[token("hunterlab:", |_| ColorSpace::HunterLab)]
    #[token("xyz:", |_| ColorSpace::Xyz)]
    #[token("yxy:", |_| ColorSpace::Yxy)]
    #[token("gry:", |_| ColorSpace::Gray)]
    ColorSpace(ColorSpace),

    #[regex(r"[a-zA-Z]\w*")]
    ColorComponent(&'a str),

    #[regex(r"[0-9]*\.[0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Value(f64),

    #[token("=", |_| Modifier::Set)]
    #[token("+", |_| Modifier::Add)]
    #[token("-", |_| Modifier::Sub)]
    #[token("\\-", |_| Modifier::Sub)]
    #[token("*", |_| Modifier::Mul)]
    #[token("/", |_| Modifier::Div)]
    Modifier(Modifier),

    #[token("%")]
    Percent,

    #[token(",")]
    Comma,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Modifier {
    Set,
    Add,
    Sub,
    Mul,
    Div,
}

enum State {
    Start {
        allow_comma: bool,
    },
    ColorSpace(ColorSpace),
    Key {
        key: FilterKey,
        paren: bool,
    },
    Modifier {
        key: FilterKey,
        m: Modifier,
        paren: bool,
    },
    Value {
        key: FilterKey,
        value: FilterValue,
        paren: bool,
        percent: bool,
    },
}

pub fn parse_filter(input: &str) -> Result<FilterList> {
    let mut items = Vec::new();
    let mut state = State::Start { allow_comma: false };
    let mut lex = Token::lexer(input);

    while let Some(token) = lex.next() {
        state = match state {
            State::Start { allow_comma } => match token {
                Token::CssFilter(key) => State::Key { key, paren: false },
                Token::ColorSpace(s) => State::ColorSpace(s),
                Token::Comma if allow_comma => State::Start { allow_comma: false },
                Token::ColorComponent(c) => {
                    let s = match c {
                        "r" | "g" | "b" => ColorSpace::Rgb,
                        "h" | "s" | "l" => ColorSpace::Hsl,
                        _ => bail!("Unknown color component {:?}", c),
                    };
                    State::Key {
                        key: FilterKey::Other(s, c.into()),
                        paren: false,
                    }
                }
                _ => bail!("Expected filter, got {:?}", lex.slice()),
            },
            State::ColorSpace(s) => match token {
                Token::ColorComponent(c) => State::Key {
                    key: FilterKey::Other(s, c.into()),
                    paren: false,
                },
                _ => bail!("Expected color component, got {:?}", lex.slice()),
            },
            State::Key { key, paren } => match token {
                Token::Value(v) => State::Value {
                    key,
                    value: FilterValue::Add(v),
                    paren,
                    percent: false,
                },
                Token::Modifier(m) => State::Modifier { key, m, paren },
                Token::OpenParen if !paren => State::Key { key, paren: true },
                _ => bail!("Expected value or operator, got {:?}", lex.slice()),
            },
            State::Modifier { key, m, paren } => match token {
                Token::Value(v) => {
                    let value = match m {
                        Modifier::Set => FilterValue::Set(v),
                        Modifier::Add => FilterValue::Add(v),
                        Modifier::Sub => FilterValue::Add(-v),
                        Modifier::Mul => FilterValue::Mul(v),
                        Modifier::Div => FilterValue::Div(v),
                    };
                    State::Value {
                        key,
                        value,
                        paren,
                        percent: false,
                    }
                }
                _ => bail!("Expected value, got {:?}", lex.slice()),
            },
            State::Value {
                key,
                value,
                paren,
                percent,
            } => match token {
                Token::CssFilter(f) if !paren => {
                    items.push((key, value));
                    State::Key {
                        key: f,
                        paren: false,
                    }
                }
                Token::ColorSpace(s) if !paren => {
                    items.push((key, value));
                    State::ColorSpace(s)
                }
                Token::Comma if !paren => {
                    items.push((key, value));
                    State::Start { allow_comma: false }
                }
                Token::CssFilter(_) | Token::ColorSpace(_) | Token::Comma => {
                    bail!("Expected `)`, got {:?}", lex.slice());
                }
                Token::ColorComponent(c) => {
                    items.push((key, value));
                    let s = match c {
                        "r" | "g" | "b" => ColorSpace::Rgb,
                        "h" | "s" | "l" => ColorSpace::Hsl,
                        _ => bail!("Unknown color component {:?}", c),
                    };
                    State::Key {
                        key: FilterKey::Other(s, c.into()),
                        paren: false,
                    }
                }
                Token::CloseParen if paren => {
                    items.push((key, value));
                    State::Start { allow_comma: true }
                }
                Token::Percent if !percent => State::Value {
                    key,
                    value: match value {
                        FilterValue::Add(n) => FilterValue::Add(n / 100.0),
                        FilterValue::Mul(n) => FilterValue::Mul(n / 100.0),
                        FilterValue::Div(n) => FilterValue::Div(n / 100.0),
                        FilterValue::Set(n) => FilterValue::Set(n / 100.0),
                    },
                    paren,
                    percent: true,
                },
                _ => bail!("Expected filter, got {:?}", lex.slice()),
            },
        };
    }

    match state {
        State::Start { allow_comma: false } => {
            bail!("Unexpected trailing comma")
        }
        State::ColorSpace(_) => {
            bail!("Expected color component, got EOF")
        }
        State::Key { .. } | State::Modifier { .. } => {
            bail!("Expected value, got EOF")
        }
        State::Value {
            key, value, paren, ..
        } => {
            if paren {
                bail!("Unclosed parenthesis");
            }
            items.push((key, value));
        }
        _ => {}
    }

    Ok(FilterList { items })
}
