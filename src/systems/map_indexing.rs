use std::ptr::swap;

use specs::prelude::*;
use rltk::{console, RandomNumberGenerator};

use crate::{components::{Position, Monster, Direction, Player, Renderable, Solid}, resources::PlayerPos, map::{Map, Tile}, util::Dir};

pub struct MapIndexing {}

impl<'a> System<'a> for MapIndexing {
    type SystemData = ( WriteExpect<'a, Map>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Solid>
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, posses, solids) = data;
        
        map.clear_all_entities();

        // basic AI: move around, dont bump into things
        for (pos, _) in (&posses, &solids).join() {
            map.apply_entity(pos.x, pos.y);
        }
    }
}