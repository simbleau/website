use once_cell::sync::Lazy;

use crate::themes::{colors::BLACK, colors::WHITE, Theme};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThemeChoice {
    Dark,
    Light,
}

impl ThemeChoice {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            fg1: BLACK,
            fg2: BLACK,
            bg1: WHITE,
            bg2: WHITE,
            ac1: BLACK,
            ac2: BLACK,
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            fg1: WHITE,
            fg2: WHITE,
            bg1: BLACK,
            bg2: BLACK,
            ac1: WHITE,
            ac2: WHITE,
        });

        match self {
            ThemeChoice::Dark => &DARK_THEME,
            ThemeChoice::Light => &LIGHT_THEME,
        }
    }
}
