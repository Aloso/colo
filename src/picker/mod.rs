use anyhow::Result;
use color_space::{Cmy, Hsl, Hsv, Rgb, ToRgb};
use console::{Key, Term};

use crate::color::{Color, ColorSpace};
use three_lines::{InputLine, ThreeLines};

mod three_lines;

pub struct ColorPicker {
    inputs: [ThreeLines; 4],
    current: usize,
}

impl ColorPicker {
    pub fn new(color: Option<Color>, cs: Option<ColorSpace>) -> Self {
        let current = match cs {
            Some(cs) => match cs {
                ColorSpace::Rgb => 0,
                ColorSpace::Cmy => 3,
                ColorSpace::Cmyk => 3,
                ColorSpace::Hsv => 2,
                ColorSpace::Hsl => 1,
                ColorSpace::Lch => 0,
                ColorSpace::Luv => 0,
                ColorSpace::Lab => 0,
                ColorSpace::HunterLab => 0,
                ColorSpace::Xyz => 0,
                ColorSpace::Yxy => 0,
            },
            None => 1,
        };

        let mut s = Self {
            inputs: [rgb_input(), hsl_input(), hsv_input(), cmy_input()],
            current,
        };

        if let Some(c) = color {
            s.inputs[s.current].set_color(c);
        };

        s
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn current(&self) -> &ThreeLines {
        &self.inputs[self.current]
    }

    pub fn current_mut(&mut self) -> &mut ThreeLines {
        &mut self.inputs[self.current]
    }

    pub fn next_color_space(&mut self) {
        let c = self.current().color();
        self.current = (self.current + 1) % self.len();
        self.current_mut().set_color(c);
    }

    pub fn prev_color_space(&mut self) {
        let c = self.current().color();
        self.current = (self.current + self.len() - 1) % self.len();
        self.current_mut().set_color(c);
    }

    pub fn display(&mut self) -> Result<Color> {
        let term = Term::stdout();

        let mut color = self.current().color();

        loop {
            self.current_mut().print(color, &term)?;

            let key = term.read_key()?;
            self.current_mut().reset_term(&term)?;

            match key {
                Key::Enter | Key::Escape => break,
                Key::Tab => self.next_color_space(),
                Key::BackTab => self.prev_color_space(),
                k => self.current_mut().enter_key(k),
            }

            color = self.current().color();
        }
        Ok(color)
    }
}

fn rgb_input() -> ThreeLines {
    fn to_color(v: &[f64]) -> Color {
        Color::Rgb(Rgb::new(v[0], v[1], v[2]))
    }
    fn from_color(c: Color) -> Vec<f64> {
        let rgb = c.to_rgb();
        vec![rgb.r, rgb.g, rgb.b]
    }

    ThreeLines::new(
        "RGB",
        from_color,
        to_color,
        &[
            InputLine::new("R", 0.0)
                .with_bounds(0.0, 255.0)
                .with_color_rgb(255, 60, 60),
            InputLine::new("G", 0.0)
                .with_bounds(0.0, 255.0)
                .with_color_rgb(60, 255, 60),
            InputLine::new("B", 0.0)
                .with_bounds(0.0, 255.0)
                .with_color_rgb(60, 60, 255),
        ],
    )
}

fn cmy_input() -> ThreeLines {
    fn to_color(v: &[f64]) -> Color {
        Color::Cmy(Cmy::new(v[0], v[1], v[2]))
    }
    fn from_color(c: Color) -> Vec<f64> {
        let cmy = Cmy::from(c.to_rgb());
        vec![cmy.c, cmy.m, cmy.y]
    }

    ThreeLines::new(
        "CMY",
        from_color,
        to_color,
        &[
            InputLine::new("C", 0.0)
                .with_bounds(0.0, 1.0)
                .as_percent()
                .with_color_rgb(0, 255, 255),
            InputLine::new("M", 0.0)
                .with_bounds(0.0, 1.0)
                .as_percent()
                .with_color_rgb(255, 0, 255),
            InputLine::new("Y", 0.0)
                .with_bounds(0.0, 1.0)
                .as_percent()
                .with_color_rgb(255, 255, 0),
        ],
    )
}

fn hsl_input() -> ThreeLines {
    fn to_color(v: &[f64]) -> Color {
        Color::Hsl(Hsl::new(v[0], v[1], v[2]))
    }
    fn from_color(c: Color) -> Vec<f64> {
        let hsl = Hsl::from(c.to_rgb());
        vec![hsl.h, hsl.s, hsl.l]
    }

    ThreeLines::new(
        "HSL",
        from_color,
        to_color,
        &[
            InputLine::new("H", 0.0).with_bounds(0.0, 360.0),
            InputLine::new("S", 1.0).with_bounds(0.0, 1.0).as_percent(),
            InputLine::new("L", 0.5).with_bounds(0.0, 1.0).as_percent(),
        ],
    )
}

fn hsv_input() -> ThreeLines {
    fn to_color(v: &[f64]) -> Color {
        Color::Hsv(Hsv::new(v[0], v[1], v[2]))
    }
    fn from_color(c: Color) -> Vec<f64> {
        let hsv = Hsv::from(c.to_rgb());
        vec![hsv.h, hsv.s, hsv.v]
    }

    ThreeLines::new(
        "HSV",
        from_color,
        to_color,
        &[
            InputLine::new("H", 0.0).with_bounds(0.0, 360.0),
            InputLine::new("S", 1.0).with_bounds(0.0, 1.0).as_percent(),
            InputLine::new("V", 1.0).with_bounds(0.0, 1.0).as_percent(),
        ],
    )
}
