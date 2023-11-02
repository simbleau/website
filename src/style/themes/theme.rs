use crate::style::themes::{DARK_THEME, LEGO_THEME, LIGHT_THEME};
use hex_color::HexColor;
use serde::{Deserialize, Serialize};
use themer::{
    browser::BrowserPreference,
    macros::{theme, theme_key},
    traits::ThemeKey,
};

#[theme_key]
#[derive(Serialize, Deserialize)]
pub enum ThemeChoice {
    Light,
    Dark,
    Lego,
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
            ThemeChoice::Lego => &LEGO_THEME,
        }
    }
}

#[theme]
pub struct ThemeSpec {
    /// Foreground color
    pub color: HexColor,
    /// Background color
    pub background_color: HexColor,
    /// Link color
    pub link: HexColor,
    /// Link hover color
    pub link_hover: HexColor,

    // Fonts
    pub body_font: &'static str,
    pub mono_font: &'static str,

    /// Font size - mobile (default)
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
