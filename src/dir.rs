/**
 * General purpose direction
 */

use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down
}

pub fn dir_to_xy(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
    }
}
