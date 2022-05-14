use std::borrow::{Borrow, BorrowMut};

use crate::{cons, dir::{Dir, dir_to_xy}, matrix::Matrix};
use rltk::RGB;

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

    pub fn render(&self, ctx : &mut rltk::Rltk) {
    
        let mut y = 0;
        let mut x = 0;
        for tile in self.tiles.data.iter() {
            // Render a tile depending upon the tile type
            match tile {
                Tile::Floor => {
                    ctx.set(x, y, 
                        RGB::from_u8(8, 30, 140), 
                        cons::RGB_BACKGROUND, 
                        rltk::to_cp437('#')); // •
                }
                Tile::Wall => {
                    ctx.set(x, y, 
                        RGB::from_u8(0, 255, 0), 
                        cons::RGB_BACKGROUND, 
                        rltk::to_cp437('#'));
                }
                Tile::Abyss => {
                    ctx.set(x, y, 
                        RGB::from_u8(10, 10, 10), 
                        RGB::from_u8(0, 0, 0), 
                        rltk::to_cp437(' '));
                }
            }
    
            // Move the coordinates
            x += 1;
            if x > self.tiles.width - 1 {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn apply_push(&mut self, x: i32, y: i32, dir: Dir) -> PushResult {
    
        let (dx, dy) = dir_to_xy(dir);
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




