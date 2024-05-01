use crate::hooks::BrowserPreference;
use hex_color::HexColor;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ThemeChoice {
    Light,
    Dark,
    Lego,
}

impl std::ops::Deref for ThemeChoice {
    type Target = Theme;

    fn deref(&self) -> &'static Self::Target {
        match self {
            ThemeChoice::Light => &LIGHT_THEME,
            ThemeChoice::Dark => &DARK_THEME,
            ThemeChoice::Lego => &LEGO_THEME,
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
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

pub static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    color: HexColor::from_u24(0xcccccc),
    background_color: HexColor::from_u24(0x222222),
    link: HexColor::from_u24(0x3399FF),
    link_hover: HexColor::from_u24(0x3399FF).scale(1.2),
    body_font: "Barlow",
    mono_font: "Barlow",
    fsm: "1.2rem",
    fst: "1.4rem",
    fsd: "1.6rem",
    fw: "400",
    fwh: "700",
});

pub static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    color: HexColor::from_u24(0x212529),
    background_color: HexColor::from_u24(0xf8f9fa),
    link: HexColor::from_u24(0x0033dd),
    link_hover: HexColor::from_u24(0x4C7FFF),
    body_font: "Barlow",
    mono_font: "Barlow",
    fsm: "1.2rem",
    fst: "1.4rem",
    fsd: "1.6rem",
    fw: "400",
    fwh: "700",
});

pub static LEGO_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    color: HexColor::from_u24(0xFFD700),
    background_color: HexColor::from_u24(0xDA291C),
    link: HexColor::from_u24(0xFFFFFF),
    link_hover: HexColor::from_u24(0x000000),
    body_font: "Barlow",
    mono_font: "Barlow",
    fsm: "1.2rem",
    fst: "1.4rem",
    fsd: "1.6rem",
    fw: "400",
    fwh: "700",
});
