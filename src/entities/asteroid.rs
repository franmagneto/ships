use super::base_entity::{Entity, Renderable};
use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub(crate) struct Asteroid<'a> {
    graphics: Texture<'a>,
    rect: Rect,
    position: Point,
    velocity: Point,
}

impl<'a> Asteroid<'a> {
    pub(crate) fn new(tc: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            graphics: tc.load_texture("assets/asteroid.png").unwrap(),
            rect: Rect::new(0, 0, 16, 16),
            position: Point::new(256, 100),
            velocity: Point::new(-1, 0),
        }
    }
}

impl Entity for Asteroid<'_> {
    fn update(&mut self) {
        self.position += self.velocity;
        self.rect.center_on(self.position);
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
