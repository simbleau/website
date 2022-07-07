use once_cell::sync::Lazy;

use crate::style::Color;
use crate::style::Theme;

pub static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: Color {
        red: 0x44,
        green: 0x44,
        blue: 0x44,
    },
    fg2: Color {
        red: 0x33,
        green: 0x33,
        blue: 0x33,
    },
    bg1: Color {
        red: 0xff,
        green: 0xff,
        blue: 0xff,
    },
    bg2: Color {
        red: 0xf1,
        green: 0xf1,
        blue: 0xf1,
    },
    ac1: Color {
        red: 0x00,
        green: 0x66,
        blue: 0xee,
    },
    ac2: Color {
        red: 0x00,
        green: 0x99,
        blue: 0xff,
    },
});

pub static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: Color {
        red: 0xcc,
        green: 0xcc,
        blue: 0xcc,
    },
    fg2: Color {
        red: 0xdd,
        green: 0xdd,
        blue: 0xdd,
    },
    bg1: Color {
        red: 0x22,
        green: 0x22,
        blue: 0x22,
    },
    bg2: Color {
        red: 0x30,
        green: 0x30,
        blue: 0x30,
    },
    ac1: Color {
        red: 0x00,
        green: 0xaa,
        blue: 0xff,
    },
    ac2: Color {
        red: 0x00,
        green: 0xcf,
        blue: 0xff,
    },
});
