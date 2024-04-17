use image::io::Reader as ImageReader;

use super::color::Color;

pub(crate) struct Sprite {
    graphics: Vec<u32>,
    width: u32,
    height: u32,
}

impl Sprite {
    pub(crate) fn new(image_path: &str) -> Self {
        let image = ImageReader::open(image_path).unwrap().decode().unwrap();
        Self {
            graphics: image
                .as_rgba8()
                .unwrap()
                .chunks_exact(4)
                .map(|pixel| *Color::from_rgb(pixel[0], pixel[1], pixel[2]))
                .collect(),
            width: image.width(),
            height: image.height(),
        }
    }

    pub(crate) fn graphics(&self) -> &[u32] {
        &self.graphics
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }
}
