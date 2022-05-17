/**
 * General purpose direction
 */

use specs::prelude::*;
use specs_derive::Component;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
}; // 0.8.0

#[derive(Component, Debug, Copy, Clone, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down
}

impl Dir {

    pub fn from_num(i: i32) -> Dir {
        assert!(i > -1 && i < 4);
        match i {
            0 => Dir::Left,
            1 => Dir::Down,
            2 => Dir::Right,
            _ => Dir::Up,
        }
    }

    pub fn next(self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
        }    
    }

    pub fn prev(self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
        }    
    }

    pub fn to_xy(self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }
}

impl Distribution<Dir> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dir {
        
        match rng.gen_range(0..=3) {
            0 => Dir::Left,
            1 => Dir::Right,
            2 => Dir::Up,
            _ => Dir::Down,
        }
    }
}