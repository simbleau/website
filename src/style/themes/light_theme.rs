use once_cell::sync::Lazy;

use super::Theme;
use crate::style::colors::Color;

pub static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: Color::opaque(0x44, 0x44, 0x44),
    fg2: Color::opaque(0x33, 0x33, 0x33),
    bg1: Color::opaque(0xff, 0xff, 0xff),
    bg2: Color::opaque(0xf1, 0xf1, 0xf1),
    ac1: Color::opaque(0x00, 0x66, 0xee),
    ac2: Color::opaque(0x00, 0x99, 0xff),
});
