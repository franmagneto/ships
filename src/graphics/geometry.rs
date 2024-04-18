use std::{
    cmp::{max, min},
    ops::{AddAssign, DivAssign},
};

#[derive(Clone, Copy)]
pub(crate) struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = clamp_position(self.x + rhs.x);
        self.y = clamp_position(self.y + rhs.y);
    }
}

impl DivAssign<i32> for Point {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl From<Point> for (i32, i32) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl From<Rect> for Point {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl Rect {
    pub(crate) fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x: clamp_position(x),
            y: clamp_position(y),
            w: clamp_size(width) as i32,
            h: clamp_size(height) as i32,
        }
    }

    pub(crate) fn center_on<P>(&mut self, point: P)
    where
        P: Into<(i32, i32)>,
    {
        let (x, y) = point.into();
        self.x = clamp_position(clamp_position(x) - self.w / 2);
        self.y = clamp_position(clamp_position(y) - self.h / 2);
    }
}

fn max_int_value() -> u32 {
    i32::max_value() as u32 / 2
}

fn min_int_value() -> i32 {
    i32::min_value() / 2
}

fn clamp_size(val: u32) -> u32 {
    max(1, min(max_int_value(), val))
}

fn clamp_position(val: i32) -> i32 {
    min(max_int_value() as i32, max(min_int_value(), val))
}
