use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::{cmp::{max, min}, borrow::BorrowMut};
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

#[derive(Component, Debug, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down
}

#[derive(Component, Debug)]
struct Projectile {
    dir: Dir,
    lifetime: i32,
}

///////////////////////////////////////////////////////////

#[derive(Component, Debug)]
struct Player {
    dir: Dir,
}

impl Player {
    
    fn new() -> Self {
        Self { dir: Dir::Left }
    }
}

fn try_move_player(dir: Dir, ecs: &mut World) {

    let char = match dir {
        Dir::Left  => '◄',
        Dir::Right => '►',
        Dir::Up    => '▲',
        Dir::Down  => '▼',
    };

    let (dx, dy) = dir_to_xy(dir);

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut rends = ecs.write_storage::<Renderable>();

    for (player, pos, rends) in (&mut players, &mut positions, &mut rends).join() {
        pos.x = min(WIDTH-1 , max(0, pos.x + dx));
        pos.y = min(HEIGHT-1, max(0, pos.y + dy));
        player.dir = dir;
        rends.glyph = rltk::to_cp437(char);
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
            rltk::to_cp437('.'), 
            RGB::named(rltk::GRAY), 
            RGB::named(rltk::BLACK)))
        .build();
}

fn dir_to_xy(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Left => (-1, 0),
        Dir::Right => (1, 0),
        Dir::Up => (0, -1),
        Dir::Down => (0, 1),
    }
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

///////////////////////////////////////////////////////////

struct State {
    ecs: World,
}

fn print_menu(ctx : &mut Rltk) {
    ctx.print(4, HH + 0, "Welcome, Dungeoneer!");
    ctx.print(4, HH + 1, "|------------------|");
    ctx.print(4, HH + 3, "> Play");
    ctx.print(4, HH + 4, "  Options");
    ctx.print(4, HH + 5, "  Quit");
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        moveProjectiles(self, ctx);
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

fn moveProjectiles(state: &mut State, ctx : &mut Rltk) {

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