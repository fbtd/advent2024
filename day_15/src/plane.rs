//                                        0: E    1: S    2: W     3: N
pub const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Clone, Debug)]
pub struct Plane {
    pub width: usize,
    pub height: usize,
}

impl Plane {
    pub fn new(width:usize, height:usize) -> Self {
        Plane {
            width,
            height,
        }
    }
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    pub fn tile_plus_xy(&self, origin: usize, dirs: (i32, i32)) -> Option<usize> {
        let x = dirs.0;
        let y = dirs.1;
        let new_x: i32 = (origin % self.width) as i32 + x;
        let new_y: i32 = (origin / self.width) as i32 + y;
        if new_x >= self.width as i32 || new_x < 0 || new_y >= self.height as i32 || new_y < 0 {
            return None;
        }
        Some(new_x as usize + new_y as usize * self.width)
    }

    pub fn tile_to_xy(&self, index: usize) -> (i32, i32) {
        let x = index as i32 % self.width as i32;
        let y = index as i32 / self.width as i32;
        (x, y)
    }

    pub fn delta_xy(&self, indexes: (usize, usize)) -> (i32, i32) {
        let first = self.tile_to_xy(indexes.0);
        let second = self.tile_to_xy(indexes.1);
        (second.0 - first.0, second.1 - first.1)
    }
}
