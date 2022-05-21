use crate::{components::{Position, Player}, map::{Map}, state::MyState, geo::{Circle, Point, Line}, cons, util};
use specs::prelude::*;

pub fn light_system(state: &mut MyState) {

    const radius: f32 = 9.5;
    let dropoff = 0.5;

    let positions = state.ecs.read_storage::<Position>();
    let players = state.ecs.read_storage::<Player>();

    let mut map = state.ecs.fetch_mut::<Map>();

    map.darken_all();

    for (pos, _player) in (&positions, &players).join() {
        
        let c = Circle::new(Point::new(pos.x, pos.y), radius);
        let points = c.to_grid_edge();
        for p in points {
            let line = Line::new(c.center.clone(), p);
            let lps = line.to_grid();
            // let mut scale = 0.0;
            for (i, l) in lps.iter().enumerate() {
                let scale = i as f32 / lps.len() as f32;
                
                let f = 1.0 - scale; 
                // if scale > dropoff {
                //     f = 1.0 - (scale-dropoff) * (1.0 / dropoff); 
                // }
                map.light.set(l.x, l.y, f);
                // if !map.is_free(l.x, l.y) { scale += 0.5 };
            }
            // map.light.set(line.to.x, line.to.y, 0.2);
        }

        // for p in circle.to_grid_arc(dir.rad() - range, dir.rad() + range) {
        //     let line = Line::new(circle.center.clone(), p);
        //     for l in line.to_grid() {
        //         map.
        //     }
        //     spawn(ecs, line.to.x, line.to.y, 'A');
        // }

        // 1 build arc
        // 2 build lines between arc 
        // 3 make all those points visible
    }
}