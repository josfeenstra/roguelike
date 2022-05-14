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