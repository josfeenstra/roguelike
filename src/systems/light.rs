use crate::{State, components::{Position}, map::{Map, Tile}, player::Player};
use specs::Entity;
use specs::prelude::*;

pub fn light_system(state: &mut State) {

    let mut positions = state.ecs.read_storage::<Position>();
    let mut players = state.ecs.read_storage::<Player>();

    let mut map = state.ecs.fetch_mut::<Map>();

    map.make_all_invisible();

    for (pos, player) in (&positions, &players).join() {
        // 1 build arc
        // 2 build lines between arc 
        // 3 make all those points visible
    }
}