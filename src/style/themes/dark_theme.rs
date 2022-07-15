use once_cell::sync::Lazy;

use super::Theme;
use crate::style::colors::Color;

pub static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
    fg1: Color::opaque(0xcc, 0xcc, 0xcc),
    fg2: Color::opaque(0xdd, 0xdd, 0xdd),
    bg1: Color::opaque(0x22, 0x22, 0x22),
    bg2: Color::opaque(0x30, 0x30, 0x30),
    ac1: Color::opaque(0x00, 0xaa, 0xff),
    ac2: Color::opaque(0x00, 0xcf, 0xff),
    fs: "1rem",
    fsm: "1.04rem",
    fst: "1.08rem",
    fsd: "1.12rem",
    fw: "400",
    fwh: "700",
});
