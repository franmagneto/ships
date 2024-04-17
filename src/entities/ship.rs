use crate::graphics::{
    geometry::{Point, Rect},
    sprite::Sprite,
};

use super::base_entity::{Entity, Renderable};

pub(crate) struct Ship {
    sprite: Sprite,
    position: Point,
    velocity: Point,
    rect: Rect,
}

impl Ship {
    pub(crate) fn new() -> Self {
        Self {
            sprite: Sprite::new("assets/ship.png"),
            position: Point::new(16, 112),
            velocity: Point::new(0, 0),
            rect: Rect::new(0, 0, 16, 16),
        }
    }

    pub(crate) fn go_up(&mut self) {
        self.velocity += Point::new(0, -2);
    }

    pub(crate) fn go_down(&mut self) {
        self.velocity += Point::new(0, 2);
    }
}

impl Entity for Ship {
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

impl Renderable for Ship {
    fn sprite(&self) -> &Sprite {
        &self.sprite
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}
