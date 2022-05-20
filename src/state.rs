use rltk::{GameState, Rltk};
use specs::World;
use specs::prelude::*;

use crate::components::Player;
use crate::components::Position;
use crate::components::Renderable;
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

        let _players = self.ecs.read_storage::<Player>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let offset = Point::new(10,10);
        // for (pos, _player) in (&positions, &players).join() {
        //     offset.set(-pos.x, -pos.y);
        // }

        let map = self.ecs.fetch::<Map>();
        map.render(ctx, offset);


        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.foreground, render.background, render.glyph);
        }
    }
}