use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


const WIDTH: i32 = 50;
const HEIGHT: i32 = 30;
const HW: i32 = WIDTH / 2;
const HH: i32 = HEIGHT / 2;

///////////////////////////////////////////////////////////

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {

    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }
}

///////////////////////////////////////////////////////////

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    foreground: RGB,
    background: RGB,
}

impl Renderable {
    
    fn new(glyph: rltk::FontCharType, foreground: RGB, background: RGB) -> Self {
        Self {glyph, foreground, background}
    }
}

///////////////////////////////////////////////////////////

#[derive(Component, Debug)]
struct Player {
}

impl Player {
    
    fn new() -> Self {
        Self {}
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World, c: char) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut rends = ecs.write_storage::<Renderable>();

    for (_player, pos, rends) in (&mut players, &mut positions, &mut rends).join() {
        pos.x = min(WIDTH-1 , max(0, pos.x + delta_x));
        pos.y = min(HEIGHT-1, max(0, pos.y + delta_y));
        rends.glyph = rltk::to_cp437(c);
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left  => try_move_player(-1, 0, &mut gs.ecs, '◄'),
            VirtualKeyCode::Right => try_move_player(1,  0, &mut gs.ecs, '►'),
            VirtualKeyCode::Up    => try_move_player(0, -1, &mut gs.ecs, '▲'),
            VirtualKeyCode::Down  => try_move_player(0,  1, &mut gs.ecs, '▼'),
            _ => {}
        },
    }
}

///////////////////////////////////////////////////////////

struct State {
    ecs: World,
}

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, HH + 1, "--------------------");
    ctx.print(4, HH + 3, "> Play");
    ctx.print(4, HH + 4, "  Options");
    ctx.print(4, HH + 5, "  Quit");
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        print_menu(ctx);
        render(self, ctx);
    }
}

fn render(state: &mut State, ctx : &mut Rltk) {
    let positions = state.ecs.read_storage::<Position>();
    let renderables = state.ecs.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
        ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
    }
}


fn draw(ecs: &mut World, x: i32, y: i32, c: char) {
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
    draw(ecs, 7,6,'▲');
    draw(ecs, 8,7,'►');
    draw(ecs, 6,7,'◄');
    draw(ecs, 7,8,'▼');

    draw(ecs, 10,9,'╗');
    draw(ecs, 10,10,'║');
    draw(ecs, 10,11,'╝');
    draw(ecs, 8,9,'╔');
    draw(ecs, 8,10,'║');
    draw(ecs, 8,11,'╚');
    draw(ecs, 9,9,'═');
    draw(ecs, 9,11,'═');
    draw(ecs, 9,10,'▒');

    draw(ecs, 20,3,'░');
    draw(ecs, 20,4,'•');
    draw(ecs, 20,5,'◘');
    
    // ╣║╗╝╚╔╩╦╠═╬
}

fn main() -> rltk::BError {

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    drawing_things(&mut gs.ecs);

    gs.ecs
        .create_entity()
        .with(Position::new(20, 20))
        .with(Renderable::new(
            rltk::to_cp437('►'), 
            RGB::named((255,0,0)), 
            RGB::named((0,0,0))))
        .with(Player::new())
        .build();

    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(WIDTH, HEIGHT).unwrap()
        .with_title("Roguelike Tutorial")
        .build()?;

    rltk::main_loop(context, gs)
}