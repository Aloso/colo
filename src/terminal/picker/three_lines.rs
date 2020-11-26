use anyhow::Result;
use colored::{Color as TermColor, Colorize};
use console::{Key, Term};
use std::{fmt, io, io::Write};

use crate::color::{Color, TextColor};

#[derive(Clone)]
pub struct ThreeLines {
    name: &'static str,
    components: Vec<InputLine>,
    current: usize,
    typing: Option<String>,
    from_color: fn(Color) -> Vec<f64>,
    to_color: fn(&[f64]) -> Color,
    expanded: bool,
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
            expanded: false,
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
        if c == 's' {
            self.expanded = !self.expanded;
        } else if let Some(mut s) = self.typing.take() {
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
        const GRAY: TermColor = TermColor::TrueColor {
            r: 127,
            g: 127,
            b: 127,
        };

        let color = color.to_term_color();
        let (_, term_width) = term.size();
        let term_width = (term_width.min(100) - 24) as u8;
        let mut buf = Vec::new();
        writeln!(buf)?;

        write!(buf, " {}  ", "        ".on_color(color))?;
        writeln!(buf, "{}", self.name)?;

        if !self.expanded {
            for (i, current) in self.components.iter().enumerate() {
                let is_current = i == self.current;
                self.print_gradient_line(&mut buf, color, i, current, is_current, term_width)?;
            }
        } else {
            let is_last = self.current == self.len() - 1;
            let iter = self
                .components
                .iter()
                .enumerate()
                .skip(if is_last { 1 } else { 0 })
                .chain(
                    self.components
                        .first()
                        .filter(|_| is_last)
                        .into_iter()
                        .enumerate(),
                );

            let mut last_current = None;
            for (i, current) in iter {
                if i == self.current {
                    last_current = Some(current);
                } else if let Some(prev) = last_current {
                    let (min_h, max_h) = prev.bounds.unwrap();
                    let (min_v, max_v) = current.bounds.unwrap();
                    last_current = None;

                    let v_highlight = ((current.value - min_v) * 29.0 / (max_v - min_v)) as u32;

                    for j in 0..15 {
                        write!(buf, " {}", "        ".on_color(color))?;
                        if j < 2 {
                            let input = if j == 0 { prev } else { current };
                            let mut name = input.name.bold();
                            if let Some(color) = input.color {
                                name = name.color(color);
                            }
                            write!(buf, "  {}", name)?;
                            write!(buf, "  {}  ", FmtValue::new(input, true, 6))?;
                        } else {
                            write!(buf, "             ")?;
                        }

                        let values = self.values();
                        let v_value = min_v + (max_v - min_v) * j as f64 / 15.0;

                        RectGradientLine {
                            values,
                            to_color: self.to_color,
                            comp: ((i + self.len() - 1) % self.len(), i),
                            v_values: (v_value, v_value + (max_v - min_v) / 30.0),
                            bounds: (min_h, max_h),
                            highlight: Some(v_highlight % 2 == 0).filter(|_| j == v_highlight / 2),
                        }
                        .write(&mut buf, term_width)?;
                        writeln!(buf)?;
                    }
                } else {
                    self.print_gradient_line(&mut buf, color, i, current, false, term_width)?;
                }
            }
        }

        writeln!(buf, "<S>      {}", "Toggle mode".color(GRAY))?;
        writeln!(buf, "<Tab ↹>  {}", "Toggle color space".color(GRAY))?;

        term.write_all(&buf)?;
        Ok(())
    }

    fn print_gradient_line(
        &self,
        mut buf: impl Write,
        color: TermColor,
        i: usize,
        current: &InputLine,
        is_current: bool,
        term_width: u8,
    ) -> io::Result<()> {
        write!(buf, " {}", "        ".on_color(color))?;

        let mut name = current.name.bold();
        if let Some(color) = current.color {
            name = name.color(color);
        }
        write!(buf, "  {}", name)?;
        write!(buf, "  {} ", FmtValue::new(current, is_current, 6))?;

        if let Some(bounds) = current.bounds {
            Gradient {
                values: self.values(),
                component: i,
                bounds,
                to_color: self.to_color,
            }
            .write(&mut buf, term_width)?;
        }
        writeln!(buf)?;

        Ok(())
    }

    pub fn reset_term(&self, term: &Term) -> Result<()> {
        let lines = if self.expanded { 20 } else { 7 };
        for _ in 0..lines {
            term.move_cursor_up(1)?;
            term.clear_line()?;
        }
        Ok(())
    }

