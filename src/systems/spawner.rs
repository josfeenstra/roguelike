use crate::{map::Map, state::MyState, components::{Position, Renderable, Monster, Direction}, cons, util::Dir};
use rltk::RGB;
use specs::prelude::*;

pub fn spawn_monsters(state: &mut MyState, map: &Map, count: u32) {

    let mut rng = rltk::RandomNumberGenerator::new();
    for _ in 0..count {
        let x = ((rng.range(0, map.width / 2) * 2) + 1) as i32;
        let y = ((rng.range(0, map.height / 2) * 2) + 1) as i32;

        let glyph : rltk::FontCharType;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => { glyph = rltk::to_cp437('<') }
            _ => { glyph = rltk::to_cp437('>') }
        }

        state.ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{
                glyph,
                foreground: RGB::named(rltk::RED),
                background: cons::RGB_BACKGROUND,
            })
            // .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })
            .with(Monster{})
            .with(Direction{ dir: rng.rand() })
            .build();
    }

        
    
}
