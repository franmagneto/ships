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
            rect: Rect::new(0, 0, width.get(), height.get()),
            color: Color::from_rgba(0, 0, 0, 0xff),
        }
    }

    pub(crate) fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub(crate) fn clear(&self) {
        let mut surface = self.surface.borrow_mut();
        let mut buffer = surface.buffer_mut().unwrap();
        buffer.fill(self.color.into());
    }

    pub(crate) fn present(&self) {
        let mut surface = self.surface.borrow_mut();
        let buffer = surface.buffer_mut().unwrap();
        buffer.present().unwrap();
    }

    pub(crate) fn blit(&self, sprite: &Sprite, position: Point) {
        let dest = Rect::from_point(position, sprite.width(), sprite.height());
        if let Some(dest) = self.rect.intersection(dest) {
            let mut surface = self.surface.borrow_mut();
            let mut buffer = surface.buffer_mut().unwrap();
            let start_x = dest.x() - position.x();
            let start_y = dest.y() - position.y();
            let sprite_lines = sprite
                .as_lines()
                .skip(start_y as usize)
                .take(dest.h() as usize);
            for (y, line) in sprite_lines.enumerate() {
                let line_u32: Vec<u32> = line
                    .chunks_exact(4)
                    .map(|pixel| Color::from(pixel).into())
                    .skip(start_x as usize)
                    .take(dest.w() as usize)
                    .collect();
                let start = ((dest.y() + y as i32) * self.rect.w() + dest.x()) as usize;
                let end = start + dest.w() as usize;
                let buffer_slice = &mut buffer[start..end];
                buffer_slice.copy_from_slice(&line_u32);
            }
        }
    }
}
