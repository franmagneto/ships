use crate::graphics::{
    geometry::{Point, Rect},
    sprite::Sprite,
};

use super::base_entity::{Entity, Renderable};

pub(crate) struct Asteroid {
    sprite: Sprite,
    rect: Rect,
    position: Point,
    velocity: Point,
}

impl Asteroid {
    pub(crate) fn new() -> Self {
        Self {
            sprite: Sprite::new("assets/asteroid.png"),
            rect: Rect::new(0, 0, 16, 16),
            position: Point::new(256, 100),
            velocity: Point::new(-1, 0),
        }
    }
}

impl Entity for Asteroid {
    fn update(&mut self) {
        self.position += self.velocity;
        self.rect.center_on(self.position);
    }
}

impl Renderable for Asteroid {
    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}
