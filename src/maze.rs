use rltk::{RGB, Rltk};

#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor,
    Abyss,
}


pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Tile>,
}

impl Map {

    pub fn new(width: usize, height: usize) -> Self {

        let data = vec![Tile::Floor; width * height];
        return Self {width, height, data};
    }

    pub fn new_random(width: usize, height: usize) -> Self {
        let mut map = Map::new(width, height);

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

        for _i in 0..100 as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            map.set(x, y, Tile::Wall);
        }

        for _i in 0..100 as i32 {
            let x = rng.roll_dice(1, w-2);
            let y = rng.roll_dice(1, h-2);
            map.set(x, y, Tile::Abyss);
        }

        return map;
    }

    pub fn set(&mut self, x: i32, y: i32, value: Tile) -> usize {
        let id = self.to_index(x, y);
        self.data[id] = value;
        id
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        self.data[self.to_index(x, y)]
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