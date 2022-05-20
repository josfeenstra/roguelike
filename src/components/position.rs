use specs::prelude::*;
use specs_derive::Component;

// NOTE: its completely stupid why this is not a Point
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {

    pub fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}
