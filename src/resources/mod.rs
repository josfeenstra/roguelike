use crate::geo::Point;


// global resources


pub struct Camera {
    pub offset: Point,
}

pub struct PlayerPos {
    pub pos: Point,
}

pub struct Lives {
    pub count: i32,
    pub max: i32,
}