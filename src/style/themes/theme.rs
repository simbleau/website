use crate::style::themes::{DARK_THEME, LIGHT_THEME};
use cssugar::prelude::*;
use serde::{Deserialize, Serialize};
use themer::prelude::*;

#[theme_key]
#[derive(Serialize, Deserialize)]
pub enum ThemeChoice {
    Light,
    Dark,
}

impl Default for ThemeChoice {
    fn default() -> Self {
        BrowserPreference::get()
            .map(|p| match p {
                BrowserPreference::Dark => ThemeChoice::Dark,
                BrowserPreference::Light => ThemeChoice::Light,
            })
            .unwrap_or(ThemeChoice::Light)
    }
}

impl ThemeKey for ThemeChoice {
    type Theme = ThemeSpec;

    fn theme(&self) -> &'static Self::Theme {
        match self {
            ThemeChoice::Light => &LIGHT_THEME,
            ThemeChoice::Dark => &DARK_THEME,
        }
    }
}

#[theme]
pub struct ThemeSpec {
    /// Foreground color
    pub color: Color,
    /// Background color
    pub background_color: Color,
    /// Link color
    pub link: Color,
    /// Link hover color
    pub link_hover: Color,
    /// Header color
    pub header_color: Color,

    // Fonts
    pub header_font: &'static str,
    pub body_font: &'static str,
    pub mono_font: &'static str,

    /// Font size - default
    pub fs: &'static str,
    /// Font size - mobile
    pub fsm: &'static str,
    /// Font size - tablet
    pub fst: &'static str,
    /// Font size - desktop
    pub fsd: &'static str,

    /// Font width - text
    pub fw: &'static str,
    /// Font width - headers
    pub fwh: &'static str,
}
