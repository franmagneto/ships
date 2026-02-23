use std::{fs::File, io::Read, path::Path, slice::ChunksExact};

use png_decoder::PngHeader;

pub(crate) struct Sprite {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Sprite {
    pub(crate) fn load_png<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let mut png_data = vec![];
        match File::open(path) {
            Ok(mut png_file) => {
                if let Err(err) = png_file.read_to_end(&mut png_data) {
                    return Err(err.to_string());
                }
            }
            Err(err) => return Err(err.to_string()),
        }
        match png_decoder::decode(&png_data) {
            Ok((PngHeader { width, height, .. }, data)) => Ok(Self {
                data: data.into_flattened(),
                width,
                height,
            }),
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn as_lines(&self) -> ChunksExact<'_, u8> {
        self.data.chunks_exact(4 * self.width as usize)
    }
}
