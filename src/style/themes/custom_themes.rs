use crate::style::themes::ThemeSpec;
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static LEGO_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
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
