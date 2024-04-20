use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use winit::{dpi::PhysicalSize, window::Window};

use super::{
    color::Color,
    geometry::{Point, Rect},
    sprite::Sprite,
};

pub(crate) struct Canvas {
    surface: Rc<RefCell<Surface<Rc<Window>, Rc<Window>>>>,
    screen: Vec<u8>,
    rect: Rect,
    color: Color,
    scale: (f64, f64),
}

impl Canvas {
    pub(crate) fn new(window: Rc<Window>, width: u32, height: u32) -> Self {
        let context = Context::new(window.clone()).unwrap();
        let surface = Rc::new(RefCell::new(
            Surface::new(&context, window.clone()).unwrap(),
        ));
        let PhysicalSize {
            width: window_width,
            height: window_height,
        } = window.clone().inner_size();
        surface
            .borrow_mut()
            .resize(
                NonZeroU32::new(window_width).unwrap(),
                NonZeroU32::new(window_height).unwrap(),
            )
            .unwrap();
        Self {
            surface,
            screen: vec![0; 4 * width as usize * height as usize],
            rect: Rect::new(0, 0, width, height),
            color: Color::from_rgba(0, 0, 0, 0xff),
            scale: (
                window_width as f64 / width as f64,
                window_height as f64 / height as f64,
            ),
        }
    }

    pub(crate) fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.surface.borrow_mut().resize(width, height).unwrap();
        self.scale = (
            width.get() as f64 / self.rect.w() as f64,
            height.get() as f64 / self.rect.h() as f64,
        )
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

    pub(crate) fn present(&self) {
        let mut surface = self.surface.borrow_mut();
        let window = surface.window().clone();
        let mut buffer = surface.buffer_mut().unwrap();
        let new_buffer: Vec<u32> = self
            .screen
            .chunks_exact(4)
            .map(|pixel| {
                let pixel: &[u8; 4] = pixel.try_into().unwrap();
                Color::from_multiplied(pixel).into()
            })
            .collect();
        let PhysicalSize { width, .. } = window.inner_size();
        for i in 0..buffer.len() {
            let x = ((i as u32 % width) as f64 / self.scale.0) as usize;
            let y = ((i as u32 / width) as f64 / self.scale.1) as usize;

            buffer[i] = new_buffer[y * self.rect.w() as usize + x];
        }
        buffer.present().unwrap();
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
