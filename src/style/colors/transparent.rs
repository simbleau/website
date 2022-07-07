#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TransparentColor {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
    pub(crate) alpha: f32,
}
