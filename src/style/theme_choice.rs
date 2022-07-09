use serde::{Deserialize, Serialize};

use crate::style::themes::{DARK_THEME, LIGHT_THEME};
use crate::style::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeChoice {
    Dark,
    Light,
}

impl Default for ThemeChoice {
    fn default() -> Self {
        ThemeChoice::Light
    }
}

impl ThemeChoice {
    pub fn theme(&self) -> &Theme {
        match self {
            ThemeChoice::Dark => &DARK_THEME,
            ThemeChoice::Light => &LIGHT_THEME,
        }
    }
}
