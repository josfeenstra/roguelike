#![allow(dead_code, unused_imports, unused_variables)]

use components::Position;
use components::Renderable;
/**
 * main contains main, but also all non-refactored stuff 
 */

use rltk::{Rltk, RGB};
use specs::prelude::*;

mod resources;
mod geo;
mod map;
mod cons;
mod util;

mod systems;
mod components;
mod state;

use geo::Circle;
use geo::Point;
use geo::Line;
use util::Dir;

use crate::components::Direction;
use crate::components::Monster;
use crate::components::Player;
use crate::components::Projectile;
use crate::map::Map;
use crate::resources::Camera;
use crate::resources::PlayerPos;
use crate::state::MyState;
use crate::systems::spawn_monsters;

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, cons::HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, cons::HH + 1, "|------------------|");
    ctx.print(4, cons::HH + 3, "> Play");
    ctx.print(4, cons::HH + 4, "  Levels");
    ctx.print(4, cons::HH + 5, "  Options");
    ctx.print(4, cons::HH + 6, "  Quit");
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

    // let circle = Circle::new(Point::new(10, 10), 7.5);
    // let dir = Dir::Down;
    // let range = cons::PI * 0.49;
    // for p in circle.to_grid_arc(dir.rad() - range, dir.rad() + range) {
    //     let line = Line::new(circle.center.clone(), p);
    //     for l in line.to_grid() {
    //         spawn(ecs, l.x, l.y, 'L');
    //     }
    //     spawn(ecs, line.to.x, line.to.y, 'A');
    // }

    // spawn(ecs, 7,6,'█');
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
    
    // o╣║╗╝╚╔╩╦╠═╬
    // ╨╞╡
}

/////////////////////////////////////////////////////////////////


fn make_player(ecs: &mut World) {
    ecs
        .create_entity()
        .with(Position::new(3, 3))
        .with(Renderable::new(
            rltk::to_cp437('►'), 
            RGB::named(rltk::YELLOW), 
            cons::RGB_BACKGROUND))
        .with(Player {})
        .with(Direction { dir: Dir::Down})
        .build();
}

fn main() -> rltk::BError {

    // init the state
    let mut gs = MyState::new();

    // register all used components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Direction>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Projectile>();
    gs.ecs.register::<Monster>();

    // create the player
    make_player(&mut gs.ecs);

    // create the map resource
    let maze = Map::new_maze(cons::WIDTH, cons::HEIGHT);
    spawn_monsters(&mut gs, &maze, 5);
    gs.ecs.insert(maze);

    // create other resources
    gs.ecs.insert(Camera { offset: Point::new(0,0) });
    gs.ecs.insert(PlayerPos { pos: Point::new(0,0) });

    // spawn the window
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(cons::WIDTH, cons::HEIGHT)
        .unwrap()
        .with_title("Roguelike")
        .build()?;
        
    // context.with_post_scanlines(true);
    rltk::main_loop(context, gs)
}