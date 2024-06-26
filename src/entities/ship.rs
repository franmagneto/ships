use std::collections::HashSet;

use winit::keyboard::{Key, NamedKey};

use crate::graphics::{
    geometry::{Point, Rect},
    sprite::Sprite,
};

use super::base_entity::{Controllable, Entity, Renderable};

pub(crate) struct Ship {
    sprite: Sprite,
    position: Point,
    velocity: Point,
    rect: Rect,
}

impl Ship {
    pub(crate) fn new() -> Self {
        let sprite = Sprite::load_png("assets/ship.png").unwrap();
        let width = sprite.width();
        let height = sprite.height();
        Self {
            sprite,
            rect: Rect::new(0, 0, width, height),
            position: Point::new(16, 112),
            velocity: Point::new(0, 0),
        }
    }
}

impl Entity for Ship {
    fn update(&mut self) {
        self.position += self.velocity;
        if self.position.y() <= 8 {
            self.velocity.set_y(0);
            self.position.set_y(8);
        } else if self.position.y() >= 216 {
            self.velocity.set_y(0);
            self.position.set_y(216);
        } else {
            self.velocity /= 2;
        }
        self.rect.center_on(self.position);
    }
}

impl Renderable for Ship {
    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}

impl Controllable for Ship {
    fn handle_input(&mut self, keys: &HashSet<Key>) {
        if keys.contains(&Key::Named(NamedKey::ArrowUp)) {
            self.velocity += Point::new(0, -2);
        }
        if keys.contains(&Key::Named(NamedKey::ArrowDown)) {
            self.velocity += Point::new(0, 2);
        }
    }
}
