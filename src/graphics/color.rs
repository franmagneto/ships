use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color([u8; 4]);

impl Color {
    pub(crate) fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut ret = Self([r, g, b, a]);
        ret.multiply();
        ret
    }

    pub(crate) fn from_multiplied(value: &[u8]) -> Self {
        Self([value[0], value[1], value[2], value[3]])
    }

    pub(crate) fn blend(&self, source: Self) -> Self {
        let [sr, sg, sb, sa] = source.0;
        let [dr, dg, db, da] = self.0;
        if sa == 0xff {
            Self(source.0)
        } else {
            Self([
                (sr as u32 + (dr as u32 * (255 - sa as u32))) as u8,
                (sg as u32 + (dg as u32 * (255 - sa as u32))) as u8,
                (sb as u32 + (db as u32 * (255 - sa as u32))) as u8,
                (sa as u32 + (da as u32 * (255 - sa as u32))) as u8,
            ])
        }
    }

    fn multiply(&mut self) {
        let [r, g, b, a] = self.0;
        if a != 0xff {
            self.0[0] = multiply_channel(r, a);
            self.0[1] = multiply_channel(g, a);
            self.0[2] = multiply_channel(b, a);
        }
    }

    fn demultiply(&self) -> [u8; 4] {
        let [r, g, b, a] = self.0;
        if a == 0xff {
            self.0
        } else {
            [
                demultiply_channel(r, a),
                demultiply_channel(g, a),
                demultiply_channel(b, a),
                a,
            ]
        }
    }
}

impl Deref for Color {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        value.demultiply()
    }
}

impl From<&[u8; 4]> for Color {
    fn from(value: &[u8; 4]) -> Self {
        Self::from_rgba(value[0], value[1], value[2], value[3])
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> u32 {
        let demultiplied = value.demultiply();
        ((demultiplied[0] as u32) << 16)
            | ((demultiplied[1] as u32) << 8)
            | (demultiplied[2] as u32)
    }
}

fn multiply_channel(c: u8, a: u8) -> u8 {
    (c as u32 * a as u32 / 255) as u8
}

fn demultiply_channel(c: u8, a: u8) -> u8 {
    (c as u32 * 255 / a as u32) as u8
}
