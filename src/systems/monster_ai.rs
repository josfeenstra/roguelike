use std::ptr::swap;

use specs::prelude::*;
use rltk::{console, RandomNumberGenerator};

use crate::{components::{Position, Monster, Direction, Player, Renderable}, resources::PlayerPos, map::{Map, Tile}, util::Dir};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, PlayerPos>,
                        ReadExpect<'a, Map>,
                        WriteExpect<'a, RandomNumberGenerator>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Renderable>,
                        WriteStorage<'a, Direction>);

    fn run(&mut self, data : Self::SystemData) {
        let (_pos, map, mut rng, mut poss, mobs, mut rends, mut dirs) = data;
        
        // basic AI: move around, dont bump into things
        for (_mob, pos, dir) in (&mobs, &mut poss, &mut dirs).join() {
            let vector = dir.dir.vector();
            let pt = pos.to_point();
            let next = pt.add(&vector);
            if map.is_free_at(next) {
                pos.x += vector.x;    
                pos.y += vector.y;
            } else {
                // change direction semi randomly
                let mut left = dir.dir.next();
                let mut right = dir.dir.prev();
                if rng.range(0, 2) > 0 {
                    let temp = left;
                    left = right;
                    right = temp;
                }

                if map.is_free_at(pt.add(&left.vector())) {
                    dir.dir = left;
                } else if map.is_free_at(pt.add(&right.vector())) {
                    dir.dir = right;
                } else {
                    dir.dir = left;
                }
            }
        }

        // fix the looks
        for (_mob, rend, dir) in (&mobs, &mut rends, &mut dirs).join() {
            rend.glyph = match dir.dir {
                Dir::Left  => rltk::to_cp437('<'),
                Dir::Right => rltk::to_cp437('>'),
                Dir::Up    => rltk::to_cp437('^'),
                Dir::Down  => rltk::to_cp437('v'),
            }
        }
    }
}