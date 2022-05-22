use std::cmp::{min, max};

use rltk::{RGB, VirtualKeyCode, Rltk};
use specs::prelude::*;

use crate::{util::Dir, components::{Position, Player, Renderable, Projectile, Direction}, map::Map, cons, state::{MyState, RunState}};


fn try_move_player(dir: Dir, ecs: &mut World) {
    
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut rends = ecs.write_storage::<Renderable>();
    let mut dirs = ecs.write_storage::<Direction>();
    let map = ecs.fetch_mut::<Map>();

    let char = match dir {
        Dir::Left  => '◄', // < ◄
        Dir::Right => '►', // > ►
        Dir::Up    => '▲', // ^ ▲
        Dir::Down  => '▼', // v ▼
    };

    let (dx, dy) = dir.xy();

    for (_player, pos, rends, d) in (&mut players, &mut positions, &mut rends, &mut dirs).join() {

        // fix dir
        d.dir = dir;
        rends.glyph = rltk::to_cp437(char);
        
        let (nx, ny) = (pos.x + dx, pos.y + dy);

        // actually move (but never out of screen)
        if map.is_free(nx, ny) {
            pos.x = min((cons::WIDTH - 1) as i32 , max(0, nx));
            pos.y = min((cons::HEIGHT - 1) as i32, max(0, ny));
        }
    }
}

fn get_player( ecs: &mut specs::World) -> (Position, Dir) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut directions = ecs.write_storage::<Direction>();

    let mut position = Position::new(0,0);
    let mut direction = Dir::Left;
    for (player, pos, dir) in (&mut players, &mut positions, &mut directions).join() {
        position.x = pos.x;
        position.y = pos.y;
        direction = dir.dir.clone();
    }
    return (position, direction);
}

fn try_player_shoot(ecs: &mut World) {
    let (pos, dir) = get_player(ecs);
    // let (dx, dy) = dir_to_xy(dir);

    ecs
        .create_entity()
        .with(Position::new(pos.x, pos.y))
        .with(Projectile {dir, lifetime: 10})
        .with(Renderable::new(
            rltk::to_cp437('◙'), 
            RGB::named(rltk::BLUE2), 
            RGB::named(rltk::BLACK)))
        .build();
}

pub fn player_input(gs: &mut MyState, ctx: &mut Rltk) -> RunState {
    
    // Player movement
    match ctx.key {
        None => { return RunState::Paused } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left  => try_move_player(Dir::Left, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(Dir::Right, &mut gs.ecs),
            VirtualKeyCode::Up    => try_move_player(Dir::Up, &mut gs.ecs),
            VirtualKeyCode::Down  => try_move_player(Dir::Down, &mut gs.ecs),
            VirtualKeyCode::Space  => try_player_shoot(&mut gs.ecs),
            _ => { return RunState::Paused }
        },
    }
    RunState::Running
}