use specs::prelude::*;
use specs_derive::Component;
use crate::{geo::Point, util::Dir};

#[derive(Component, Debug)]
pub struct Direction {
    pub dir: Dir,
}

#[derive(Component)]
pub struct Monster {}


pub enum ViewShedKind {
    Radial,
    Cone,
    Line,
}


#[derive(Component)]
pub struct Viewshed {
    kind: ViewShedKind,
    visible: Vec<usize>,
    radius: f32,
    arc: f32,
    dirty: bool,
}


#[derive(Component)]
pub struct Projectile {
    pub dir: Dir,
    pub lifetime: i32,
}



// global storage things

pub struct Camera {
    pub offset: Point,
}
