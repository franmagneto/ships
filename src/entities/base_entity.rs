use sdl2::{
    keyboard::Keycode,
    rect::Rect,
    render::{Texture, WindowCanvas},
};
use std::collections::HashSet;

pub(crate) trait Entity {
    fn update(&mut self);
}

pub(crate) trait Renderable {
    fn graphics(&self) -> &Texture;
    fn rect(&self) -> Rect;
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas
            .copy(self.graphics(), None, self.rect())
            .unwrap();
    }
}

pub(crate) trait Controllable {
    fn handle_input(&mut self, keys: HashSet<Keycode>);
}
