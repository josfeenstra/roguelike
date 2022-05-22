use specs::prelude::*;
use rltk::{console};

use crate::{components::{Position, Monster, Direction, Player}, resources::PlayerPos, map::{Map, Tile}, util::Dir};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadExpect<'a, PlayerPos>,
                        ReadExpect<'a, Map>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Direction>);

    fn run(&mut self, data : Self::SystemData) {
        let (pos, map, mut poss, mobs, mut dirs) = data;
        
        for (pos, _monster, d) in (&mut poss, &mobs, &mut dirs).join() {
            // let vector = d.dir.vector();
            let next = pos.to_point().add(&Dir::Left.vector());
            
            if map.is_free_at(next) {
                pos.x += 1;    
                // pos.y += vector.y;
            } else {
                // d.dir = d.dir.next();
            }
            console::log(format!("{:?} {:?} ", next, d.dir));
        }
    }
}