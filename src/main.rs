#![allow(dead_code)]

/**
 * main contains main, but also all non-refactored stuff 
 */

use map::Tile;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod systems;
mod geo;
mod map;
mod cons;
mod dir;
mod components;
mod player;
mod matrix;
mod js;

use crate::map::*;
use crate::components::*;
use crate::player::*;
use geo::Circle;
use geo::Point;
use dir::Dir;
use geo::Line;
use systems::projectile_system;

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, cons::HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, cons::HH + 1, "|------------------|");
    ctx.print(4, cons::HH + 3, "> Play");
    ctx.print(4, cons::HH + 4, "  Levels");
    ctx.print(4, cons::HH + 5, "  Options");
    ctx.print(4, cons::HH + 6, "  Quit");
}
pub struct State {
    ecs: World,
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        projectile_system(self);
        self.render(ctx);
    }
}
impl State {

    fn render(&mut self, ctx : &mut Rltk) {

        let map = self.ecs.fetch::<Map>();
        map.render(ctx);

        // render entities on top
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
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

    // for i in 0..256 {

    //     let x = i % 16;
    //     let y = i / 16;

    //     ecs
    //     .create_entity()
    //     .with(Position::new(0 + x, y))
    //     .with(Renderable::new(
    //         i.try_into().unwrap(), 
    //         RGB::named(rltk::GREEN), 
    //         RGB::named(rltk::BLACK)))
    //     .build();
    // }

    let circle = Circle::new(Point::new(10, 10), 7.5);
    let dir = Dir::Down;
    let range = cons::PI * 0.49;
    for p in circle.to_grid_arc(dir.rad() - range, dir.rad() + range) {
        let line = Line::new(circle.center.clone(), p);
        for l in line.to_grid() {
            spawn(ecs, l.x, l.y, 'L');
        }
        spawn(ecs, line.to.x, line.to.y, 'A');
    }




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
        .with(Position::new(17, 17))
        .with(Renderable::new(
            rltk::to_cp437('►'), 
            RGB::named((255,0,0)), 
            cons::RGB_BACKGROUND))
        .with(Player::new())
        .build();

    // render the world
    let maze = Map::new_maze(cons::WIDTH, cons::HEIGHT);
    gs.ecs.insert(maze);

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(cons::WIDTH, cons::HEIGHT)
        .unwrap()
        .with_title("Roguelike")
        .build()?;
        
    // context.with_post_scanlines(true);

    rltk::main_loop(context, gs)
}