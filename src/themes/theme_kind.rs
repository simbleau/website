use once_cell::sync::Lazy;
use yew::html::ImplicitClone;

use crate::themes::Theme;

#[derive(Debug, Clone, PartialEq)]
pub enum ThemeKind {
    Dark,
    Light,
}
impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "black".to_string(),
            background_color: "rgb(237, 244, 255)".to_string(),
            paper_color: "white".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "white".to_string(),
            background_color: "black".to_string(),
            paper_color: "rgb(50, 50, 50)".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }
}
