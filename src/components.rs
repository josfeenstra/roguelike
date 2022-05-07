use specs::prelude::*;
use specs_derive::Component;
use rltk::RGB;

use crate::dir::Dir;

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

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

impl Renderable {
    
    pub fn new(glyph: rltk::FontCharType, foreground: RGB, background: RGB) -> Self {
        Self {glyph, foreground, background}
    }
}


#[derive(Component, Debug)]
pub struct Projectile {
    pub dir: Dir,
    pub lifetime: i32,
}
