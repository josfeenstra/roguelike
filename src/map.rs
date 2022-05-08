
pub struct Matrix<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: PartialEq + Clone + Copy> Matrix<T> {

    pub fn new(width: usize, height: usize, value: T) -> Self {
        let data = vec![value; width * height];
        Self {width, height, data}
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) -> usize {
        let id = self.to_index(x, y);
        self.data[id] = value;
        id
    }

    pub fn get(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None
        }
        Some(self.data[self.to_index(x, y)])
    }

    pub fn to_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    pub fn to_coord(&self, i: usize) -> (i32, i32) {
        ((i % self.width) as i32, (i / self.width) as i32)
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }
}

use rltk::RGB;

use crate::cons;


#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
    Abyss,
}


pub fn new_random(width: usize, height: usize, num_walls: u32, num_holes: u32) -> Matrix<Tile> {
    let mut map = Matrix::new(width, height, Tile::Floor);

    let w = width as i32;
    let h = height as i32;

    for i in 0..w {
        map.set(i, 0, Tile::Wall);
        map.set(i, h-1, Tile::Wall);
    } 
    for i in 0..h {
        map.set(0, i, Tile::Wall);
        map.set(w-1, i, Tile::Wall);
    } 

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..num_walls as i32 {
        let x = rng.roll_dice(1, w-2);
        let y = rng.roll_dice(1, h-2);
        map.set(x, y, Tile::Wall);
    }

    for _i in 0..num_holes as i32 {
        let x = rng.roll_dice(1, w-2);
        let y = rng.roll_dice(1, h-2);
        map.set(x, y, Tile::Abyss);
    }

    map
}


pub fn draw_world(map: &Matrix<Tile>, ctx : &mut rltk::Rltk) {

    let mut y = 0;
    let mut x = 0;
    for tile in map.data.iter() {
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
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}