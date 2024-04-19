use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

use softbuffer::{Context, Surface};
use winit::window::Window;

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
}

impl Canvas {
    pub(crate) fn new(window: Rc<Window>, width: NonZeroU32, height: NonZeroU32) -> Self {
        let context = Context::new(window.clone()).unwrap();
        let surface = Rc::new(RefCell::new(Surface::new(&context, window).unwrap()));
        surface.borrow_mut().resize(width, height).unwrap();
        Self {
            surface,
            screen: vec![0; 4 * width.get() as usize * height.get() as usize],
            rect: Rect::new(0, 0, width.get(), height.get()),
            color: Color::from_rgba(0, 0, 0, 0xff),
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
            .map(|pixel| Color::from(pixel).into())
            .collect();
        buffer.copy_from_slice(&new_buffer);
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
                screen_slice.copy_from_slice(&line[sprite_start..sprite_end]);
            }
        }
    }
}
