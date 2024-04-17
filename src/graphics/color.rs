use std::ops::Deref;

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

pub(crate) struct Color {
    val: u32,
}

impl From<RGB> for Color {
    fn from(value: RGB) -> Self {
        Self {
            val: (value.b as u32) | ((value.g as u32) << 8) | ((value.r as u32) << 16),
        }
    }
}

impl Deref for Color {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl Color {
    pub(crate) fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }.into()
    }
}
