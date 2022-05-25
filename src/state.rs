use rltk::RGB;
use rltk::{GameState, Rltk};
use specs::World;
use specs::prelude::*;

use crate::components::Player;
use crate::components::Position;
use crate::components::Renderable;
use crate::cons;
use crate::geo::Point;
use crate::resources::{Camera, PlayerPos, Lives};
use crate::systems::{MonsterAI, player_input, MapIndexing};
use crate::{systems::{projectile_system, light_system}, map::Map};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { AwaitingInput, PreRun, PlayerTurn, MonsterTurn }

pub struct MyState {
    pub ecs: World,
    pub runstate : RunState
}

impl GameState for MyState {

    fn tick(&mut self, ctx : &mut Rltk) {
        
        match self.runstate {
            RunState::PreRun => {
                self.run_systems(ctx);
                self.runstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                self.runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems(ctx);
                self.runstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems(ctx);
                self.runstate = RunState::AwaitingInput;
            }
        }

        self.update_resources();

        // render 
        ctx.cls();
        self.render(ctx);
    }
}
impl MyState {

    pub fn new() -> Self {
        Self {
            ecs: World::new(),
            runstate: RunState::PreRun,
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

        if self.runstate == RunState::MonsterTurn {
            let mut mob = MonsterAI{};
            mob.run_now(&self.ecs);
        }

        let mut mapindex = MapIndexing{};
        mapindex.run_now(&self.ecs);
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

        // UI
        let lives = self.ecs.fetch::<Lives>();
        for i in 0..lives.max { 
            ctx.set(
                cons::WIDTH as i32 - 2,
                cons::HEIGHT as i32 - 2 - i,
                RGB::named(rltk::GREY),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('♥')
            )
        }
        for i in 0..lives.count { 
            ctx.set(
                cons::WIDTH as i32 - 2,
                cons::HEIGHT as i32 - 2 - i,
                RGB::named(rltk::RED),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('♥')
            )
         }


    }
}