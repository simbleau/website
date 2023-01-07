use crate::style::themes::BrandTheme;
use cssugar::prelude::*;
use once_cell::sync::Lazy;

pub static DARK_THEME: Lazy<BrandTheme> = Lazy::new(|| BrandTheme {
    color: Color::from_rgb(0xcc, 0xcc, 0xcc),
    background_color: Color::from_rgb(0x22, 0x22, 0x22),
    link: Color::from_rgb(0x00, 0xaa, 0xff),
    link_hover: Color::from_rgb(0x00, 0xcf, 0xff),
    header_color: Color::from_rgb(0xcc, 0xcc, 0xcc),
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
