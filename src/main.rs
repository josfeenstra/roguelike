#![allow(dead_code)]

use map::Tile;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod map;
mod cons;
mod dir;
mod components;
mod player;
mod matrix;

use crate::map::*;
use crate::dir::*;
use crate::components::*;
use crate::player::*;

///////////////////////////////////////////////////////////

pub struct State {
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
        move_projectiles(self);
        self.render(ctx);
    }
}
impl State {

    fn render(&mut self, ctx : &mut Rltk) {
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        map.render(ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}

pub fn move_projectiles(state: &mut State) {

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
            let (dx, dy) = dir_to_xy(proj.dir);
            let (nx, ny) = (pos.x + dx, pos.y + dy);
            
            let next_tile = map.get_tiles().get(nx, ny).unwrap_or(Tile::Wall);
            let next_tile_free = next_tile == Tile::Abyss || next_tile == Tile::Floor; 
            if next_tile_free {
                pos.x += dx;
                pos.y += dy;
            } else {
                let _res = map.apply_push(nx, ny, proj.dir);
                removed.push(e);
            }
        }
    }

    for r in removed {
        state.ecs.delete_entity(r).expect("could not delete entity...");
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
        .with(Position::new(0 + x, y))
        .with(Renderable::new(
            i.try_into().unwrap(), 
            RGB::named(rltk::GREEN), 
            RGB::named(rltk::BLACK)))
        .build();
    }

    // spawn(ecs, 7,6,'▲');
    // spawn(ecs, 8,7,'►');
    // spawn(ecs, 6,7,'◄');
    // spawn(ecs, 7,8,'▼');
    // spawn(ecs, 10,9,'╗');
    // spawn(ecs, 10,10,'║');
    // spawn(ecs, 10,11,'╝');
    // spawn(ecs, 8,9,'╔');
    // spawn(ecs, 8,10,'║');
    // spawn(ecs, 8,11,'╚');
    // spawn(ecs, 9,9,'═');
    // spawn(ecs, 9,11,'═');
    // spawn(ecs, 9,10,'▒');
    // spawn(ecs, 20,3,'░');
    // spawn(ecs, 20,4,'•');
    // spawn(ecs, 20,5,'◘');
    
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

    drawing_things(&mut gs.ecs);

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
    let mut context = RltkBuilder::simple(cons::WIDTH, cons::HEIGHT)
        .unwrap()
        .with_title("Roguelike")
        .build()?;
        
    context.with_post_scanlines(true);

    rltk::main_loop(context, gs)
}