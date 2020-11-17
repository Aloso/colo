use anyhow::Result;
use colored::Colorize;
use console::{Key, Term};
use std::{
    fmt,
    io::{self, Write},
};

use crate::color::{Color, TextColor};

#[derive(Clone)]
pub struct ThreeLines {
    name: &'static str,
    components: Vec<InputLine>,
    current: usize,
    typing: Option<String>,
    from_color: fn(Color) -> Vec<f64>,
    to_color: fn(&[f64]) -> Color,
}

impl ThreeLines {
    pub fn new(
        name: &'static str,
        from_color: fn(Color) -> Vec<f64>,
        to_color: fn(&[f64]) -> Color,
        components: &[InputLine],
    ) -> Self {
        assert!(!components.is_empty());
        Self {
            name,
            components: components.iter().copied().collect(),
            current: 0,
            typing: None,
            from_color,
            to_color,
        }
    }

    pub fn values(&self) -> Vec<f64> {
        self.components.iter().map(|c| c.value).collect()
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn inc(&mut self) {
        self.current_mut().inc();
        self.typing = None;
    }

    pub fn dec(&mut self) {
        self.current_mut().dec();
        self.typing = None;
    }

    pub fn set_max(&mut self) {
        self.current_mut().set_max();
        self.typing = None;
    }

    pub fn set_min(&mut self) {
        self.current_mut().set_min();
        self.typing = None;
    }

    pub fn reset(&mut self) {
        self.current_mut().reset();
        self.typing = None;
    }

    pub fn set_current(&mut self, c: usize) {
        assert!(c < self.len());
        if c != self.current {
            self.typing = None;
        }
        self.current = c;
    }

    pub fn inc_current(&mut self) {
        self.current = (self.current + 1) % self.len();
        self.typing = None;
    }

    pub fn dec_current(&mut self) {
        self.current = (self.current + self.len() - 1) % self.len();
        self.typing = None;
    }

    pub fn set(&mut self, v: f64) {
        let in_bounds_and_whole = self.current_mut().set(v);
        if !in_bounds_and_whole {
            self.typing = None;
        }
    }

    pub fn type_char(&mut self, c: char) {
        if let Some(mut s) = self.typing.take() {
            s.push(c);
            if let Some(v) = (self.current().from_str)(&s) {
                self.typing = Some(s);
                self.set(v);
            } else {
                s.pop();
                self.typing = Some(s);
            }
        } else {
            let s = c.to_string();
            if let Some(v) = (self.current().from_str)(&s) {
                self.typing = Some(s);
                self.set(v);
            }
        }
    }

    pub fn backspace(&mut self) {
        if let Some(mut s) = self.typing.take() {
            s.pop();
            if s.is_empty() {
                self.reset();
            } else if let Some(v) = (self.current().from_str)(&s) {
                self.typing = Some(s);
                self.set(v);
            } else {
                self.typing = Some(s);
            }
        } else {
            self.reset();
        }
    }

    pub fn print(&self, color: Color, mut term: &Term) -> Result<()> {
        let color = color.to_term_color();
        let (_, term_width) = term.size();
        let term_width = (term_width.min(100) - 24) as u8;
        let mut buf = Vec::new();
        writeln!(buf)?;

        write!(buf, " {}  ", "        ".on_color(color))?;
        write!(buf, "{}", self.name,)?;
        if term_width > 60 {
            writeln!(
                buf,
                "    {}",
                "(you can Tab ↹ through the color spaces)".italic().dimmed()
            )?;
        } else {
            writeln!(buf)?;
        }

        for (i, current) in self.components.iter().enumerate() {
            write!(buf, " {}", "        ".on_color(color))?;

            let mut name = current.name.bold();
            if let Some(color) = current.color {
                name = name.color(color);
            }
            write!(buf, "  {}", name)?;
            write!(buf, "  {} ", FmtValue::new(current, i == self.current, 6))?;

            if let Some((min, max)) = current.bounds {
                let dw = (term_width * 2) as f64;

                let highlighted = (current.value * dw / (max - min)) as u8;
                let highlighted = highlighted.max(0).min(dw as u8 - 1);

                let mut left_color = self.values();
                left_color[i] = min;
                let left_color = (self.to_color)(&left_color);
                write!(buf, "{}", "▕".color(left_color.to_term_color()))?;

                for j in 0..term_width {
                    let v1 = max * ((2 * j) as f64 / dw);
                    let v2 = max * ((2 * j + 1) as f64 / dw);

                    let mut vals1 = self.values();
                    let mut vals2 = vals1.clone();
                    vals1[i] = v1;
                    vals2[i] = v2;
                    let c1 = (self.to_color)(&vals1);
                    let c2 = (self.to_color)(&vals2);
                    let tc1 = c1.to_term_color();
                    let tc2 = c2.to_term_color();

                    if j == highlighted / 2 {
                        let t = match c1.text_color() {
                            TextColor::Black => colored::Color::Black,
                            TextColor::White => colored::Color::BrightWhite,
                        };
                        if highlighted % 2 == 0 {
                            write_gradient_char(&mut buf, t, tc2)?;
                        } else {
                            write_gradient_char(&mut buf, tc1, t)?;
                        }
                    } else {
                        write_gradient_char(&mut buf, tc1, tc2)?;
                    }
                }
                let mut right_color = self.values();
                right_color[i] = max;
                let right_color = (self.to_color)(&right_color);
                write!(buf, "{}", "▏".color(right_color.to_term_color()))?;
            }
            writeln!(buf)?;
        }

        fn write_gradient_char(
            mut buf: impl Write,
            c1: colored::Color,
            c2: colored::Color,
        ) -> io::Result<()> {
            if c1 == c2 {
                write!(buf, "{}", "█".color(c1))
            } else {
                write!(buf, "{}", "▌".color(c1).on_color(c2))
            }
        }

        term.write_all(&buf)?;
        Ok(())
    }

    pub fn reset_term(&self, term: &Term) -> Result<()> {
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
        term.move_cursor_up(1)?;
        term.clear_line()?;
        Ok(())
    }

    pub fn enter_key(&mut self, key: Key) {
        match key {
            Key::ArrowUp => self.dec_current(),
            Key::ArrowDown => self.inc_current(),
            Key::ArrowRight => self.inc(),
            Key::ArrowLeft => self.dec(),
            Key::Backspace => self.backspace(),
            Key::Del => self.reset(),
            Key::Home => self.set_current(0),
            Key::End => self.set_current(self.len() - 1),
            Key::PageUp => self.set_max(),
            Key::PageDown => self.set_min(),
            Key::Char(c) => self.type_char(c),
            Key::Enter | Key::Escape | Key::Tab | Key::BackTab => {}
            Key::Insert | Key::Unknown | Key::UnknownEscSeq(_) => {}
            _ => {}
        }
    }

    pub fn color(&self) -> Color {
        (self.to_color)(&self.values())
    }

    pub fn set_color(&mut self, color: Color) {
        let values = (self.from_color)(color);
        for (v, c) in values.into_iter().zip(&mut self.components) {
            c.set(v);
        }
    }

    fn current(&self) -> &InputLine {
        &self.components[self.current]
    }

    fn current_mut(&mut self) -> &mut InputLine {
        &mut self.components[self.current]
    }
}

#[derive(Copy, Clone)]
pub struct InputLine {
    name: &'static str,
    value: f64,
    bounds: Option<(f64, f64)>,
    initial: f64,
    color: Option<colored::Color>,
    step: f64,
    to_string: fn(f64) -> String,
    from_str: fn(&str) -> Option<f64>,
}

impl InputLine {
    pub fn new(name: &'static str, value: f64) -> Self {
        fn to_string(v: f64) -> String {
            round10(v).to_string()
        }
        fn from_str(s: &str) -> Option<f64> {
            let s = if s.ends_with('.') {
                &s[..s.len() - 1]
            } else {
                s
            };
            s.parse::<f64>().ok()
        }

        Self {
            name,
            value,
            initial: value,
            bounds: None,
            color: None,
            step: 1.0,
            to_string,
            from_str,
        }
    }

