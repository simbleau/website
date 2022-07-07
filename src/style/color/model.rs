use std::fmt;

use crate::style::color::{OpaqueColor, TransparentColor};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Opaque(OpaqueColor),
    Transparent(TransparentColor),
}

impl Color {
    pub fn opaque(r: u8, g: u8, b: u8) -> Color {
        Color::Opaque(OpaqueColor {
            red: r,
            green: g,
            blue: b,
        })
    }

    pub fn transparent(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color::Transparent(TransparentColor {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        })
    }

    pub fn r(&self) -> u8 {
        match self {
            Color::Opaque(c) => c.red,
            Color::Transparent(c) => c.red,
        }
    }

    pub fn g(&self) -> u8 {
        match self {
            Color::Opaque(c) => c.green,
            Color::Transparent(c) => c.green,
        }
    }

    pub fn b(&self) -> u8 {
        match self {
            Color::Opaque(c) => c.blue,
            Color::Transparent(c) => c.blue,
        }
    }

    pub fn a(&self) -> f32 {
        match self {
            Color::Opaque(_) => 1.0,
            Color::Transparent(c) => c.alpha,
        }
    }

    pub fn with_alpha(&self, a: f32) -> Color {
        Color::Transparent(TransparentColor {
            red: self.r(),
            green: self.g(),
            blue: self.b(),
            alpha: a,
        })
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Opaque(c) => {
                write!(f, "rgb({}, {}, {})", c.red, c.green, c.blue)
            }
            Color::Transparent(c) => {
                write!(
                    f,
                    "rgba({}, {}, {}, {})",
                    c.red, c.green, c.blue, c.alpha
                )
            }
        }
    }
}
