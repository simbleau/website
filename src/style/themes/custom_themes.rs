use crate::style::themes::ThemeSpec;
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static LEGO_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
    color: HexColor::from_u24(0xFFD700),
    background_color: HexColor::from_u24(0xDA291C),
    link: HexColor::from_u24(0xFFFFFF),
    link_hover: HexColor::from_u24(0x000000),
    body_font: "Roboto",
    mono_font: "Roboto",
    fs: "1rem",
    fsm: "1.04rem",
    fst: "1.08rem",
    fsd: "1.12rem",
    fw: "400",
    fwh: "700",
});
