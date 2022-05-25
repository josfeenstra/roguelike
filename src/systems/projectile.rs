use crate::{MyState, components::{Position, Projectile}, map::{Map, Tile}};
use specs::Entity;
use specs::prelude::*;

pub fn projectile_system(state: &mut MyState) {

    let mut removed : Vec<Entity> = Vec::new();
    
    // I live to please the borrow checker
    {
        let entities = state.ecs.entities();
        let mut positions = state.ecs.write_storage::<Position>();
        let mut projectiles = state.ecs.write_storage::<Projectile>();
        let mut map = state.ecs.fetch_mut::<Map>();

        for (e, mut pos, mut proj) in (&entities, &mut positions, &mut projectiles).join() {
            proj.lifetime -= 1;
            if proj.lifetime < 0 {
                removed.push(e);
                continue;
            }
            let (dx, dy) = proj.dir.xy();
            let (nx, ny) = (pos.x + dx, pos.y + dy);
            
            let next_tile = map.get_tile(nx, ny).unwrap_or(Tile::Wall);
            let next_tile_free = next_tile == Tile::Empty || next_tile == Tile::Floor; 
            if next_tile_free {
                pos.x += dx;
                pos.y += dy;
            } else {
                let _res = map.apply_push_effect(nx, ny, proj.dir);
                removed.push(e);
            }
        }
    }

    for r in removed {
        state.ecs.delete_entity(r).expect("could not delete entity...");
    }
}