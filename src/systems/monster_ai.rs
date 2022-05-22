use specs::prelude::*;
use rltk::{console};

use crate::components::{Position, Monster, Direction};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>,
                        ReadStorage<'a, Direction>);

    fn run(&mut self, data : Self::SystemData) {
        let (poss, mobs, dirs) = data;
        
        for (_pos, _monster, _dir) in (&poss, &mobs, &dirs).join() {
            console::log("Monster considers its own existence");
        }
    }
}