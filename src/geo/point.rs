use specs::prelude::*;
use specs_derive::Component;
use crate::cons;

#[derive(Component)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    /// i'm not doing derrive clone, to keep cloning extra explicit
    pub fn clone(&self) -> Self {
        Self {x: self.x, y: self.y}
    }

    pub fn copy_from(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    // computes the angle in radians with respect to the positive x-axis
    pub fn angle(&self) -> f32 {
        f32::atan2(self.y as f32, self.x as f32) + cons::PI
    }

    #[inline]
    pub fn add(&self, other: &Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }

    #[inline]
    pub fn sub(&self, other: &Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}