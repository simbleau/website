use hex_color::HexColor;

pub fn lighten(c: &HexColor, l: f32) -> HexColor {
    let (r, g, b, a) = c.split();
    let (r, g, b) = (
        (r as f32 * l).min(u8::MAX as f32) as u8,
        (g as f32 * l).min(u8::MAX as f32) as u8,
        (b as f32 * l).min(u8::MAX as f32) as u8,
    );
    HexColor::from_u32(
        (0xff & r as u32) << 24
            | (0xff & g as u32) << 16
            | (0xff & b as u32) << 8
            | a as u32,
    )
}
