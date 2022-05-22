use rltk::RGB;
use rltk::{GameState, Rltk};
use specs::World;
use specs::prelude::*;

use crate::components::Player;
use crate::components::Position;
use crate::components::Renderable;
use crate::cons;
use crate::geo::Point;
use crate::resources::{Camera, PlayerPos};
use crate::systems::{MonsterAI, player_input};
use crate::{systems::{projectile_system, light_system}, map::Map};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct MyState {
    pub ecs: World,
    pub runstate : RunState
}

impl GameState for MyState {

    fn tick(&mut self, ctx : &mut Rltk) {
        
        // logic 
        if self.runstate == RunState::Running {
            self.run_systems(ctx);
            self.update_resources();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
            self.update_resources();
        }
        
        // render 
        ctx.cls();
        self.render(ctx);
    }
}
impl MyState {

    pub fn new() -> Self {
        Self {
            ecs: World::new(),
            runstate: RunState::Running,
        }   
    }

    fn update_resources(&mut self) {
        let players = self.ecs.read_storage::<Player>();
        let positions = self.ecs.read_storage::<Position>();

        let mut cam = self.ecs.fetch_mut::<Camera>();
        let mut player_pos = self.ecs.fetch_mut::<PlayerPos>();

        cam.offset = Point::new(cons::WIDTH as i32 / 2,cons::HEIGHT as i32 / 2);

        for (pos, player) in (&positions, &players).join() {
            player_pos.pos.set(pos.x, pos.y);
            cam.offset.addn(-pos.x, -pos.y);
        }
    }

    fn run_systems(&mut self, ctx : &mut Rltk) {
        projectile_system(self);
        light_system(self);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn render(&mut self, ctx : &mut Rltk) {

        let players = self.ecs.read_storage::<Player>();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Map>();
        let cam = self.ecs.fetch::<Camera>();
        
        map.render(ctx, &cam.offset);
        
        for (pos, render) in (&positions, &renderables).join() {
            let light = map.get_light(pos.x, pos.y).unwrap_or(0.0);
            if light < 0.1 { continue };
            ctx.set(pos.x + cam.offset.x, pos.y + cam.offset.y, 
                render.foreground, 
                RGB::lerp(&RGB::named(rltk::BLACK), render.background, light), 
                render.glyph);
        }
    }
}