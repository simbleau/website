use crate::style::themes::{DARK_THEME, LIGHT_THEME};
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
    pub color: &'static str,
    /// Background color
    pub background_color: &'static str,
    /// Link color
    pub link: &'static str,
    /// Link hover color
    pub link_hover: &'static str,
    /// Header color
    pub header_color: &'static str,

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
