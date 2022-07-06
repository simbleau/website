use once_cell::sync::Lazy;

use crate::style::colors::{BLACK, WHITE};
use crate::style::Color;
use crate::style::Theme;

pub static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: Color {
        red: 0x44,
        green: 0x44,
        blue: 0x44,
        alpha: 0xff,
    },
    fg2: Color {
        red: 0x33,
        green: 0x33,
        blue: 0x33,
        alpha: 0xff,
    },
    bg1: Color {
        red: 0xff,
        green: 0xff,
        blue: 0xff,
        alpha: 0xff,
    },
    bg2: Color {
        red: 0xf1,
        green: 0xf1,
        blue: 0xf1,
        alpha: 0xff,
    },
    ac1: Color {
        red: 0x00,
        green: 0x06,
        blue: 0xee,
        alpha: 0xff,
    },
    ac2: Color {
        red: 0x00,
        green: 0x99,
        blue: 0xff,
        alpha: 0xff,
    },
});

pub static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: WHITE,
    fg2: WHITE,
    bg1: BLACK,
    bg2: BLACK,
    ac1: WHITE,
    ac2: WHITE,
});
