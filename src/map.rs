use std::{borrow::{Borrow, BorrowMut}};

use crate::{cons, dir::{Dir}, matrix::Matrix, components::Position, js};
use rand::prelude::SliceRandom;
use rltk::{RGB, RandomNumberGenerator};

#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
    Abyss,
}

#[derive(PartialEq)]
pub enum PushResult {
    Free, // nothing to push to begin with
    Pushed, // we just pushed something to the next tile
    Blocked, // something could be pushed, but was blocked by something standing behind it
    Tumble, // we just pushed something down to a lower level
}

// a space on the map
pub struct Space {
    tile: Tile,
    visible: bool,
}

pub struct Map {
    tiles: Matrix<Tile>
}

impl Map {

    pub fn get_tiles(&self) -> &Matrix<Tile> {
        self.tiles.borrow()
    }

    pub fn get_tiles_mut(&mut self) -> &mut Matrix<Tile> {
        self.tiles.borrow_mut()
    }

    pub fn new_random(width: usize, height: usize, num_walls: u32, num_holes: u32) -> Map {

        let mut tiles = Matrix::new(width, height, Tile::Floor);
    
        let w = width as i32;
        let h = height as i32;
    
        for i in 0..w {
            tiles.set(i, 0, Tile::Wall);
            tiles.set(i, h-1, Tile::Wall);
        } 
        for i in 0..h {
            tiles.set(0, i, Tile::Wall);
            tiles.set(w-1, i, Tile::Wall);
        } 
    
        let mut rng = rltk::RandomNumberGenerator::new();
    
        for _i in 0..num_walls as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            tiles.set(x, y, Tile::Wall);
        }
    
