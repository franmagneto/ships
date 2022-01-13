use super::base_entity::{Entity, Renderable};
use crate::utils::CenterOnVector;
use nalgebra::{vector, Vector2};
use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub(crate) struct Ship<'a> {
    graphics: Texture<'a>,
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    rect: Rect,
}

impl<'a> Ship<'a> {
    pub(crate) fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            graphics: tc.load_texture("assets/ship.png").unwrap(),
            position: vector![16.0, 112.0],
            velocity: vector![0.0, 0.0],
            rect: Rect::new(0, 0, 16, 16),
        }
    }
}

impl Entity for Ship<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
        self.rect.center_on_vector(self.position);
    }
}

impl Renderable for Ship<'_> {
    fn graphics(&self) -> &Texture {
        &self.graphics
    }

    fn position(&self) -> Vector2<f64> {
        self.position
    }

    fn velocity(&self) -> Vector2<f64> {
        self.velocity
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}
