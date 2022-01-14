use nalgebra::Vector2;
use sdl2::rect::Rect;

pub(crate) trait CenterOnVector {
    fn center_on_vector(&mut self, vector: Vector2<i32>);
}

impl CenterOnVector for Rect {
    fn center_on_vector(&mut self, vector: Vector2<i32>) {
        self.center_on((vector.x, vector.y));
    }
}
