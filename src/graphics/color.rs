#[derive(Clone, Copy)]
pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub(crate) fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: ((value & 0xff0000) >> 16) as u8,
            g: ((value & 0xff00) >> 8) as u8,
            b: (value & 0xff) as u8,
            a: 0xff,
        }
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        ((value.r as u32) << 16) | ((value.g as u32) << 8) | (value.b as u32)
    }
}

impl From<&[u8]> for Color {
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 4);
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        let Color { r, g, b, a } = value;
        [r, g, b, a]
    }
}
