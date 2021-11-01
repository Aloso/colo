use anyhow::{Context, Result};
use color_space::{Rgb, ToRgb};
use std::io::{stdout, Write};

use crate::color::{Color, ColorFormat};

pub(crate) fn print_text(mut text: &str, colors: &[(Color, ColorFormat)]) -> Result<()> {
    let mut buf = Vec::with_capacity(text.len() * 3 / 2);

    let mut color_idx = 0;

    while let Some(next_index) = text.find(|c| matches!(c, '\\' | '%')) {
        if next_index > 0 {
            let (before, after) = text.split_at(next_index);
            buf.extend(before.as_bytes());
            text = after;
        }

        let mut chars = text.chars();
        match chars.next().unwrap() {
            '\\' => {
                match chars.next() {
                    Some('%') => buf.push(b'%'),
                    Some('\\') | None => buf.push(b'\\'),
                    Some('a') => buf.push(b'\x07'),
                    Some('b') => buf.push(b'\x08'),
                    Some('e') => buf.push(b'\x1B'),
                    Some('f') => buf.push(b'\x0C'),
                    Some('n') => buf.push(b'\x0A'),
                    Some('r') => buf.push(b'\x0D'),
                    Some('t') => buf.push(b'\x09'),
                    Some('v') => buf.push(b'\x0B'),
                    Some(c) => buf.write_fmt(format_args!("\\{}", c)).unwrap(),
                }
                text = chars.as_str();
            }
            '%' => {
                match chars.next() {
                    None => buf.push(b'%'),
                    Some('r') => buf.extend(b"\x1B[m"),
                    Some('B') => buf.extend(b"\x1B[1m"),
                    Some('F') => buf.extend(b"\x1B[2m"),
                    Some('I') => buf.extend(b"\x1B[3m"),
                    Some('U') => buf.extend(b"\x1B[4m"),
                    Some('X') => buf.extend(b"\x1B[7m"),
                    Some('S') => buf.extend(b"\x1B[9m"),
                    Some('c') => {
                        let (color, _) =
                            colors.get(color_idx).context("too few colors provided")?;
                        color_idx += 1;
                        let Rgb { r, g, b } = color.to_rgb();
                        buf.write_fmt(format_args!("\x1B[38;2;{};{};{}m", r, g, b))
                            .unwrap();
                    }
                    Some('b') => {
                        let (color, _) =
                            colors.get(color_idx).context("too few colors provided")?;
                        color_idx += 1;
                        let Rgb { r, g, b } = color.to_rgb();
                        buf.write_fmt(format_args!("\x1B[48;2;{};{};{}m", r, g, b))
                            .unwrap();
                    }
                    Some(c) => buf.write_fmt(format_args!("%{}", c)).unwrap(),
                }
                text = chars.as_str();
            }
            _ => unreachable!(),
        }
    }
    buf.extend(text.as_bytes());

    stdout().write_all(&buf)?;
    Ok(())
}
