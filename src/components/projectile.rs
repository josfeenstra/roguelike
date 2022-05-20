use specs::prelude::*;
use specs_derive::Component;

use crate::util::Dir;

#[derive(Component, Debug)]
pub struct Projectile {
    pub dir: Dir,
    pub lifetime: i32,
}