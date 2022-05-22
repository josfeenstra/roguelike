use crate::{cons, util::{Dir}, components::Position, geo::Point};
use rand::prelude::SliceRandom;
use rltk::{RGB, RandomNumberGenerator, console};

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

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,
    pub light: Vec<f32>
}

// basic data properties
impl Map {

    pub fn new(width: usize, height: usize, def_tile: Tile, def_light: f32) -> Self {
        let tiles = vec![def_tile; width * height];
        let light = vec![def_light; width * height];
        Self {width, height, tiles, light}
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) -> Option<usize> {
        let id = self.to_index(x, y)?;
        self.tiles[id] = tile;
        Some(id)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<Tile> {
        let id = self.to_index(x, y)?;
        Some(self.tiles[id])
    }

    pub fn set_light(&mut self, x: i32, y: i32, light: f32) -> Option<usize> {
        let id = self.to_index(x, y)?;
        self.light[id] = light;
        Some(id)
    }

    pub fn get_light(&self, x: i32, y: i32) -> Option<f32> {
        let id = self.to_index(x, y)?;
        Some(self.light[id])
    }

    pub fn to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some((y as usize * self.width) + x as usize)
        }
    }

    pub fn to_coord(&self, i: usize) -> (i32, i32) {
        ((i % self.width) as i32, (i / self.width) as i32)
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }
}

// advanced map business
impl Map {

    pub fn new_random(width: usize, height: usize, num_walls: u32, num_holes: u32) -> Map {

        let mut map = Map::new(width, height, Tile::Floor, 0.0);

        let w = width as i32;
        let h = height as i32;
    
        for i in 0..w {
            map.set_tile(i, 0, Tile::Wall);
            map.set_tile(i, h-1, Tile::Wall);
        } 
        for i in 0..h {
            map.set_tile(0, i, Tile::Wall);
            map.set_tile(w-1, i, Tile::Wall);
        } 
    
        let mut rng = rltk::RandomNumberGenerator::new();
    
        for _i in 0..num_walls as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            map.set_tile(x, y, Tile::Wall);
        }
    
        for _i in 0..num_holes as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            map.set_tile(x, y, Tile::Abyss);
        }
    
        map
    }

    pub fn new_empty(width: usize, height: usize, filler: Tile, border: bool) -> Map {

        let mut map = Map::new(width, height, filler, 0.0);

        let w = width as i32;
        let h = height as i32;
        
        if border {
            for i in 0..w {
                map.set_tile(i, 0,   Tile::Wall);
                map.set_tile(i, h-1, Tile::Wall);
            } 
            for i in 0..h {
                map.set_tile(0,   i, Tile::Wall);
                map.set_tile(w-1, i, Tile::Wall);
            } 
        }

        map
    }

    /// a elaborate procedure to just create a nice, maze like map.
    pub fn new_maze(width: usize, height: usize) -> Map {
        
        fn to_even(n: i32) -> i32 {
            n / 2 * 2
        }

        /// get only the valid directions, so we never choose something out of bounds
        fn get_valid_dirs(a: &Position, width: i32, height : i32) -> Vec<Dir> {
            let mut options: Vec<Dir> = Vec::new();
            for i in 0..4 {
                let dir = Dir::from_num(i);
                let (dx, dy) = dir.xy();
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
            let mut valid_dirs = get_valid_dirs(&a, maze.width as i32, maze.height as i32);
            valid_dirs.shuffle(rand);
            for dir in valid_dirs.iter() {
                let (dx, dy) = dir.xy();
                let t = maze.get_tile(a.x + dx * 2, a.y + dy * 2).unwrap();
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
            maze.set_tile(pos.x, pos.y, Tile::Floor);
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

                let (dx, dy) = dir.xy();

                for _ in 0..2 {
                    a.x = (a.x + dx).clamp(0, width as i32);
                    a.y = (a.y + dy).clamp(0, height as i32);
                    maze.set_tile(a.x, a.y, Tile::Floor);
                }
            }
        }

        console::log(&format!("{}", to_even(5)));
        // let them run wild for a couple of iterations
        // for _ in 0..1000 {
        //     num_agents
        // }

        // return this maze
        maze
    }

    pub fn is_free(&self, x: i32, y: i32) -> bool {
        let t = self.get_tile(x, y).unwrap_or(Tile::Wall);
        t == Tile::Floor
    }

    pub fn darken_all(&mut self) {
        self.light.fill(0.0)
    }

    pub fn apply_push(&mut self, x: i32, y: i32, dir: Dir) -> PushResult {
    
        let (dx, dy) = dir.xy();
        let tile = self.get_tile(x, y).unwrap_or(Tile::Wall);
        if tile != Tile::Floor { // bump into something?
            if tile == Tile::Wall { // bump into wall?
                let afterwall = self.get_tile(x+dx, y+dy).unwrap_or(Tile::Wall);
                if afterwall == Tile::Wall { return PushResult::Blocked }
                if afterwall == Tile::Floor { // after wall floor? push.
                    self.set_tile(x, y, Tile::Floor);
                    self.set_tile(x+dx, y+dy, Tile::Wall);
                    return PushResult::Pushed;
                }
                if afterwall == Tile::Abyss { // after wall abyss? push it in
                    self.set_tile(x, y, Tile::Floor);
                    self.set_tile(x+dx, y+dy, Tile::Floor);
                    return PushResult::Tumble;
                }
            } 
            // bump into something else? dont go there
            return PushResult::Blocked;
        };
    
        // next tile is free!
        return PushResult::Free;
    }


    pub fn render(&self, ctx : &mut rltk::Rltk, offset: &Point) {
        
        // implement offset!
        let black: RGB = RGB::from_u8(0, 0, 0);
        let mut y = 0;
        let mut x = 0;

        for (tile, light) in self.tiles.iter().zip(self.light.iter()) {
            
            // Render a tile depending upon the tile type
            if *light > 0.0 { 

                let (fg, bg, glyph) = match tile {
                    Tile::Floor => (cons::RGB_BACKGROUND, cons::RGB_BACKGROUND, rltk::to_cp437(' ')),
                    Tile::Abyss => (cons::RGB_BACKGROUND, black.clone(), rltk::to_cp437(' ')),
                    Tile::Wall => {
                        let char = getwall(
                            self.get_tile(x, y-1).unwrap_or(Tile::Floor),
                            self.get_tile(x-1, y).unwrap_or(Tile::Floor),
                            self.get_tile(x, y+1).unwrap_or(Tile::Floor),
                            self.get_tile(x+1, y).unwrap_or(Tile::Floor),
                        );
                        (RGB::from_u8(100, 100, 200), cons::RGB_BACKGROUND, rltk::to_cp437(char))
                    }    
                };

                ctx.set(x + offset.x * 1, 
                    y + offset.y * 1, 
                    RGB::lerp(&black, fg, *light), 
                    RGB::lerp(&black, bg, *light), 
                    glyph);

            };

    
            // Move the coordinates
            x += 1;
            if x > self.width as i32 - 1 {
                x = 0;
                y += 1;
            }
        }
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