        for _i in 0..num_holes as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            tiles.set(x, y, Tile::Abyss);
        }
    
        Map { tiles }
    }

    pub fn new_empty(width: usize, height: usize, filler: Tile, border: bool) -> Map {
        let mut tiles = Matrix::new(width, height, filler);
    
        let w = width as i32;
        let h = height as i32;
        
        if border {
            for i in 0..w {
                tiles.set(i, 0, Tile::Wall);
                tiles.set(i, h-1, Tile::Wall);
            } 
            for i in 0..h {
                tiles.set(0, i, Tile::Wall);
                tiles.set(w-1, i, Tile::Wall);
            } 
        }

        Map { tiles }
    }

    pub fn new_maze(width: usize, height: usize) -> Map {
        
        fn to_even(n: i32) -> i32 {
            n / 2 * 2
        }

        /// get only the valid directions, so we never choose something out of bounds
        fn get_valid_dirs(a: &Position, width: i32, height : i32) -> Vec<Dir> {
            let mut options: Vec<Dir> = Vec::new();
            for i in 0..4 {
                let dir = Dir::from_num(i);
                let (dx, dy) = dir.to_xy();
                let (nx, ny) = (a.x + dx * 2, a.y + dy * 2);
                if nx < 1 || nx > width - 2 || ny < 1 || ny > height - 2 {
                    continue;
                }
                options.push(dir);
            }
            options
        }

        /// select a random direction with unvisited tiles
        fn try_select(rng: &mut RandomNumberGenerator, a: &Position, maze: &Map) -> Option<Dir> {
            let rand = rng.get_rng();
            let mut valid_dirs = get_valid_dirs(&a, maze.tiles.width as i32, maze.tiles.height as i32);
            valid_dirs.shuffle(rand);
            for dir in valid_dirs.iter() {
                let (dx, dy) = dir.to_xy();
                let t = maze.tiles.get(a.x + dx * 2, a.y + dy * 2).unwrap();
                if t == Tile::Floor {
                    // skip this iteration
                    continue;
                } 
                return Some(dir.clone());
            }
            None
        }

        let openness = 50; // number between 0 and 100, with 0 being very claustrofobic, and 100 being almost not a maze anymore
        let num_agents = 100;
        let num_iterations = 6;

        // create the area filled with walls
        let mut maze = Self::new_empty(width, height, Tile::Wall, true);
        
        // build a bunch of agents 
        let mut rng = RandomNumberGenerator::new();
        let mut positions: Vec<(Position, bool)> = Vec::new();

        for _ in 0..num_agents {
            let pos = Position {
                x: to_even(rng.range(0, width as i32 - 2)) + 1,
                y: to_even(rng.range(0, height as i32 - 2)) + 1,
            };
            maze.tiles.set(pos.x, pos.y, Tile::Floor);
            let continuous: bool = rng.range::<i32>(0, 100) < openness;
            positions.push((pos, continuous));
        }

        // let them walk around, digging tunnels
        for _ in 0..num_iterations {
            for (a, continuous) in positions.iter_mut() {
                let dir = match try_select(&mut rng, &a, &maze) {
                    Some(dir) => dir,
                    None => {
                        // what to do if all directions are already visisted?
                        if *continuous {
                            // go wander visited regions by selecting a valid direction
                            get_valid_dirs(&a, width as i32, height as i32)
                                .choose(rng.get_rng())
                                .unwrap()
                                .to_owned()   
                        } else {
                            // just dont go any further. TODO `a` should be deleted
                            continue;
                        }
                    },
                };

                let (dx, dy) = dir.to_xy();

                for _ in 0..2 {
                    a.x = (a.x + dx).clamp(0, width as i32);
                    a.y = (a.y + dy).clamp(0, height as i32);
                    maze.tiles.set(a.x, a.y, Tile::Floor);
                }
            }
        }

        js::print(&format!("{}", to_even(5)));

        // let them run wild for a couple of iterations
        // for _ in 0..1000 {
        //     num_agents
        // }

        // return this maze
        maze
    }

    pub fn is_free(&self, x: i32, y: i32) -> bool {
        let t = self.tiles.get(x, y).unwrap_or(Tile::Wall);
        t == Tile::Floor
    }

    pub fn render(&self, ctx : &mut rltk::Rltk) {
    
        let mut y = 0;
        let mut x = 0;
        for tile in self.tiles.data.iter() {
            // Render a tile depending upon the tile type
            match tile {
                Tile::Floor => {
                    ctx.set(x, y, 
                        cons::RGB_BACKGROUND, 
                        cons::RGB_BACKGROUND, 
                        rltk::to_cp437('#')); // •
                }
                Tile::Wall => {
                    let char = getwall(
                        self.tiles.get(x, y-1).unwrap_or(Tile::Floor),
                        self.tiles.get(x-1, y).unwrap_or(Tile::Floor),
                        self.tiles.get(x, y+1).unwrap_or(Tile::Floor),
                        self.tiles.get(x+1, y).unwrap_or(Tile::Floor),
                    );
                    ctx.set(x, y, 
                        RGB::from_u8(140, 140, 160), 
                        cons::RGB_BACKGROUND, 
                        rltk::to_cp437(char));
                }
                Tile::Abyss => {
                    ctx.set(x, y, 
                        cons::RGB_BACKGROUND, 
                        RGB::from_u8(0, 0, 0), 
                        rltk::to_cp437(' ')); //α
                }
            }
    
            // Move the coordinates
            x += 1;
            if x > self.tiles.width as i32 - 1 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn apply_push(&mut self, x: i32, y: i32, dir: Dir) -> PushResult {
    
        let (dx, dy) = dir.to_xy();
        let tile = self.tiles.get(x, y).unwrap_or(Tile::Wall);
        if tile != Tile::Floor { // bump into something?
            if tile == Tile::Wall { // bump into wall?
                let afterwall = self.tiles.get(x+dx, y+dy).unwrap_or(Tile::Wall);
                if afterwall == Tile::Wall { return PushResult::Blocked }
                if afterwall == Tile::Floor { // after wall floor? push.
                    self.tiles.set(x, y, Tile::Floor);
                    self.tiles.set(x+dx, y+dy, Tile::Wall);
                    return PushResult::Pushed;
                }
                if afterwall == Tile::Abyss { // after wall abyss? push it in
                    self.tiles.set(x, y, Tile::Floor);
                    self.tiles.set(x+dx, y+dy, Tile::Floor);
                    return PushResult::Tumble;
                }
            } 
            // bump into something else? dont go there
            return PushResult::Blocked;
        };
    
        // next tile is free!
        return PushResult::Free;
    }
}        

/// do all the walling. 
/// some characters: o╣║╗╝╚╔╩╦╠═╬╨╞╡◙
/// not sure about the singulars, the 'columns. ohwell'
fn getwall(up: Tile, left: Tile, bot: Tile, right: Tile) -> char {
    
    let i = (up == Tile::Wall) as i32 +
         (right == Tile::Wall) as i32 * 2 + 
           (bot == Tile::Wall) as i32 * 4+ 
          (left == Tile::Wall) as i32 * 8;

    match i {
        //ldru
        0b0000 => '■', // unconnected
        0b0001 => '║', 
        0b0010 => '═',  
        0b0011 => '╚', 
        0b0100 => '║', 
        0b0101 => '║',  
        0b0110 => '╔', 
        0b0111 => '╠', // filled
        0b1000 => '═', 
        0b1001 => '╝', 
        0b1010 => '═',  
        0b1011 => '╩', 
        0b1100 => '╗', 
        0b1101 => '╣',  
        0b1110 => '╦', 
        0b1111 => '╬', // filled
        _ => '■'
    }
}
