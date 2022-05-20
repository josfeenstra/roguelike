/**
 * General purpose direction
 */

use specs::prelude::*;
use specs_derive::Component;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use crate::cons; // 0.8.0

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

    pub fn next(&self) -> Dir {
        match self {
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
            Dir::Up => Dir::Left,
        }    
    }

    pub fn prev(&self) -> Dir {
        match self {
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
            Dir::Up => Dir::Right,
        }    
    }

    pub fn xy(&self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }

    /// the degree in respect to the positive X axis, going counter clockwise (as conventional within 3d engines)
    pub fn degree(&self) -> i32 {
        match self {
            Dir::Right => 0,
            Dir::Up    => 90,
            Dir::Left  => 180,
            Dir::Down  => 270,
        }
    }

    pub fn radians(&self) -> f32 {
        match self {
            Dir::Right => 0.0,
            Dir::Up    => cons::HALF_PI,
            Dir::Left  => cons::PI,
            Dir::Down  => cons::HALF_PI * 3.0,
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