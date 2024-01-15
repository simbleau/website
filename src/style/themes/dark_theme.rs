use crate::style::themes::ThemeSpec;
use hex_color::HexColor;
use once_cell::sync::Lazy;

pub static DARK_THEME: Lazy<ThemeSpec> = Lazy::new(|| ThemeSpec {
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
