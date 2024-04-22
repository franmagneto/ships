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
    scaled_screen_indexes: Vec<usize>,
    window_size: (u32, u32),
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
            width: scaled_width,
            height: scaled_height,
        } = window.clone().inner_size();
        let mut ret = Self {
            surface,
            screen: vec![0; 4 * width as usize * height as usize],
            scaled_screen_indexes: vec![],
            window_size: (1, 1),
            rect: Rect::new(0, 0, width, height),
            color: Color::from_rgba(0, 0, 0, 0xff),
            scale: (1.0, 1.0),
        };
        ret.resize(
            NonZeroU32::new(scaled_width).unwrap(),
            NonZeroU32::new(scaled_height).unwrap(),
        );

        ret
    }

    pub(crate) fn resize(&mut self, width: NonZeroU32, height: NonZeroU32) {
        if (width.get(), height.get()) != self.window_size {
            self.surface.borrow_mut().resize(width, height).unwrap();
            self.scale = (
                width.get() as f64 / self.rect.w() as f64,
                height.get() as f64 / self.rect.h() as f64,
            );
            self.scaled_screen_indexes = vec![0; width.get() as usize * height.get() as usize];
            for (i, index) in self.scaled_screen_indexes.iter_mut().enumerate() {
                let x = ((i as u32 % width) as f64 / self.scale.0) as usize;
                let y = ((i as u32 / width) as f64 / self.scale.1) as usize;

                *index = y * self.rect.w() as usize + x;
            }
            self.window_size = (width.get(), height.get());
        }
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
        let mut buffer = surface.buffer_mut().unwrap();
        let new_buffer: Vec<u32> = self
            .screen
            .chunks_exact(4)
            .map(|pixel| {
                let pixel: &[u8; 4] = pixel.try_into().unwrap();
                Color::from_multiplied(pixel).into()
            })
            .collect();
        for (i, pixel) in buffer.iter_mut().enumerate() {
            *pixel = new_buffer[self.scaled_screen_indexes[i]]
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
