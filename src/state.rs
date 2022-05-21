use rltk::{GameState, Rltk};
use specs::World;
use specs::prelude::*;

use crate::components::Camera;
use crate::components::Player;
use crate::components::Position;
use crate::components::Renderable;
use crate::cons;
use crate::geo::Point;
use crate::{components::player_input, systems::{projectile_system, light_system}, map::Map};



pub struct MyState {
    pub ecs: World,
}

impl GameState for MyState {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        projectile_system(self);
        light_system(self);
        self.render(ctx);
    }
}
impl MyState {

    fn render(&mut self, ctx : &mut Rltk) {

        let players = self.ecs.read_storage::<Player>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Map>();
        let mut cam = self.ecs.fetch_mut::<Camera>();

        // TODO abstract away camera system
        // create the 'camera', which is always just an offset
        // cam.offset = Point::new(0, 0);
        cam.offset = Point::new(cons::WIDTH as i32 / 2,cons::HEIGHT as i32 / 2);
        for (pos, _player) in (&positions, &players).join() {
            cam.offset.addn(-pos.x, -pos.y);
        }
        
        map.render(ctx, &cam.offset);
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x + cam.offset.x, pos.y + cam.offset.y, render.foreground, render.background, render.glyph);
        }
    }
}