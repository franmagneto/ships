use std::{
    cmp::{max, min},
    ops::{AddAssign, DivAssign},
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn new(x: i32, y: i32) -> Self {
        Self {
            x: clamp_position(x),
            y: clamp_position(y),
        }
    }

    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    pub(crate) fn y(&self) -> i32 {
        self.y
    }

    pub(crate) fn _set_x(&mut self, x: i32) {
        self.x = clamp_position(x);
    }

    pub(crate) fn set_y(&mut self, y: i32) {
        self.y = clamp_position(y);
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

#[derive(Clone, Copy, Debug)]
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

    pub(crate) fn from_point(point: Point, width: u32, height: u32) -> Self {
        Self {
            x: clamp_position(point.x),
            y: clamp_position(point.y),
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

    pub(crate) fn intersection(&self, other: Self) -> Option<Self> {
        let self_end_point = self.end_point();
        let other_end_point = other.end_point();
        let w = min(self_end_point.x, other_end_point.x) - max(self.x, other.x);
        let h = min(self_end_point.y, other_end_point.y) - max(self.y, other.y);
        if w < 0 || h < 0 {
            return None;
        }
        Some(Self {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
            w,
            h,
        })
    }

    fn end_point(&self) -> Point {
        Point {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }

    pub(crate) fn x(&self) -> i32 {
        self.x
    }

    pub(crate) fn y(&self) -> i32 {
        self.y
    }

    pub(crate) fn w(&self) -> i32 {
        self.w
    }

    pub(crate) fn h(&self) -> i32 {
        self.h
    }
}

fn max_int_value() -> u32 {
    i32::MAX as u32 / 2
}

fn min_int_value() -> i32 {
    i32::MIN / 2
}

fn clamp_size(val: u32) -> u32 {
    max(1, min(max_int_value(), val))
}

fn clamp_position(val: i32) -> i32 {
    min(max_int_value() as i32, max(min_int_value(), val))
}
