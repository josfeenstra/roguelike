#![allow(dead_code)]

use map::Tile;
use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use std::{cmp::{max, min}};

mod map;
mod cons;
mod dir;
mod components;

use crate::map::*;
use crate::dir::*;
use crate::components::*;

///////////////////////////////////////////////////////////

#[derive(Component, Debug)]
struct Player {
    dir: Dir,
}

impl Player {
    
    fn new() -> Self {
        Self { dir: Dir::Right }
    }
}

fn try_move_player(dir: Dir, ecs: &mut World) {
    
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut rends = ecs.write_storage::<Renderable>();
    let map = ecs.fetch::<Map>();

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
        
        // bump into wall
        if map.get(nx, ny) != Tile::Floor { continue };

        // actually move (but not out of screen)
        pos.x = min((cons::WIDTH - 1) as i32 , max(0, nx));
        pos.y = min((cons::HEIGHT - 1) as i32, max(0, ny));
    }
}

fn get_player( ecs: &mut World) -> (Position, Dir) {
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



fn player_input(gs: &mut State, ctx: &mut Rltk) {
    
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

pub fn draw_map(map: &Map, ctx : &mut Rltk) {

    let mut y = 0;
    let mut x = 0;
    for tile in map.data.iter() {
        // Render a tile depending upon the tile type
        match tile {
            Tile::Floor => {
                ctx.set(x, y, 
                    RGB::from_u8(8, 30, 60), 
                    cons::RGB_BACKGROUND, 
                    rltk::to_cp437('#')); // •
            }
            Tile::Wall => {
                ctx.set(x, y, 
                    RGB::from_u8(0, 255, 0), 
                    cons::RGB_BACKGROUND, 
                    rltk::to_cp437('#'));
            }
            Tile::Abyss => {
                ctx.set(x, y, 
                    RGB::from_u8(10, 10, 10), 
                    RGB::from_u8(0, 0, 0), 
                    rltk::to_cp437(' '));
            }
        }

        // Move the coordinates
        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}

///////////////////////////////////////////////////////////

struct State {
    ecs: World,
}

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, cons::HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, cons::HH + 1, "|------------------|");
    ctx.print(4, cons::HH + 3, "> Play");
    ctx.print(4, cons::HH + 4, "  Levels");
    ctx.print(4, cons::HH + 5, "  Options");
    ctx.print(4, cons::HH + 6, "  Quit");
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        move_projectiles(self, ctx);
        self.render(ctx);
    }
}
impl State {

    fn render(&mut self, ctx : &mut Rltk) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let maze = self.ecs.fetch::<Map>();

        draw_map(&maze, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}



fn move_projectiles(state: &mut State, _ctx : &mut Rltk) {

    let mut positions = state.ecs.write_storage::<Position>();
    let mut projectiles = state.ecs.write_storage::<Projectile>();

    for (mut pos, mut proj) in (&mut positions, &mut projectiles).join() {
        proj.lifetime -= 1;
        if (proj.lifetime < 0) {
            // kill it. but how?
            // state.ecs.delete_entity()
        }
        let (dx, dy) = dir_to_xy(proj.dir);
        pos.x += dx;
        pos.y += dy;
    }
}

fn spawn(ecs: &mut World, x: i32, y: i32, c: char) {
    ecs
    .create_entity()
    .with(Position::new(x, y))
    .with(Renderable::new(
        rltk::to_cp437(c), 
        RGB::named(rltk::GREEN), 
        RGB::named(rltk::BLACK)))
    .build();
}

fn drawing_things(ecs: &mut World) {

    for i in 0..256 {

        let x = i % 16;
        let y = i / 16;

        ecs
        .create_entity()
        .with(Position::new(30 + x, y))
        .with(Renderable::new(
            i.try_into().unwrap(), 
            RGB::named(rltk::GREEN), 
            RGB::named(rltk::BLACK)))
        .build();
    }

    spawn(ecs, 7,6,'▲');
    spawn(ecs, 8,7,'►');
    spawn(ecs, 6,7,'◄');
    spawn(ecs, 7,8,'▼');

    spawn(ecs, 10,9,'╗');
    spawn(ecs, 10,10,'║');
    spawn(ecs, 10,11,'╝');
    spawn(ecs, 8,9,'╔');
    spawn(ecs, 8,10,'║');
    spawn(ecs, 8,11,'╚');
    spawn(ecs, 9,9,'═');
    spawn(ecs, 9,11,'═');
    spawn(ecs, 9,10,'▒');

    spawn(ecs, 20,3,'░');
    spawn(ecs, 20,4,'•');
    spawn(ecs, 20,5,'◘');
    
    // ╣║╗╝╚╔╩╦╠═╬
}

fn main() -> rltk::BError {

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Projectile>();

    // drawing_things(&mut gs.ecs);

    // create the player
    gs.ecs
        .create_entity()
        .with(Position::new(20, 20))
        .with(Renderable::new(
            rltk::to_cp437('►'), 
            RGB::named((255,0,0)), 
            RGB::named((0,0,0))))
        .with(Player::new())
        .build();

    // render the world
    let maze = Map::new_random(cons::WIDTH, cons::HEIGHT, 200, 100);
    gs.ecs.insert(maze);

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(cons::WIDTH, cons::HEIGHT).unwrap()
        .with_title("Roguelike Tutorial")
        .build()?;

    rltk::main_loop(context, gs)
}