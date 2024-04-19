use std::collections::HashSet;

use winit::keyboard::Key;

use crate::graphics::{canvas::Canvas, geometry::Rect, sprite::Sprite};

pub(crate) trait Entity {
    fn update(&mut self);
}

pub(crate) trait Renderable {
    fn sprite(&self) -> &Sprite;
    fn rect(&self) -> Rect;
    fn render(&self, canvas: &mut Canvas) {
        canvas.blit(self.sprite(), self.rect().into())
    }
}

pub(crate) trait Controllable {
    fn handle_input(&mut self, keys: &HashSet<Key>);
}
