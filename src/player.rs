use std::cmp::{min, max};

use crate::{dir::{Dir, dir_to_xy}, components::{Position, Renderable, Projectile}, map::{Matrix, Tile}, State, cons};
use rltk::{Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Player {
    dir: Dir,
}

impl Player {
    
    pub fn new() -> Self {
        Self { dir: Dir::Right }
    }
}

fn try_move_player(dir: Dir, ecs: &mut World) {
    
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut rends = ecs.write_storage::<Renderable>();
    let mut map = ecs.fetch_mut::<Matrix<Tile>>();

    let char = match dir {
        Dir::Left  => '◄',
        Dir::Right => '►',
        Dir::Up    => '▲',
        Dir::Down  => '▼',
    };

    let (dx, dy) = dir_to_xy(dir);

    for (player, pos, rends) in (&mut players, &mut positions, &mut rends).join() {

        // fix dir
        player.dir = dir;
        rends.glyph = rltk::to_cp437(char);
        
        let (nx, ny) = (pos.x + dx, pos.y + dy);
        
        let nb = map.get(nx, ny).unwrap_or(Tile::Wall);
        if nb != Tile::Floor { // bump into something?
            if nb == Tile::Wall { // bump into wall?
                let afterwall = map.get(nx+dx, ny+dy).unwrap_or(Tile::Wall);
                if afterwall == Tile::Wall { continue }
                if afterwall == Tile::Floor { // after wall floor? push.
                    map.set(nx, ny, Tile::Floor);
                    map.set(nx+dx, ny+dy, Tile::Wall);
                }
                if afterwall == Tile::Abyss { // after wall abyss? push it in
                    map.set(nx, ny, Tile::Floor);
                    map.set(nx+dx, ny+dy, Tile::Floor);
                }
            } else { // bump into something else? dont go there
                continue;
            } 
        };

        // actually move (but not out of screen)
        pos.x = min((cons::WIDTH - 1) as i32 , max(0, nx));
        pos.y = min((cons::HEIGHT - 1) as i32, max(0, ny));
    }
}

fn get_player( ecs: &mut specs::World) -> (Position, Dir) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    let mut position = Position::new(0,0);
    let mut direction = Dir::Left;
    for (player, pos) in (&mut players, &mut positions).join() {
        position.x = pos.x;
        position.y = pos.y;
        direction = player.dir;
    }
    return (position, direction);
}

fn try_player_shoot(ecs: &mut World) {
    let (pos, dir) = get_player(ecs);
    let (dx, dy) = dir_to_xy(dir);

    ecs
        .create_entity()
        .with(Position::new(pos.x + dx, pos.y + dy))
        .with(Projectile {dir, lifetime: 10})
        .with(Renderable::new(
            rltk::to_cp437('o'), 
            RGB::named(rltk::GRAY), 
            RGB::named(rltk::BLACK)))
        .build();
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left  => try_move_player(Dir::Left, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(Dir::Right, &mut gs.ecs),
            VirtualKeyCode::Up    => try_move_player(Dir::Up, &mut gs.ecs),
            VirtualKeyCode::Down  => try_move_player(Dir::Down, &mut gs.ecs),
            VirtualKeyCode::Space  => try_player_shoot(&mut gs.ecs),
            _ => {}
        },
    }
}