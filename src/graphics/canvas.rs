use std::{cell::RefCell, num::NonZeroU32, rc::Rc, usize};

use softbuffer::{Context, Surface};
use winit::window::Window;

use super::{color::Color, geometry::Point, sprite::Sprite};

struct Size {
    width: NonZeroU32,
    height: NonZeroU32,
}

impl From<Size> for (u32, u32) {
    fn from(value: Size) -> Self {
        (value.width.get(), value.height.get())
    }
}

pub(crate) struct Canvas {
    surface: Rc<RefCell<Surface<Rc<Window>, Rc<Window>>>>,
    width: NonZeroU32,
    height: NonZeroU32,
    color: Color,
}

impl Canvas {
    pub(crate) fn new(window: Rc<Window>, width: NonZeroU32, height: NonZeroU32) -> Self {
        let context = Context::new(window.clone()).unwrap();
        let surface = Rc::new(RefCell::new(Surface::new(&context, window).unwrap()));
        surface.borrow_mut().resize(width, height).unwrap();
        Self {
            surface,
            width,
            height,
            color: Color::from_rgb(0, 0, 0),
        }
    }

    pub(crate) fn resize(
        &mut self,
        width: NonZeroU32,
        height: NonZeroU32,
    ) -> Result<(), softbuffer::SoftBufferError> {
        self.width = width;
        self.height = height;
        self.surface.borrow_mut().resize(width, height)
    }

    pub(crate) fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    pub(crate) fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub(crate) fn clear(&self) {
        let mut surface = self.surface.borrow_mut();
        let mut buffer = surface.buffer_mut().unwrap();
        buffer.fill(*self.color);
    }

    pub(crate) fn present(&self) {
        let mut surface = self.surface.borrow_mut();
        let buffer = surface.buffer_mut().unwrap();
        buffer.present().unwrap();
    }

    pub(crate) fn blit(&self, sprite: &Sprite, position: Point) {
        let mut surface = self.surface.borrow_mut();
        let mut buffer = surface.buffer_mut().unwrap();
        for (i, pixels) in sprite
            .graphics()
            .chunks_exact(sprite.width() as usize)
            .enumerate()
        {
            let line_start = position.x as usize
                + position.y as usize * self.width.get() as usize
                + i * self.width.get() as usize;
            let line_end = line_start + sprite.width() as usize;
            let line_start = if line_start < buffer.len() {
                line_start
            } else {
                buffer.len()
            };
            let line_end = if line_end < buffer.len() {
                line_end
            } else {
                buffer.len()
            };
            let line = &mut buffer[line_start..line_end];
            line.copy_from_slice(&pixels[..line_end - line_start]);
        }
    }
}