    pub fn enter_key(&mut self, key: Key) {
        if self.expanded {
            match key {
                Key::ArrowUp => self.next_mut().dec(),
                Key::ArrowDown => self.next_mut().inc(),
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
        } else {
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

    fn next_mut(&mut self) -> &mut InputLine {
        let len = self.len();
        &mut self.components[(self.current + 1) % len]
    }
}

struct Gradient {
    values: Vec<f64>,
    component: usize,
    bounds: (f64, f64),
    to_color: fn(&[f64]) -> Color,
}

impl Gradient {
    fn write(&mut self, mut buf: impl Write, term_width: u8) -> io::Result<()> {
        let values = self.values.as_mut_slice();
        let comp = self.component;
        let (min, max) = self.bounds;

        let dw = (term_width * 2 - 1) as f64;
        let highlighted = (values[comp] - min) * dw;
        let highlighted = (highlighted / (max - min)) as u8;

        values[comp] = min;
        let left_color = (self.to_color)(&values);
        write!(buf, "{}", "▕".color(left_color.to_term_color()))?;

        for i in 0..term_width {
            let v1 = min + (max - min) * ((2 * i) as f64 / dw);
            let v2 = min + (max - min) * ((2 * i + 1) as f64 / dw);

            values[comp] = v1;
            let c1 = (self.to_color)(&values);
            values[comp] = v2;
            let c2 = (self.to_color)(&values);
            let tc1 = c1.to_term_color();
            let tc2 = c2.to_term_color();

            if i == highlighted / 2 {
                let t = match c1.text_color() {
                    TextColor::Black => TermColor::Black,
                    TextColor::White => TermColor::BrightWhite,
                };
                if highlighted % 2 == 0 {
                    Self::write_char(&mut buf, t, tc2)?;
                } else {
                    Self::write_char(&mut buf, tc1, t)?;
                }
            } else {
                Self::write_char(&mut buf, tc1, tc2)?;
            }
        }

        values[comp] = max;
        let right_color = (self.to_color)(&values);
        write!(buf, "{}", "▏".color(right_color.to_term_color()))?;

        Ok(())
    }

    fn write_char(mut buf: impl Write, c1: TermColor, c2: TermColor) -> io::Result<()> {
        if c1 == c2 {
            write!(buf, "{}", "█".color(c1))
        } else {
            write!(buf, "{}", "▌".color(c1).on_color(c2))
        }
    }
}

struct RectGradientLine {
    values: Vec<f64>,
    to_color: fn(&[f64]) -> Color,
    comp: (usize, usize),
    v_values: (f64, f64),
    bounds: (f64, f64),
    highlight: Option<bool>,
}

impl RectGradientLine {
    fn write(&mut self, mut buf: impl Write, term_width: u8) -> io::Result<()> {
        let values = self.values.as_mut_slice();
        let (comp1, comp2) = self.comp;
        let (v_value1, v_value2) = self.v_values;
        let (min, max) = self.bounds;

        let dw = (term_width - 1) as f64;
        let highlight_h = (values[comp1] - min) * dw;
        let highlight_h = (highlight_h / (max - min)) as u8;

        for i in 0..term_width {
            let h_value = min + (max - min) * (i as f64 / dw);

            values[comp1] = h_value;
            values[comp2] = v_value1;
            let c1 = (self.to_color)(&values);
            values[comp2] = v_value2;
            let c2 = (self.to_color)(&values);
            let tc1 = c1.to_term_color();
            let tc2 = c2.to_term_color();

            if highlight_h == i {
                if let Some(first) = self.highlight {
                    let t = match c1.text_color() {
                        TextColor::Black => TermColor::Black,
                        TextColor::White => TermColor::BrightWhite,
                    };
                    if first {
                        Self::write_char(&mut buf, t, tc2)?;
                    } else {
                        Self::write_char(&mut buf, tc1, t)?;
                    }
                    continue;
                }
            }
            Self::write_char(&mut buf, tc1, tc2)?;
        }

        Ok(())
    }

    fn write_char(mut buf: impl Write, c1: TermColor, c2: TermColor) -> io::Result<()> {
        if c1 == c2 {
            write!(buf, "{}", "█".color(c1))
        } else {
            write!(buf, "{}", "▀".color(c1).on_color(c2))
        }
    }
}

#[derive(Copy, Clone)]
pub struct InputLine {
    name: &'static str,
    value: f64,
    bounds: Option<(f64, f64)>,
    initial: f64,
    color: Option<TermColor>,
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
            s.strip_suffix('.').unwrap_or(s).parse::<f64>().ok()
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
        assert!(min.is_finite() && max.is_finite());
        self.bounds = Some((min, max));
        self
    }

    pub fn with_color(mut self, c: TermColor) -> Self {
        self.color = Some(c);
        self
    }

    pub fn with_color_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_color(TermColor::TrueColor { r, g, b })
    }

    pub fn as_percent(mut self) -> Self {
        fn to_string(v: f64) -> String {
            format!("{}%", round10(v * 100.0))
        }
        fn from_str(s: &str) -> Option<f64> {
            s.strip_suffix('.')
                .unwrap_or(s)
                .parse::<f64>()
                .map(|n| n / 100.0)
                .ok()
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