    pub fn with_bounds(mut self, min: f64, max: f64) -> Self {
        assert!(min <= max);
        self.bounds = Some((min, max));
        self
    }

    pub fn with_color(mut self, c: colored::Color) -> Self {
        self.color = Some(c);
        self
    }

    pub fn with_color_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_color(colored::Color::TrueColor { r, g, b })
    }

    pub fn as_percent(mut self) -> Self {
        fn to_string(v: f64) -> String {
            format!("{}%", round10(v * 100.0))
        }
        fn from_str(s: &str) -> Option<f64> {
            let s = if s.ends_with('.') {
                &s[..s.len() - 1]
            } else {
                s
            };
            s.parse::<f64>().map(|n| n / 100.0).ok()
        }

        self.step = 0.01;
        self.to_string = to_string;
        self.from_str = from_str;
        self
    }

    pub fn inc(&mut self) {
        let old = self.value;
        self.value += self.step;
        if let Some((min, max)) = self.bounds {
            if self.value > max {
                if old < max {
                    self.value = max;
                } else {
                    self.value = min;
                }
            }
        }
    }

    pub fn dec(&mut self) {
        let old = self.value;
        self.value -= self.step;
        if let Some((min, max)) = self.bounds {
            if self.value < min {
                if old > min {
                    self.value = min;
                } else {
                    self.value = max;
                }
            }
        }
    }

    /// Returns whether the value was within bounds and
    /// doesn't have too many decimals
    pub fn set(&mut self, v: f64) -> bool {
        if let Some((min, max)) = self.bounds {
            if min < v && v < max {
                self.value = v;
                ((v / self.step) % 1.0).abs() < 0.001
            } else {
                self.value = v.max(min).min(max);
                false
            }
        } else {
            self.value = v;
            true
        }
    }

    pub fn reset(&mut self) {
        self.value = self.initial;
    }

    pub fn set_min(&mut self) {
        if let Some((min, _)) = self.bounds {
            self.value = min;
        }
    }

    pub fn set_max(&mut self) {
        if let Some((_, max)) = self.bounds {
            self.value = max;
        }
    }
}

struct FmtValue<'a> {
    component: &'a InputLine,
    is_current: bool,
    width: usize,
}

impl<'a> FmtValue<'a> {
    fn new(component: &'a InputLine, is_current: bool, width: usize) -> Self {
        Self {
            component,
            is_current,
            width,
        }
    }
}

impl fmt::Display for FmtValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.component;
        let s = (c.to_string)(c.value);
        let s = if self.is_current {
            s.reverse()
        } else {
            s.as_str().into()
        };
        write!(f, "{:width$}", s, width = self.width)
    }
}

fn round10(f: f64) -> f64 {
    (f * 10.0).round() / 10.0
}
