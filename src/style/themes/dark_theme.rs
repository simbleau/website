use crate::style::themes::BrandTheme;
use cssugar::prelude::*;
use once_cell::sync::Lazy;

pub static DARK_THEME: Lazy<BrandTheme> = Lazy::new(|| BrandTheme {
    color: Color::from_rgb(0xf8, 0xf9, 0xfa),
    background_color: Color::from_rgb(0x21, 0x25, 0x29),
    link: Color::from_rgb(0x00, 0x33, 0xdd),
    link_hover: Color::from_rgb(0x0d, 0x6e, 0xfd),
    header_color: Color::from_rgb(0x21, 0x25, 0x29),
    header_font: "Roboto",
    body_font: "Roboto",
    mono_font: "Roboto",
    fs: "1rem",
    fsm: "1.04rem",
    fst: "1.08rem",
    fsd: "1.12rem",
    fw: "400",
    fwh: "700",
});
