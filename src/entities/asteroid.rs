use super::base_entity::{Entity, Renderable};
use crate::utils::CenterOnVector;
use nalgebra::{vector, Vector2};
use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub(crate) struct Asteroid<'a> {
    graphics: Texture<'a>,
    rect: Rect,
    position: Vector2<f64>,
    velocity: Vector2<f64>,
}

impl<'a> Asteroid<'a> {
    pub(crate) fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            graphics: tc.load_texture("assets/asteroid.png").unwrap(),
            rect: Rect::new(0, 0, 16, 16),
            position: vector![256.0, 100.0],
            velocity: vector![-1.0, 0.0],
        }
    }
}

impl Entity for Asteroid<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
        self.rect.center_on_vector(self.position);
    }
}

impl Renderable for Asteroid<'_> {
    fn graphics(&self) -> &Texture<'_> {
        &self.graphics
    }

    fn rect(&self) -> Rect {
        self.rect
    }
}
