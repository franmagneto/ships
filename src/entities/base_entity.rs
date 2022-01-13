use crate::utils::CenterOnVector;
use nalgebra::Vector2;
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
    fn position(&self) -> Vector2<f64>;
    fn velocity(&self) -> Vector2<f64>;
    fn rect(&self) -> Rect;
    fn render(&self, canvas: &mut WindowCanvas) {
        let render_position = self.position() + self.velocity();
        let mut render_rect = self.rect().clone();
        render_rect.center_on_vector(render_position);
        canvas
            .copy_ex(self.graphics(), None, render_rect, 0.0, None, false, false)
            .unwrap();
    }
}

pub(crate) trait Controllable {
    fn handle_input(&mut self, keys: HashSet<Keycode>);
}
