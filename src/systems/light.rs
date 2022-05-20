use crate::{components::{Position, Player}, map::{Map, Tile}, state::MyState};
use specs::Entity;
use specs::prelude::*;

pub fn light_system(state: &mut MyState) {

    let mut positions = state.ecs.read_storage::<Position>();
    let mut players = state.ecs.read_storage::<Player>();

    let mut map = state.ecs.fetch_mut::<Map>();

    map.make_all_invisible();

    for (pos, player) in (&positions, &players).join() {
        
        // for p in circle.to_grid_arc(dir.rad() - range, dir.rad() + range) {
        //     let line = Line::new(circle.center.clone(), p);
        //     for l in line.to_grid() {
        //         map.
        //     }
        //     spawn(ecs, line.to.x, line.to.y, 'A');
        // }

        // 1 build arc
        // 2 build lines between arc 
        // 3 make all those points visible
    }
}