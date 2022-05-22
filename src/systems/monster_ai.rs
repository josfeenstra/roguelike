use specs::prelude::*;
use rltk::{console};

use crate::components::{Position, Monster};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = ( ReadStorage<'a, Position>,
                        ReadStorage<'a, Monster>);

    fn run(&mut self, data : Self::SystemData) {
        let (pos, monster) = data;
        
        for (pos,_monster) in (&pos, &monster).join() {
            // console::log("Monster considers their own existence");
        }
    }
}