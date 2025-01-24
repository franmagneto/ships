use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

use super::{
    color::Color,
    geometry::{Point, Rect},
    sprite::Sprite,
};

pub(crate) struct Canvas<'a> {
    pixels: Pixels<'a>,
    screen: Vec<u8>,
    rect: Rect,
    color: Color,
}

impl<'a> Canvas<'a> {
    pub(crate) fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            Pixels::new(width, height, surface_texture).unwrap()
        };

        Self {
            pixels,
            screen: vec![0; 4 * width as usize * height as usize],
            rect: Rect::new(0, 0, width, height),
            color: Color::from_rgba(0, 0, 0, 0xff),
        }
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height).unwrap();
    }

    pub(crate) fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub(crate) fn clear(&mut self) {
        let color = *self.color;
        for pixel in self.screen.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }

    pub(crate) fn present(&mut self) {
        let frame = self.pixels.frame_mut();
        for (frame_pixel, screen_pixel) in
            frame.chunks_exact_mut(4).zip(self.screen.chunks_exact(4))
        {
            let screen_color = Color::from_multiplied(screen_pixel);
            let mut screen_pixel: [u8; 4] = screen_color.into();
            screen_pixel[3] = 0xff;
            frame_pixel.copy_from_slice(&screen_pixel);
        }
        self.pixels.render().unwrap();
    }

    pub(crate) fn blit(&mut self, sprite: &Sprite, position: Point) {
        let dest = Rect::from_point(position, sprite.width(), sprite.height());
        if let Some(dest) = self.rect.intersection(dest) {
            let start_x = dest.x() - position.x();
            let start_y = dest.y() - position.y();
            let sprite_lines = sprite
                .as_lines()
                .skip(start_y as usize)
                .take(dest.h() as usize);
            for (y, line) in sprite_lines.enumerate() {
                let screen_start = 4 * ((dest.y() + y as i32) * self.rect.w() + dest.x()) as usize;
                let screen_end = screen_start + 4 * dest.w() as usize;
                let screen_slice = &mut self.screen[screen_start..screen_end];

                let sprite_start = 4 * start_x as usize;
                let sprite_end = sprite_start + 4 * dest.w() as usize;
                let sprite_slice = &line[sprite_start..sprite_end];

                for (screen_pixel, sprite_pixel) in screen_slice
                    .chunks_exact_mut(4)
                    .zip(sprite_slice.chunks_exact(4))
                {
                    screen_pixel.copy_from_slice(
                        &*Color::from_multiplied(screen_pixel)
                            .blend(Color::from_multiplied(sprite_pixel)),
                    );
                }
            }
        }
    }
}
