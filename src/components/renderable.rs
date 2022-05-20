use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

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
