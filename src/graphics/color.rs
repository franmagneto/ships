use std::ops::Deref;

#[derive(Clone, Copy)]
pub(crate) struct Color([u8; 4]);

impl Color {
    pub(crate) fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }
}

impl Deref for Color {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u8]> for Color {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 4);
        Self([value[0], value[1], value[2], value[3]])
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        ((value.0[0] as u32) << 16) | ((value.0[1] as u32) << 8) | (value.0[2] as u32)
    }
}
