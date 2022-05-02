use super::base_entity::{Controllable, Entity, Renderable};
use sdl2::{
    image::LoadTexture,
    keyboard::Keycode,
    rect::{Point, Rect},
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::collections::HashSet;

pub(crate) struct Ship<'a> {
    graphics: Texture<'a>,
    position: Point,
    velocity: Point,
    rect: Rect,
}

impl<'a> Ship<'a> {
    pub(crate) fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            graphics: tc.load_texture("assets/ship.png").unwrap(),
            position: Point::new(16, 112),
            velocity: Point::new(0, 0),
            rect: Rect::new(0, 0, 16, 16),
        }
    }
}

impl Entity for Ship<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
        if self.position.y <= 8 {
            self.velocity.y = 0;
            self.position.y = 8;
        } else if self.position.y >= 216 {
            self.velocity.y = 0;
            self.position.y = 216;
        } else {
            self.velocity /= 2;
        }
        self.rect.center_on(self.position);
    }
}

impl Renderable for Ship<'_> {
    fn graphics(&self) -> &Texture {
        &self.graphics
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}

impl Controllable for Ship<'_> {
    fn handle_input(&mut self, keys: HashSet<Keycode>) {
        if keys.contains(&Keycode::Up) {
            self.velocity += Point::new(0, -2);
        }
        if keys.contains(&Keycode::Down) {
            self.velocity += Point::new(0, 2);
        }
    }
}
